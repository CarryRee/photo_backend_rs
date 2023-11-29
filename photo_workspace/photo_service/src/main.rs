use tracing::Level;
use std::net::SocketAddr; 
use std::env;

use axum::{
    routing::{get, post, delete},
    extract::DefaultBodyLimit,
    Router,
    middleware,
};
use tower_http::limit::RequestBodyLimitLayer; 

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
    let pool = database::db_config::get_pool().await.unwrap();

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
        //.layer(auth::AuthLayer {state: pool.clone()})
        .route_layer(middleware::from_fn_with_state(pool.clone(), 
        |state, req, next: middleware::Next| auth_core::middleware::auth::auth(state, req, next)),)
        .with_state(pool);

    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], port.parse::<u16>().unwrap()));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    // run it with hyper on localhost:port
    axum::serve(listener, app).await.unwrap();

}