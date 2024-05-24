pub fn load_or_create_config(path: &str, defaults: &str) -> String {
    std::fs::read_to_string(path).map_or_else(
        |_| {
            std::fs::write(path, defaults).unwrap();
            defaults.to_string()
        },
        |data| data,
    )
}
