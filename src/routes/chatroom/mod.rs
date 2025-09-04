use axum::{Router, response::Html, routing::get};
use maud::html;

use crate::builders::webpage::WebPageBuilder;

pub mod data;

pub async fn chatroom() -> Html<String> {
    WebPageBuilder::new()
        .title("Mafia Turbos")
        .body(html! {
            h1."text-2xl bold underline"{ "Mafia Turbos "}
            h2 { "Chat Room"}
            hr {}

            div hx-ext="ws" ws-connect="/ws" {
                div id="chatroom" hx-swap-oob="beforeend" {}
                form id="chatbox" ws-send {
                    input name="chat_message" {}
                }
            }

        })
        .build_as_html()
}

pub fn router() -> Router {
    Router::new().route("/chatroom", get(chatroom))
}
