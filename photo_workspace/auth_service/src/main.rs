use tracing::Level;
use std::net::SocketAddr; 
use std::env;
use std::sync::Arc;

use axum::{
    routing::{get, post, delete},
    Router,
};
use common_lib::utils::{app_state::AppState, redis};

mod api;
mod database;

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
    let port = env::var("AUTH_SERVER_PORT").unwrap();

    // database pool
    let sqlx_pool = database::config::get_pool().await.unwrap();

    let redis_clent = redis::get_client().await.unwrap();

    let arc = Arc::new (AppState {db: sqlx_pool.clone(), redis: redis_clent.clone() } );

    // build our application with a route
    let app = Router::new()
        .route("/sign-in", post(api::auth::sign_in))
        .route("/sign-up", post(api::auth::sign_up))
        .with_state(arc);
    
    let address: SocketAddr = SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>().unwrap()));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    // run it with hyper on localhost:port
    axum::serve(listener, app).await.unwrap();

}