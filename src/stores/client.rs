use {
    crate::prelude::*,
    chrono::{DateTime, Utc},
    sqlx::PgPool,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "provider")]
#[sqlx(rename_all = "lowercase")]
pub enum ProviderKind {
    Apns,
    ApnsSandbox,
    Fcm,
    #[cfg(any(debug_assertions, test))]
    Noop,
}

#[derive(sqlx::FromRow, Debug, Eq, PartialEq, Clone)]
pub struct Client {
    pub client_id: String,
    pub tenant_id: String,

    #[sqlx(rename = "type")]
    pub _type: ProviderKind,

    pub token: String,

    pub relay_url: String,
    pub relay_id: String,

    pub registered_at: DateTime<Utc>,
}

#[async_trait::async_trait]
pub trait ClientStore {
    async fn create_or_update_client(&self) -> Result<Client>;
}

#[async_trait::async_trait]
impl ClientStore for PgPool {
    async fn create_or_update_client(&self) -> Result<Client> {
        todo!()
    }
}
