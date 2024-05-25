use std::time::{SystemTime, UNIX_EPOCH};

pub fn load_or_create_config(path: &str, defaults: &str) -> String {
    std::fs::read_to_string(path).map_or_else(
        |_| {
            std::fs::write(path, defaults).unwrap();
            defaults.to_string()
        },
        |data| data,
    )
}

pub fn cur_timestamp_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

pub fn cur_timestamp_seconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}
