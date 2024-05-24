use common::util::load_or_create_config;
use lazy_static::lazy_static;
use serde::Deserialize;

const DEFAULT_CONFIG: &str = include_str!("../gameserver.default.json");

#[derive(Deserialize)]
pub struct GameServerConfig {
    pub gateway_endpoint: String,
    pub skip_tutorial: bool,
    pub system_resources_logging: bool,
}

lazy_static! {
    pub static ref CONFIGURATION: GameServerConfig =
        serde_json::from_str(&load_or_create_config("gameserver.json", DEFAULT_CONFIG))
            .expect("Failed to parse server configuration file");
}

pub fn init_config() {
    let _configuration = &*CONFIGURATION; // init static
}
