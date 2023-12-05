use redis::Client;
use sqlx::{Pool, MySql};

pub struct AppState {
    pub db: Pool<MySql>,
    pub redis: Client,
}