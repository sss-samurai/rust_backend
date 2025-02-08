use dotenv::dotenv;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use tokio_postgres::{Error, NoTls};

async fn get_column_names(
    client: &tokio_postgres::Client,
    table: &str,
) -> Result<Vec<String>, Error> {
    let query = format!(
        "SELECT column_name FROM information_schema.columns WHERE table_name = '{}' AND table_schema = 'shop';",
        table
    );
    let columns = client.query(query.as_str(), &[]).await?;
    let column_names: Vec<String> = columns.iter().map(|row| row.get::<_, String>(0)).collect();
    println!("Columns: {:?}", column_names); // Debugging: print columns
    Ok(column_names)
}

fn map_value_to_json(row: &tokio_postgres::Row, i: usize) -> Value {
    match row.try_get::<_, Option<String>>(i) {
        Ok(Some(val)) => json!(val),
        Ok(None) => json!("NULL"),
        Err(_) => match row.try_get::<_, Option<i32>>(i) {
            Ok(Some(val)) => json!(val),
            Ok(None) => json!("NULL"),
            Err(_) => match row.try_get::<_, Option<i64>>(i) {
                Ok(Some(val)) => json!(val),
                Ok(None) => json!("NULL"),
                Err(_) => match row.try_get::<_, Option<f64>>(i) {
                    Ok(Some(val)) => json!(val),
                    Ok(None) => json!("NULL"),
                    Err(_) => match row.try_get::<_, Option<bool>>(i) {
                        Ok(Some(val)) => json!(val),
                        Ok(None) => json!("NULL"),
                        Err(_) => json!("Unknown"),
                    },
                },
            },
        },
    }
}

pub async fn get_all_data(table: &str) -> Result<Value, Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("Failed to connect to the database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let column_names = get_column_names(&client, table).await?;
    let query = format!("SELECT * FROM shop.{}", table);
    println!("Executing query: {}", query); // Debugging: print the query
    let rows = client.query(query.as_str(), &[]).await?;

    println!("Fetched {} rows", rows.len()); // Debugging: print the number of rows fetched
    let mut result = Vec::new();
    for row in rows {
        let mut row_data = HashMap::new();
        for (i, column_name) in column_names.iter().enumerate() {
            let value = map_value_to_json(&row, i);
            println!("Column: {}, Value: {:?}", column_name, value); // Debugging: print each value
            row_data.insert(column_name.clone(), value);
        }
        result.push(row_data);
    }

    Ok(json!(result))
}
