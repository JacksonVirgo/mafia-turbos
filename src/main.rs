use axum::Router;
use mafia_turbos::routes;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(routes::router());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
