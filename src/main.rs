use crate::websocket::{ws_handler, DeviceSubscribers};
use axum::{
    routing::{get, post},
    serve, Router,
};
use getter::{get_device_list, get_device_logs};
use receiver::event_handler;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

mod db;
mod getter;
mod receiver;
mod websocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Server running at http://127.0.0.1:8080");

    let listener = tokio::net::TcpListener::bind(addr).await?;

    // 创建 WebSocket 订阅者存储
    let subscribers: DeviceSubscribers = Arc::new(RwLock::new(HashMap::new()));

    let app = Router::new()
        .route("/receiver", post(event_handler))
        .route("/device_id_list", get(get_device_list))
        .route("/log/{device_id}", get(get_device_logs))
        .route("/ws/{device_id}", get(ws_handler))
        .layer(cors)
        .with_state(subscribers);

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
