use axum::extract::ws::{Message, WebSocket};
use axum::extract::{Path, WebSocketUpgrade};
use axum::response::Response;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast::{self, Sender};
use tokio::sync::RwLock;

pub type DeviceSubscribers = Arc<RwLock<HashMap<String, Sender<String>>>>;

pub async fn handle_ws_connection(
    mut socket: WebSocket,
    device_id: String,
    subscribers: DeviceSubscribers,
) {
    // 获取或创建设备的广播通道
    let tx = {
        let mut subscribers = subscribers.write().await;
        subscribers
            .entry(device_id.clone())
            .or_insert_with(|| broadcast::channel(100).0)
            .clone()
    };

    let mut rx = tx.subscribe();

    // 只发送消息
    while let Ok(msg) = rx.recv().await {
        if socket.send(Message::Text(msg.into())).await.is_err() {
            break;
        }
    }

    // 连接关闭时自动清理
    subscribers.write().await.remove(&device_id);
}

// 广播新的日志消息给所有订阅者
pub async fn broadcast_log(
    device_id: &str,
    time: i64,
    event_type: &str,
    content: &str,
    timezone: &str,
    subscribers: &DeviceSubscribers,
) {
    if let Some(tx) = subscribers.read().await.get(device_id) {
        let message = json!({
            "time": time,
            "event_type": event_type,
            "content": content,
            "timezone": timezone
        })
        .to_string();

        // 忽略发送错误，因为可能没有活跃的订阅者
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
