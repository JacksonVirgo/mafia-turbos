use crate::routes;
use axum::Router;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct ServerState {
    pub db: Pool<Postgres>,
}

pub async fn start_server(db: Pool<Postgres>) -> anyhow::Result<()> {
    let address = std::env::var("ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let uri = format!("{}:{}", address, port);

    let state = ServerState { db };

    let app = Router::new().with_state(state).merge(routes::router());
    let listener = tokio::net::TcpListener::bind(uri).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
