use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use sqlx::{Pool, Postgres};
use tokio::sync::broadcast;

pub struct ServerState {
    pub db: Pool<Postgres>,
    pub rooms: Mutex<HashMap<String, RoomState>>,
}

pub struct RoomState {
    pub user_set: HashSet<String>,
    pub tx: broadcast::Sender<String>,
}

impl RoomState {
    pub fn new() -> Self {
        Self {
            user_set: HashSet::new(),
            tx: broadcast::channel(25).0,
        }
    }
}
