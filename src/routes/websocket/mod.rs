use std::collections::HashMap;

use axum::{
    Router,
    body::Body,
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    http::Response,
    routing::any,
};

pub fn router() -> Router {
    Router::new().route("/ws", any(handler))
}

async fn handler(ws: WebSocketUpgrade) -> Response<Body> {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(raw_msg) = socket.recv().await {
        let msg = match raw_msg {
            Ok(msg) => msg,
            _ => {
                return;
            }
        };

        if let Message::Text(text) = msg {
            let parsed: Result<HashMap<String, serde_json::Value>, _> = serde_json::from_str(&text);
            match parsed {
                Ok(map) => {
                    if let Some(val) = map.get("action") {
                        println!("action = {:?}", val);
                    }
                    println!("whole map = {:?}", map);
                }
                Err(err) => {
                    eprintln!("failed to parse JSON: {err}");
                }
            }
            if socket.send(Message::Text(text)).await.is_err() {
                return;
            }
        } else {
            if socket.send(msg).await.is_err() {
                return;
            }
        };
    }
}
