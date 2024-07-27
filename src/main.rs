use std::error::Error;
use sqlx::Connection;
use sqlx::PgConnection;
use sqlx::Row;
use std::env;
mod secrets;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>
{
    let url: String = secrets::getConnString();
                
    let mut conn = sqlx::postgres::PgConnection::connect(&url).await?;

    let res = sqlx::query("SELECT 1 +1 as sum")
        .fetch_one(&mut conn)
        .await?;

    let sum: i32 = res.get("sum");
    println!("1 + 1 = {}", sum);

    Ok(())
}