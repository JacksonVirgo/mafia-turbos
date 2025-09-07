use crate::{app::server::state::ServerState, routes};
use axum::Router;
use sqlx::{Pool, Postgres};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub async fn start_server(db: Pool<Postgres>) -> anyhow::Result<()> {
    let address = std::env::var("ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let uri = format!("{}:{}", address, port);

    let state = Arc::new(ServerState {
        db: db,
        rooms: Mutex::new(HashMap::new()),
    });

    let app = Router::new().merge(routes::router()).with_state(state);
    let listener = tokio::net::TcpListener::bind(uri).await?;

    let _ = axum::serve(listener, app).await?;

    Ok(())
}
