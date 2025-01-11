use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

#[derive(Debug, Serialize, Deserialize)]
struct WsMessage {
    #[serde(default)]
    type_: String,
}

// 用于存储所有设备的WebSocket连接
lazy_static::lazy_static! {
    static ref DEVICE_CONNECTIONS: Arc<Mutex<HashMap<String, Vec<broadcast::Sender<String>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub struct WebSocketConnection {
    device_id: String,
    tx: Option<broadcast::Sender<String>>,
}

impl Actor for WebSocketConnection {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                // 处理客户端发送的消息
                if let Ok(message) = serde_json::from_str::<WsMessage>(&text) {
                    if message.type_ == "close" {
                        // 收到关闭消息，立即关闭连接
                        ctx.close(None);
                        ctx.stop();
                        return;
                    }
                }
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        // 连接结束时清理资源
        if let Ok(mut connections) = DEVICE_CONNECTIONS.lock() {
            if let Some(senders) = connections.get_mut(&self.device_id) {
                if let Some(tx) = self.tx.take() {
                    senders.retain(|x| !x.same_channel(&tx));
                    if senders.is_empty() {
                        connections.remove(&self.device_id);
                    }
                }
            }
        }
    }
}

impl WebSocketConnection {
    fn new(device_id: String) -> Self {
        Self {
            device_id,
            tx: None,
        }
    }

    // 广播消息到指定设备的所有连接
    pub fn broadcast_to_device(device_id: &str, message: String) {
        if let Ok(connections) = DEVICE_CONNECTIONS.lock() {
            if let Some(senders) = connections.get(device_id) {
                for sender in senders {
                    let _ = sender.send(message.clone());
                }
            }
        }
    }
}

impl actix::Handler<String> for WebSocketConnection {
    type Result = ();

    fn handle(&mut self, msg: String, ctx: &mut Self::Context) {
        ctx.text(msg);
    }
}

pub async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    device_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let device_id = device_id.into_inner();
    let (tx, _rx) = broadcast::channel(100);

    let mut connection = WebSocketConnection::new(device_id.clone());
    connection.tx = Some(tx.clone());

    // 将新连接添加到设备连接列表中
    if let Ok(mut connections) = DEVICE_CONNECTIONS.lock() {
        connections
            .entry(device_id)
            .or_insert_with(Vec::new)
            .push(tx);
    }

    let resp = ws::start(connection, &req, stream)?;
    Ok(resp)
}
