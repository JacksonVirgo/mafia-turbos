use axum::{
    Router,
    routing::{get, post},
};

pub mod login_form;

pub fn router() -> Router {
    Router::new()
        .route("/login_form", get(login_form::login_form))
        .route("/signup", post(login_form::login_form_submission))
}
