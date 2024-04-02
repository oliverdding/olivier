use crate::config::LogConfig;
use anyhow::{Context, Result};
use tracing::warn;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Layer};

pub fn init(log_config: &LogConfig) -> Result<Option<WorkerGuard>> {
    LogTracer::init().context("failed to initialize log tracer")?;

    // used to prompt the user whether the log configuration is fallback to the default configuration due to an error
    let (mut console_fall_back, mut file_fall_back) = (None, None);

    let env_filter = EnvFilter::try_new(&log_config.directives).unwrap_or_else(|e| {
        console_fall_back = Some(e);
        EnvFilter::new("warm")
    });

    let stdout_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_filter(env_filter);

    if !log_config.file.enabled {
        let subscriber = tracing_subscriber::registry().with(stdout_subscriber);

        tracing::subscriber::set_global_default(subscriber)
            .context("unable to set global subscriber")?;

        if let Some(err) = console_fall_back {
            warn!(
                "invalid log directives '{}' for console logging, fall back to 'warm': {}",
                &log_config.directives, err
            )
        }

        return Ok(None);
    }

    let file_appender = tracing_appender::rolling::daily(&log_config.file.path, "logs.json");

    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter = EnvFilter::try_new(&log_config.file.directives).unwrap_or_else(|e| {
        file_fall_back = Some(e);
        EnvFilter::new("info")
    });

    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .json()
        .with_writer(file_writer)
        .with_filter(env_filter);

    let subscriber = tracing_subscriber::registry()
        .with(stdout_subscriber)
        .with(file_subscriber);

    tracing::subscriber::set_global_default(subscriber)
        .context("unable to set global subscriber")?;

    if let Some(err) = console_fall_back {
        warn!(
            "invalid log directives '{}' for console logging, fall back to 'warm': {}",
            &log_config.directives, err
        )
    }

    if let Some(err) = file_fall_back {
        warn!(
            "invalid log directives '{}' for file logging, fall back to 'info': {}",
            &log_config.file.directives, err
        )
    }

    Ok(Some(guard))
}
