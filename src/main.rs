use axum::{routing::post, Router, serve};
use std::net::SocketAddr;
use tokio::signal;
use receiver::event_handler;

mod receiver;
mod db;
mod getter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/receive_event", post(event_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Server running at http://127.0.0.1:8080");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    let server = serve(listener, app.into_make_service());

    tokio::select! {
        _ = server => {},
        _ = signal::ctrl_c() => {
            println!("Server shutting down...");
        },
    }

    Ok(())
}
