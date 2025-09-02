pub mod homepage;
pub mod login;
pub mod whoami;

use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new()
        .route("/", get(homepage::homepage_root))
        .route("/whoami", get(whoami::who_am_i_root))
        .merge(login::router())
}
