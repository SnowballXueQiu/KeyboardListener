use axum::{
    routing::{get, post},
    serve, Router,
};
use getter::{get_device_list, get_device_logs};
use receiver::event_handler;
use std::net::SocketAddr;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};

mod db;
mod getter;
mod receiver;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/receiver", post(event_handler))
        .route("/device_id_list", get(get_device_list))
        .route("/log/{device_id}", get(get_device_logs))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Server running at http://127.0.0.1:8080");

    let listener = tokio::net::TcpListener::bind(addr).await?;

    tokio::select! {
        result = serve(listener, app.into_make_service()) => {
            if let Err(e) = result {
                println!("Server error: {}", e);
            }
        },
        _ = signal::ctrl_c() => {
            println!("Server shutting down...");
        },
    }

    Ok(())
}
