use dotenv::dotenv;
use serde_json::Value;
use std::env;
use tokio_postgres::NoTls;

pub async fn edit_data(table: &str, data: &Value, id: i64) -> Result<String, String> {
    println!("Editing data...");

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");

    let (client, connection) = match tokio_postgres::connect(&database_url, NoTls).await {
        Ok(conn) => conn,
        Err(e) => return Err(format!("Error connecting to database: {}", e)),
    };

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let data_object = match data.as_object() {
        Some(obj) => obj,
        None => return Err("Invalid JSON data, expected a JSON object.".to_string()),
    };

    let columns: Vec<String> = data_object.keys().cloned().collect();

    // Prepare the values for the update query
    let mut values: Vec<Box<dyn tokio_postgres::types::ToSql + Sync>> = Vec::new();

    for v in data_object.values() {
        match v {
            Value::String(s) => {
                values.push(Box::new(s.clone()) as Box<dyn tokio_postgres::types::ToSql + Sync>)
            }
            Value::Number(n) if n.is_i64() => {
                values
                    .push(Box::new(n.as_i64().unwrap())
                        as Box<dyn tokio_postgres::types::ToSql + Sync>)
            }
            Value::Number(n) if n.is_f64() => {
                values
                    .push(Box::new(n.as_f64().unwrap())
                        as Box<dyn tokio_postgres::types::ToSql + Sync>)
            }
            Value::Bool(b) => {
                values.push(Box::new(*b) as Box<dyn tokio_postgres::types::ToSql + Sync>)
            }
            _ => return Err(format!("Unsupported data type in JSON object: {:?}", v)),
        }
    }

    // Create the SET part of the SQL query for updating the data
    let set_clause = columns
        .iter()
        .enumerate()
        .map(|(i, col)| format!("{} = ${}", col, i + 1))
        .collect::<Vec<String>>()
        .join(", ");

    // Prepare the SQL query with a WHERE clause based on the provided `id`
    let query = format!(
        "UPDATE shop.{} SET {} WHERE id = ${};", // Add 'shop.' to the table name
        table,
        set_clause,
        columns.len() + 1 // id will be the last parameter
    );

    // Add the `id` to the list of values for the WHERE clause
    values.push(Box::new(id) as Box<dyn tokio_postgres::types::ToSql + Sync>);

    // Convert Vec<Box<dyn ToSql>> into a slice of references to `dyn ToSql`
    let values_ref: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> =
        values.iter().map(|v| &**v).collect();

    // Execute the update query
    match client.execute(&query, &values_ref).await {
        Ok(rows_updated) => {
            if rows_updated > 0 {
                Ok("Data updated successfully.".to_string())
            } else {
                Err("No rows were updated.".to_string())
            }
        }
        Err(e) => Err(format!("Error executing query: {}", e)),
    }
}
