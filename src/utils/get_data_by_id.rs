use anyhow::{anyhow, Result};
use dotenv::dotenv;
use serde_json::json;
use std::collections::HashMap;
use std::env;
use tokio_postgres::NoTls;

pub async fn get_data_by_id(table: &str, id: i32) -> Result<serde_json::Value> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let query = format!("SELECT * FROM shop.{} WHERE id = $1", table);
    let stmt = client.prepare(&query).await?;
    let rows = client.query(&stmt, &[&id]).await?;

    if rows.is_empty() {
        return Err(anyhow!("No data found"));
    }

    let column_names: Vec<String> = rows[0]
        .columns()
        .iter()
        .map(|col| col.name().to_string())
        .collect();

    let row = &rows[0];
    let mut row_data = HashMap::new();

    for (i, column_name) in column_names.iter().enumerate() {
        // Handle different types
        let value = match row.try_get::<_, Option<i32>>(i) {
            Ok(Some(v)) => Some(v.to_string()), // Convert integer to string
            _ => match row.try_get::<_, Option<String>>(i) {
                Ok(Some(v)) => Some(v), // Handle string or null
                _ => None,              // Handle case where value is None
            },
        };

        row_data.insert(
            column_name.clone(),
            value.unwrap_or_else(|| "Unknown".to_string()),
        );
    }

    let json_result = json!(row_data);
    Ok(json_result)
}
