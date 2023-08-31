use {
    crate::config::EnvConfig,
    axum::{routing::get, Router},
    sqlx::{
        postgres::{PgConnectOptions, PgPoolOptions},
        ConnectOptions,
    },
    std::{net::SocketAddr, str::FromStr, sync::Arc, time::Duration},
    tokio::{select, sync::broadcast},
    tower::ServiceBuilder,
    tracing::log::LevelFilter,
};

pub mod config;
pub mod error;
pub mod handlers;
pub mod log;
pub mod state;
pub mod stores;
pub mod providers;

pub mod prelude {
    pub type Result<T> = std::result::Result<T, crate::error::Error>;

    pub use tracing::{debug, error, info, warn};
}

use {crate::state::AppState, prelude::*};

#[cfg(not(feature = "multitenant"))]
use crate::stores::tenant::DefaultTenantStore;

pub async fn bootstrap(
    mut shutdown: broadcast::Receiver<()>,
    config: EnvConfig,
) -> prelude::Result<()> {
    let port = config.port;

    let pg_options = PgConnectOptions::from_str(&config.database_url)?
        .log_statements(LevelFilter::Debug)
        .log_slow_statements(LevelFilter::Warn, Duration::from_millis(250))
        .clone();

    let database = PgPoolOptions::new()
        .max_connections(15)
        .min_connections(5)
        .connect_with(pg_options)
        .await?;

    sqlx::migrate!("./migrations").run(&database).await?;

    let store = Arc::new(database);

    #[cfg(not(feature = "multitenant"))]
    let tenant_store = Arc::new(DefaultTenantStore::new());

    #[cfg(feature = "multitenant")]
    let tenant_store = {
        let tenant_pg_options = PgConnectOptions::from_str(&config.tenant_database_url)?
            .log_statements(LevelFilter::Debug)
            .log_slow_statements(LevelFilter::Warn, Duration::from_millis(250))
            .clone();

        let tenant_database = PgPoolOptions::new()
            .max_connections(15)
            .min_connections(5)
            .connect_with(tenant_pg_options)
            .await?;

        sqlx::migrate!("./tenant_migrations")
            .run(&tenant_database)
            .await?;

        Arc::new(tenant_database)
    };

    let state = AppState::new(config, store, tenant_store);
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
