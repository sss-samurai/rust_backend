use dotenv::dotenv;
use serde_json::Value;
use std::env;
use tokio_postgres::NoTls;

pub async fn create_data(table: &str, data: Value) -> Result<String, String> {
    println!("Creating data...");

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

    // Ensure data is a valid JSON object
    let data_object = match data.as_object() {
        Some(obj) => obj,
        None => return Err("Invalid JSON data, expected a JSON object.".to_string()),
    };

    // Extract columns (keys) from the JSON object
    let columns: Vec<String> = data_object.keys().cloned().collect();

    // Prepare the values to be inserted into the table
    let values: Result<Vec<Box<dyn tokio_postgres::types::ToSql + Sync>>, String> = data_object
        .values()
        .map(|v| match v {
            Value::String(s) => {
                Ok(Box::new(s.clone()) as Box<dyn tokio_postgres::types::ToSql + Sync>)
            }
            Value::Number(n) if n.is_i64() => {
                Ok(Box::new(n.as_i64().unwrap()) as Box<dyn tokio_postgres::types::ToSql + Sync>)
            }
            Value::Number(n) if n.is_f64() => {
                Ok(Box::new(n.as_f64().unwrap()) as Box<dyn tokio_postgres::types::ToSql + Sync>)
            }
            Value::Bool(b) => Ok(Box::new(*b) as Box<dyn tokio_postgres::types::ToSql + Sync>),
            _ => Err(format!("Unsupported data type in JSON object: {:?}", v)),
        })
        .collect();

    let values = values?;

    // Create the columns and placeholders for the SQL query
    let columns_str = columns.join(", ");
    let placeholders = (1..=columns.len())
        .map(|i| format!("${}", i))
        .collect::<Vec<String>>()
        .join(", ");

    // Update the query to include the schema name (shop)
    let query = format!(
        "INSERT INTO shop.{} ({}) VALUES ({})", // Add 'shop.' to the table name
        table, columns_str, placeholders
    );

    // Convert the values to a vector of references
    let values_ref: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> =
        values.iter().map(|v| &**v).collect();

    // Execute the query to insert the data into the database
    match client.execute(&query, &values_ref).await {
        Ok(_) => Ok("Data inserted successfully.".to_string()),
        Err(e) => Err(format!("Error executing query: {}", e)),
    }
}
