use anyhow::Result;
use tracing::Level;

mod config;
mod data;
mod game;
mod logging;
mod net;

use config::{init_config, CONFIGURATION};
use data::init_assets;
use logging::{init_logging, init_system_logging};

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();
    init_config();
    init_assets()?;

    let span = tracing::span!(Level::DEBUG, "main");
    let _enter = span.enter();

    if CONFIGURATION.system_resources_logging {
        init_system_logging().await;
    }

    net::gateway::listen(&CONFIGURATION.gateway_endpoint).await
}
