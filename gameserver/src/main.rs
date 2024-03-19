use std::path::Path;

use anyhow::Result;
use tracing::Level;

mod game;
mod logging;
mod net;

use logging::{init_system_logging, init_tracing};

const GATE_HOST: &str = "0.0.0.0";
const GATE_PORT: u16 = 10301;

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support().unwrap();

    init_config()?;
    init_tracing();

    let span = tracing::span!(Level::DEBUG, "main");
    let _enter = span.enter();

    init_system_logging().await;

    net::gateway::listen(GATE_HOST, GATE_PORT).await?;
    Ok(())
}

fn init_config() -> Result<()> {
    let local_dotenv = Path::new(".env");
    if local_dotenv.exists() {
        dotenv::dotenv()?;
    } else {
        let config = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("No config directory found"))?
            .join("nap-gameserver");

        std::fs::create_dir_all(&config)?;

        let env = config.join(".env");

        if !env.exists() {
            std::fs::write(&env, "SKIP_TUTORIAL=0")?;
        }

        dotenv::from_path(&env)?;
    }

    Ok(())
}
