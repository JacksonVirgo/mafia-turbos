use axum::extract::ws::{Message, WebSocket};
use maud::html;

use crate::routes::websocket::data::Inbound;

pub async fn handle_chatbox(ws: &mut WebSocket, payload: &Inbound) -> anyhow::Result<()> {
    let Some(message) = payload.rest.get("chat_message") else {
        return Ok(());
    };

    let Some(msg) = message.as_str() else {
        return Ok(());
    };

    let markdown = html! {
        div."text-red-400" id="chatroom" {
            (format!("Whoo! You sent: {}", msg))
        }
    };
    let html = markdown.into_string();
    ws.send(Message::Text(html.into())).await?;
    Ok(())
}
