use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::app::server::state::ServerState;

pub mod login_form;

pub fn router() -> Router<Arc<ServerState>> {
    Router::new()
        .route("/login_form", get(login_form::login_form))
        .route("/signup", post(login_form::login_form_submission))
}
