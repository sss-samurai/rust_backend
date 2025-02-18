use dotenv::dotenv;
use std::env;
use tokio_postgres::NoTls;

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
