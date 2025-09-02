pub mod homepage;

use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(homepage::homepage_root))
        .route("/user_info", get(homepage::get_user))
        .route("/create_user", post(homepage::create_user))
}
