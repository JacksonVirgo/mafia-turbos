pub mod chatroom;
pub mod homepage;
pub mod login;
pub mod websocket;
pub mod whoami;

use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new()
        .route("/", get(homepage::homepage_root))
        .route("/whoami", get(whoami::who_am_i_root))
        .merge(websocket::router())
        .merge(chatroom::router())
        .merge(login::router())
}
