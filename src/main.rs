use crate::config::Config;
use crate::websocket::{ws_handler, DeviceSubscribers};
use axum::{
    routing::{get, post},
    serve, Router,
};
use getter::{get_device_list, get_device_logs};
use receiver::event_handler;
use flurry::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};

mod config;
mod db;
mod getter;
mod receiver;
mod websocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config =
        Config::init_config_file("config.toml").unwrap_or_else(|err| {
            println!(
                "Failed to initialize or read config.toml: {}. Using default configuration.",
                err
            );
            Config::default()
        });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let addr = SocketAddr::from((
        config.runtime.url.parse::<std::net::IpAddr>()?,
        config.runtime.port,
    ));

    println!(
        "Server running at http://{}:{}",
        config.runtime.url, config.runtime.port
    );

    let listener = tokio::net::TcpListener::bind(addr).await?;

    let subscribers: DeviceSubscribers = Arc::new(HashMap::new());

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
