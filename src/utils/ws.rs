use std::{collections::HashSet, sync::Mutex};
use tokio::sync::broadcast;

pub struct WebSocketState<T> {
    pub id_set: Mutex<HashSet<T>>,
    pub tx: broadcast::Sender<String>,
}
