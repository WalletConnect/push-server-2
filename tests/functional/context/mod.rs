use {
    self::server::PushServer,
    async_trait::async_trait,
    push_server::state::{StoreArc, TenantStoreArc},
    sqlx::{Pool, Postgres},
    std::sync::Arc,
    test_context::AsyncTestContext,
};

mod server;
mod store;

#[derive(thiserror::Error, Debug)]
pub enum ContextError {
    #[error(transparent)]
    Elapsed(#[from] tokio::time::error::Elapsed),
}

pub const DATABASE_URL: &str = "postgres://postgres:root@localhost:5432/postgres";
#[cfg(feature = "multitenant")]
pub const TENANT_DATABASE_URL: &str = "postgres://postgres:root@localhost:5433/postgres";

pub struct ServerContext(pub(crate) PushServer);

pub struct StoreContext {
    pub pool: Arc<Pool<Postgres>>,
    #[cfg(feature = "multitenant")]
    pub tenant_pool: Arc<Pool<Postgres>>,

    pub store: StoreArc,
    #[cfg(feature = "multitenant")]
    pub tenant_store: TenantStoreArc,
}

#[async_trait]
impl AsyncTestContext for ServerContext {
    async fn setup() -> Self {
        let server = PushServer::start().await;
        Self(server)
    }

    async fn teardown(mut self) {
        self.0.shutdown().await;
    }
}

#[async_trait]
impl AsyncTestContext for StoreContext {
    async fn setup() -> Self {
        #[cfg(not(feature = "multitenant"))]
        let db = store::open_pg_connections().await;

        #[cfg(feature = "multitenant")]
        let (db, tenant_db) = store::open_pg_connections().await;

        let db_arc = Arc::new(db);
        #[cfg(feature = "multitenant")]
        let tenant_db_arc = Arc::new(tenant_db);

        Self {
            pool: db_arc.clone(),
            #[cfg(feature = "multitenant")]
            tenant_pool: tenant_db_arc.clone(),
            store: db_arc,
            #[cfg(feature = "multitenant")]
            tenant_store: tenant_db_arc,
        }
    }

    async fn teardown(self) {
        self.pool.close().await;
        self.tenant_pool.close().await;
    }
}