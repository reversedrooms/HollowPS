use anyhow::Result;
use tokio::net::TcpListener;
use tracing::Instrument;

use crate::log_error;

use super::NetworkSession;

pub async fn listen(bind_addr: &str) -> Result<()> {
    let listener = TcpListener::bind(bind_addr).await?;
    tracing::info!("Listening at {bind_addr}");

    loop {
        let Ok((client_socket, client_addr)) = listener.accept().await else {
            continue;
        };

        tracing::info!("New session from {client_addr}");

        let mut session = NetworkSession::new(client_socket);
        tokio::spawn(
            async move {
                log_error!(
                    "Session from {client_addr} disconnected",
                    format!("An error occurred while processing session ({client_addr})"),
                    Box::pin(session.run()).await
                );
            }
            .instrument(tracing::info_span!("session", addr = %client_addr)),
        );
    }
}
