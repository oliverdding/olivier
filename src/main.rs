mod config;
mod error;
mod log;
mod service;

use anyhow::Result;
use tokio::time::Duration;
use tokio_graceful_shutdown::{SubsystemBuilder, Toplevel};

#[tokio::main]
async fn main() -> Result<()> {
    let global_config = config::GlobalConfig::new().await?;
    let _guard = log::configure_log(&global_config.log).await?;

    Toplevel::new(|s| async move {
        s.start(SubsystemBuilder::new("service", |a| {
            global_config.service.run(a)
        }));
    })
    .catch_signals()
    .handle_shutdown_requests(Duration::from_millis(1000))
    .await
    .map_err(Into::into)
}
