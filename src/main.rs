use std::error::Error;
use sqlx::Connection;
use sqlx::PgConnection;
use sqlx::Row;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>
{
    let url: String = getConnString();
    let mut conn = sqlx::postgres::PgConnection::connect(&url).await?;

    let res = sqlx::query("SELECT 1 +1 as sum")
        .fetch_one(&mut conn)
        .await?;

    let sum: i32 = res.get("sum");
    println!("1 + 1 = {}", sum);

    Ok(())
}

fn getConnString() -> String
{
    let key = "GAUGE_16_3_CONN";
    let val = env::var(key);
    match val{
        Ok(val) => return val,
        Err(e) => {
            println!("Couldn't find connection string from key {}: {}", key, e.to_string());
            return "".to_string()
        }
    }
}