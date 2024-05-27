use std::sync::LazyLock;

use serde_json::{Map, Value};

pub const EVENT_GRAPH_COLLECTION: &str = include_str!("../../EventGraphCollection.json");

static EVENT_MAP: LazyLock<Map<String, Value>> = LazyLock::new(|| {
    serde_json::from_str::<Value>(EVENT_GRAPH_COLLECTION)
        .unwrap()
        .as_object()
        .unwrap()
        .clone()
});

pub fn get_event_config_json(id: i32) -> &'static Value {
    EVENT_MAP.get(&id.to_string()).unwrap()
}
