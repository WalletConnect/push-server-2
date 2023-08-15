use {
    crate::config::EnvConfig,
    axum::{routing::get, Router},
    std::{net::SocketAddr, sync::Arc},
    tokio::{select, sync::broadcast},
    tower::ServiceBuilder,
};

pub mod config;
pub mod error;
pub mod handlers;
pub mod log;
pub mod state;

pub mod prelude {
    pub type Result<T> = std::result::Result<T, crate::error::Error>;

    pub use tracing::{debug, error, info, warn};
}

use {crate::state::AppState, prelude::*};

pub async fn bootstrap(
    mut shutdown: broadcast::Receiver<()>,
    config: EnvConfig,
) -> prelude::Result<()> {
    let port = config.port;

    let state = AppState::new(config);
    let state_arc = Arc::new(state);

    let global_middleware = ServiceBuilder::new();

    let app = Router::new()
        .route("/health", get(handlers::health_handler))
        .layer(global_middleware)
        .with_state(state_arc);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    select! {
        _ = axum::Server::bind(&addr).serve(app.into_make_service()) => info!("Server terminating"),
        _ = shutdown.recv() => info!("Shutdown signal received, killing servers"),
    }

    Ok(())
}
