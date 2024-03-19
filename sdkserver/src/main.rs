use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use axum::{
    routing::{get, post},
    Router,
};

mod crypto;
mod services;

use services::{auth, config, entry, errors};

const HOST: &str = "0.0.0.0";
const PORT: u16 = 21000;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing()?;

    let router = Router::new()
        .route(auth::RISKY_API_CHECK_ENDPOINT, post(auth::risky_api_check))
        .route(
            auth::LOGIN_WITH_PASSWORD_ENDPOINT,
            post(auth::login_with_password),
        )
        .route(
            auth::LOGIN_WITH_SESSION_TOKEN_ENDPOINT,
            post(auth::login_with_session_token),
        )
        .route(
            auth::GRANTER_LOGIN_VERIFICATION_ENDPOINT,
            post(auth::granter_login_verification),
        )
        .route(config::APP_CONFIG_ENDPOINT, get(config::application))
        .route(config::SERVER_LIST_ENDPOINT, get(config::server_list))
        .route(
            config::VERSIONS_BUNDLE_ENDPOINT,
            get(config::versions_bundle),
        )
        .route(entry::ACCOUNT_TOKEN_ENDPOINT, post(entry::account_token))
        .route(entry::ACCOUNT_SERVER_ENDPOINT, post(entry::account_server))
        .fallback(errors::not_found);

    let bind_url = format!("{HOST}:{PORT}");
    let http_server = axum_server::bind(bind_url.parse()?);
    tracing::info!("SDK Server is listening at {bind_url}");

    http_server.serve(router.into_make_service()).await?;
    Ok(())
}

fn init_tracing() -> Result<()> {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support().unwrap();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let log_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::try_new("info").unwrap())
        .add_directive("sdkserver=debug".parse().unwrap());

    tracing::subscriber::set_global_default(
        Registry::default()
            .with(log_filter)
            .with(tracing_subscriber::fmt::Layer::default()),
    )?;

    Ok(())
}
