pub mod homepage;
pub mod login;

use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new()
        .route("/", get(homepage::homepage_root))
        .merge(login::router())
}
