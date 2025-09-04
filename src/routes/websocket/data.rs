use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Deserialize, Debug)]
pub struct Inbound {
    #[serde(rename = "HEADERS", default)]
    pub headers: Map<String, Value>,
    #[serde(rename = "BODY", default)]
    pub body: Value,
    #[serde(flatten)]
    pub rest: Map<String, Value>,
}

pub fn sole_key(map: &Map<String, Value>) -> Option<String> {
    let mut it = map.keys().filter(|k| k.as_str() != "HEADERS");
    let first = it.next()?.to_owned();
    if it.next().is_none() {
        Some(first)
    } else {
        None
    }
}
