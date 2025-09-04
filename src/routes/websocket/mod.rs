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
use serde_json::Value;

use crate::routes::websocket::{
    chatbox::handle_chatbox,
    data::{Inbound, sole_key},
};

pub mod chatbox;
pub mod data;

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
            let Ok(input) = serde_json::from_str::<Inbound>(&text) else {
                eprintln!("Bad in-bound JSON");
                continue;
            };

            let route = input
                .headers
                .get("HX-Target")
                .and_then(Value::as_str)
                .map(str::to_owned)
                .or_else(|| {
                    input
                        .rest
                        .get("event")
                        .and_then(Value::as_str)
                        .map(str::to_owned)
                })
                .or_else(|| sole_key(&input.rest))
                .unwrap_or_else(|| "unknown".into());

            if let Err(err) = dispatch(&route, &mut socket, &input).await {
                eprintln!("dispatch({route}) error: {err}");
            }
        } else {
            if socket.send(msg).await.is_err() {
                return;
            }
        };
    }
}

async fn dispatch(route: &str, ws: &mut WebSocket, payload: &Inbound) -> anyhow::Result<()> {
    match route {
        "chatbox" => handle_chatbox(ws, payload).await,
        _ => {
            println!("Unknown Websocket Route: {}", route);
            Ok(())
        }
    }
}
