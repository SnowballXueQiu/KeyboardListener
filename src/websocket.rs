use axum::extract::ws::{Message, WebSocket};
use axum::extract::{Path, WebSocketUpgrade};
use axum::response::Response;
use serde_json::json;
use flurry::HashMap;
use tokio::sync::broadcast::{self, Sender};

pub type DeviceSubscribers = HashMap<String, Sender<String>>;

pub async fn handle_ws_connection(
    mut socket: WebSocket,
    device_id: String,
    subscribers: DeviceSubscribers,
) {
    let tx = {
        let guard=subscribers.guard();
        subscribers
            .get(&device_id.clone(),&guard)
            .or_else(||{
                let t=broadcast::channel(100).0;
                subscribers.insert(device_id.clone(),t,&guard);
                subscribers.get(&device_id.clone(),&guard)
            })
            .unwrap()
            .clone()
    };

    let mut rx = tx.subscribe();

    // 只发送消息
    while let Ok(msg) = rx.recv().await {
        if socket.send(Message::Text(msg.into())).await.is_err() {
            break;
        }
    }

    subscribers.remove(&device_id,&subscribers.guard());
}

pub async fn broadcast_log(
    device_id: &str,
    time: i64,
    event_type: &str,
    content: &str,
    timezone: &str,
    subscribers: &DeviceSubscribers,
) {
    if let Some(tx) = subscribers.get(device_id,&subscribers.guard()) {
        let message = json!({
            "time": time,
            "event_type": event_type,
            "content": content,
            "timezone": timezone
        })
        .to_string();

        let _ = tx.send(message);
    }
}

pub async fn ws_handler(
    Path(device_id): Path<String>,
    ws: WebSocketUpgrade,
    subscribers: axum::extract::State<DeviceSubscribers>,
) -> Response {
    ws.on_upgrade(move |socket| handle_ws_connection(socket, device_id, subscribers.0))
}
