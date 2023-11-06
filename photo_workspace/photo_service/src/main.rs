use tracing::Level;
use std::net::SocketAddr; 
use std::env;

use axum::{
    routing::{get, post},
    Router,
};

mod database;
mod endpoint;

#[tokio::main]
async fn main() {
    run_service().await;
}

async fn run_service() {

    // tracing
    tracing_subscriber::fmt()
    // filter spans/events with level TRACE or higher.
    .with_max_level(Level::TRACE)
    // build but do not install the subscriber.
    .init();

    // env
    dotenvy::dotenv().expect(".env file not found");
    let port = env::var("PHOTO_SEVER_PORT").unwrap();

    // database pool
    let pool = database::db_config::get_pool().await.unwrap();

    // build our application with a route
    let app = Router::new()
        .route("/", get(endpoint::api_root::root))
        .with_state(pool);

    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], port.parse::<u16>().unwrap()));

    // run it with hyper on localhost:port
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap()

}