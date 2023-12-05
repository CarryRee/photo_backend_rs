use redis::{Commands, RedisError, Client};
use std::env;

pub async fn get_client() -> Result<Client, RedisError>  {
    let url = env::var("REDIS_URL").expect("get ROCKET_REDIS fail");
    let client = Client::open(url);
    client
}
