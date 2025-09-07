use std::sync::Arc;

use axum::{Router, extract::Query, response::Html, routing::get};
use maud::html;
use serde::Deserialize;

use crate::{app::server::state::ServerState, builders::webpage::WebPageBuilder};

pub mod data;

#[derive(Deserialize)]
pub struct GuestLoginQuery {
    username: String,
    code: String,
}

pub async fn chatroom(Query(q): Query<GuestLoginQuery>) -> Html<String> {
    let ws_url = format!("/ws?username={}&channel={}", q.username, q.code);

    WebPageBuilder::new()
        .title("Mafia Turbos")
        .body(html! {
            h1."text-2xl bold underline"{ "Mafia Turbos "}
            h2 { "Chat Room"}
            hr {}

            div hx-ext="ws" ws-connect=(ws_url) {
                ul id="chat-messages" {
                    li {"Existing Item"}
                }

                form id="chatbox" ws-send hx-on::htmx:wsAfterSend="this.reset();" {
                    input name="chat_message" autocomplete="off" required {}
                }
            }

        })
        .build_as_html()
}

pub fn router() -> Router<Arc<ServerState>> {
    Router::new().route("/chatroom", get(chatroom))
}
