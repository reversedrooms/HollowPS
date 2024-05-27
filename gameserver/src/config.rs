use std::sync::LazyLock;

use common::util::load_or_create_config;
use serde::Deserialize;

const DEFAULT_CONFIG: &str = include_str!("../gameserver.default.json");

#[derive(Deserialize)]
pub struct GameServerConfig {
    pub gateway_endpoint: String,
    pub skip_tutorial: bool,
    pub system_resources_logging: bool,
}

pub static CONFIGURATION: LazyLock<GameServerConfig> = LazyLock::new(|| {
    serde_json::from_str(&load_or_create_config("gameserver.json", DEFAULT_CONFIG))
        .expect("Failed to parse server configuration file")
});

pub fn init_config() {
    let _configuration = &*CONFIGURATION;
}
