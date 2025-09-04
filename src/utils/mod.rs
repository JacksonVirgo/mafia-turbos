pub mod ws;

pub type WebsocketResponseHandler<T, V> = fn(&T) -> anyhow::Result<V>;
