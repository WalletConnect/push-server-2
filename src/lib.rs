use tokio::sync::broadcast;
use crate::config::EnvConfig;

pub mod config;
pub mod error;
pub mod state;
pub mod log;

pub mod prelude {
    pub type Result<T> = std::result::Result<T, crate::error::Error>;

    pub use tracing::{debug, error, info, warn};
}

pub async fn bootstrap(shutdown: broadcast::Receiver<()>, config: EnvConfig) -> prelude::Result<()> {
    Ok(())
}
