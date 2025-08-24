use constants::{APP_NAME, GLOBAL_LOG_DIR};
pub use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
/// Initialize logger
pub fn init_logger(level: &str) -> Result<(), String> {
    tracing_subscriber::registry()
        .with(EnvFilter::new(level))
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_level(true)
                .with_target(true)
                .with_writer(tracing_appender::rolling::daily(
                    GLOBAL_LOG_DIR.clone(),
                    format!("{}.log", APP_NAME),
                )),
        )
        .init();
    Ok(())
}
