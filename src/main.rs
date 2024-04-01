mod config;
mod dto;
mod error;
mod handler;
mod log;
mod router;
mod server;

use anyhow::Result;
use server::AppServer;
use tokio::time::Duration;
use tokio_graceful_shutdown::{SubsystemBuilder, Toplevel};

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::new()?;
    let _guard = log::init(&config.log)?;

    let server = AppServer::new(config).await?;

    Toplevel::new(|s| async move {
        s.start(SubsystemBuilder::new("service", |a| server.run(a)));
    })
    .catch_signals()
    .handle_shutdown_requests(Duration::from_millis(1000))
    .await
    .map_err(Into::into)
}
