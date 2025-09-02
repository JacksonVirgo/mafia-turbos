use axum::{
    Json, Router,
    http::StatusCode,
    response::Html,
    routing::{get, post},
};
use mafia_turbos::builders::webpage::WebPageBuilder;
use maud::html;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<String> {
    let markdown = WebPageBuilder::new()
        .title("Mafia Turbos")
        .body(html! {
            h1."text-2xl bold underline"{ "Mafia Turbos "}
            h2 { "Subheader"}
        })
        .build();

    let html = markdown.into_string();
    Html(html)
}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
