use anyhow::{anyhow, Result};
use dotenv::dotenv;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use tokio_postgres::NoTls;
pub struct RecordHandler;

impl RecordHandler {
    pub async fn create_data(table: &str, data: Value) -> Result<String, String> {
        println!("Creating data...");

        dotenv().ok();

        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");

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

        let values: Result<Vec<Box<dyn tokio_postgres::types::ToSql + Sync>>, String> =
            data_object
                .values()
                .map(|v| match v {
                    Value::String(s) => {
                        Ok(Box::new(s.clone()) as Box<dyn tokio_postgres::types::ToSql + Sync>)
                    }
                    Value::Number(n) if n.is_i64() => Ok(Box::new(n.as_i64().unwrap())
                        as Box<dyn tokio_postgres::types::ToSql + Sync>),
                    Value::Number(n) if n.is_f64() => Ok(Box::new(n.as_f64().unwrap())
                        as Box<dyn tokio_postgres::types::ToSql + Sync>),
                    Value::Bool(b) => {
                        Ok(Box::new(*b) as Box<dyn tokio_postgres::types::ToSql + Sync>)
                    }
                    _ => Err(format!("Unsupported data type in JSON object: {:?}", v)),
                })
                .collect();

        let values = values?;

        let columns_str = columns.join(", ");
        let placeholders = (1..=columns.len())
            .map(|i| format!("${}", i))
            .collect::<Vec<String>>()
            .join(", ");

        let query = format!(
            "INSERT INTO storefront.{} ({}) VALUES ({}) RETURNING *",
            table, columns_str, placeholders
        );

        let values_ref: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> =
            values.iter().map(|v| &**v).collect();

        match client.query_one(&query, &values_ref).await {
            Ok(row) => {
                println!("Inserted row: {:?}", row);
                Ok("Data inserted successfully.".to_string())
            }
            Err(e) => {
                println!("Query: {}", query);
                println!("Values: {:?}", values_ref);
                Err(format!("Error executing query: {}", e))
            }
        }
    }

    pub async fn edit_data(table: &str, data: &Value, id: i64) -> Result<String, String> {
        println!("Editing data...");

        dotenv().ok();

        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");

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

        let mut values: Vec<Box<dyn tokio_postgres::types::ToSql + Sync>> = Vec::new();

        for v in data_object.values() {
            match v {
                Value::String(s) => {
                    values.push(Box::new(s.clone()) as Box<dyn tokio_postgres::types::ToSql + Sync>)
                }
                Value::Number(n) if n.is_i64() => values
                    .push(Box::new(n.as_i64().unwrap())
                        as Box<dyn tokio_postgres::types::ToSql + Sync>),
                Value::Number(n) if n.is_f64() => values
                    .push(Box::new(n.as_f64().unwrap())
                        as Box<dyn tokio_postgres::types::ToSql + Sync>),
                Value::Bool(b) => {
                    values.push(Box::new(*b) as Box<dyn tokio_postgres::types::ToSql + Sync>)
                }
                _ => return Err(format!("Unsupported data type in JSON object: {:?}", v)),
            }
        }

        let set_clause = columns
            .iter()
            .enumerate()
            .map(|(i, col)| format!("{} = ${}", col, i + 1))
            .collect::<Vec<String>>()
            .join(", ");

        let query = format!(
            "UPDATE shop.{} SET {} WHERE id = ${};",
            table,
            set_clause,
            columns.len() + 1
        );

        values.push(Box::new(id) as Box<dyn tokio_postgres::types::ToSql + Sync>);

        let values_ref: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> =
            values.iter().map(|v| &**v).collect();

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
            let value = match row.try_get::<_, Option<i32>>(i) {
                Ok(Some(v)) => Some(v.to_string()), // Convert integer to string
                _ => match row.try_get::<_, Option<String>>(i) {
                    Ok(Some(v)) => Some(v),
                    _ => None,
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

    pub async fn delete_data(table: &str, id: i64) -> Result<String, String> {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let (client, connection) = tokio_postgres::connect(&db_url, NoTls)
            .await
            .map_err(|e| format!("Error connecting to database: {}", e))?;
        tokio::spawn(connection);
        let query = format!("DELETE FROM shop.{} WHERE id = $1", table);
        let result = client.execute(query.as_str(), &[&id]).await;
        match result {
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    Ok(format!(
                        "Successfully deleted {} row(s) from shop.{}",
                        rows_affected, table
                    ))
                } else {
                    Err(format!(
                        "No rows were deleted. Could not find a record with id {}",
                        id
                    ))
                }
            }
            Err(e) => Err(format!("Error executing delete query: {}", e)),
        }
    }
}
