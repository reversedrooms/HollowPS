use lazy_static::lazy_static;
use serde_json::{Map, Value};

pub const EVENT_GRAPH_COLLECTION: &str = include_str!("../../EventGraphCollection.json");

lazy_static! {
    static ref EVENT_MAP: Map<String, Value> = {
        serde_json::from_str::<Value>(EVENT_GRAPH_COLLECTION)
            .unwrap()
            .as_object()
            .unwrap()
            .clone()
    };
}

pub fn get_event_config_json(id: i32) -> &'static Value {
    EVENT_MAP.get(&id.to_string()).unwrap()
}
