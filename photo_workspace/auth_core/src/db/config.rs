use std::env;
use sqlx::pool::Pool;
use sqlx::error::Error;
use sqlx::mysql::MySqlPoolOptions;

pub async fn get_pool() -> Result<Pool<sqlx::MySql>, Error> {

    let database_url = env::var("DATABASE_URL").unwrap();

    let pool = MySqlPoolOptions::new()
            .max_connections(20)
            .connect(&database_url)
            .await;        

    pool
}
