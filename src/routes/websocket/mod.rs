use std::sync::Arc;

use crate::app::server::state::{RoomState, ServerState};
use axum::{
    Router,
    extract::{
        Query, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
    routing::get,
};
use futures::{sink::SinkExt, stream::StreamExt};
use maud::html;
use serde::Deserialize;
use tokio::sync::broadcast;

pub mod chatbox;
pub mod data;

#[derive(Deserialize, Clone)]
struct Connect {
    username: String,
    channel: String,
}

#[derive(Deserialize)]
struct HtmxWsMessage {
    #[serde(rename = "HEADERS")]
    headers: Option<serde_json::Value>,

    #[serde(flatten)]
    rest: std::collections::HashMap<String, serde_json::Value>,
}

pub fn router() -> Router<Arc<ServerState>> {
    Router::new().route("/ws", get(websocket_handler))
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<ServerState>>,
    Query(connect): Query<Connect>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| websocket(socket, state, connect))
}

async fn websocket(stream: WebSocket, state: Arc<ServerState>, connect: Connect) {
    let (mut sender, mut receiver) = stream.split();

    let (tx, username, channel) = {
        let mut rooms = state.rooms.lock().unwrap();
        let room = rooms
            .entry(connect.channel.clone())
            .or_insert_with(RoomState::new);

        if !room.user_set.insert(connect.username.clone()) {
            let _ = sender.send(Message::Text("Username taken".into()));
            return;
        }

        (
            room.tx.clone(),
            connect.username.clone(),
            connect.channel.clone(),
        )
    };

    let mut rx = tx.subscribe();

    let _ = tx.send(
        html! {
            div hx-swap-oob="beforeend:#chat-messages" {
                li { (format!("{} joined.", username)) }
            }
        }
        .into_string(),
    );

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    let name_for_recv = username.clone();
    let tx_for_recv = tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            let parsed: HtmxWsMessage = match serde_json::from_str(&text) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let msg = parsed
                .rest
                .get("chat_message")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();

            if msg.is_empty() {
                continue;
            }

            let html_snippet = html! {
                div hx-swap-oob="beforeend:#chat-messages" {
                    li { (format!("{}: {}", name_for_recv, msg)) }
                }
            }
            .into_string();

            let _ = tx_for_recv.send(html_snippet);
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    let _ = tx.send(
        html! {
            div hx-swap-oob="beforeend:#chat-messages" {
                li { (format!("{} left.", username)) }
            }
        }
        .into_string(),
    );

    let mut rooms = state.rooms.lock().unwrap();
    if let Some(room) = rooms.get_mut(&channel) {
        room.user_set.remove(&username);
        if room.user_set.is_empty() {
            rooms.remove(&channel);
        }
    }
}
