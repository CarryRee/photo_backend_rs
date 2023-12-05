use tracing::Level;
use std::net::SocketAddr; 
use std::env;
use std::sync::Arc;

use axum::{
    routing::{get, post, delete},
    extract::DefaultBodyLimit,
    Router,
    middleware,
};
use tower_http::limit::RequestBodyLimitLayer; 

use common_lib::utils::redis;
use common_lib::utils::app_state::AppState;

mod database;
mod endpoint;
mod model;


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
    let port = env::var("PHOTO_SERVER_PORT").unwrap();

    // database pool
    let sqlx_pool = database::db_config::get_pool().await.unwrap();

    let redis_pool = redis::get_client().await.unwrap();

    let arc = Arc::new (AppState {db: sqlx_pool.clone(), redis: redis_pool.clone() } );

    // build our application with a route
    let app = Router::new()
        .route("/", get(endpoint::api_root::root))
        .route("/photos", get(endpoint::api_photo::get_photos))
        .route("/photos", delete(endpoint::api_photo::delete_photos))
        .route("/upload_photos", post(endpoint::api_photo::upload_photo))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            100 * 1024 * 1024 // 100 mb
        ))
        .route_layer(middleware::from_fn_with_state(arc.clone(), 
        |state, req, next: middleware::Next| auth_core::middleware::auth::auth(state, req, next)),)
        .with_state(arc);


    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], port.parse::<u16>().unwrap()));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    // run it with hyper on localhost:port
    axum::serve(listener, app).await.unwrap();

}