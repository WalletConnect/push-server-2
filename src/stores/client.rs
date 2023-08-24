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
    pub deleted_at: Option<DateTime<Utc>>
}

pub struct RelayMeta {
    pub url: String,
    pub id: String
}

#[async_trait::async_trait]
pub trait ClientStore {
    async fn create_or_update_client(&self, tenant_id: String, id: String, _type: ProviderKind, token: String, relay: RelayMeta) -> Result<Client>;
    async fn get_client(&self, tenant_id: String, id: String) -> Result<Client>;
    async fn delete_client(&self, tenant_id: String, id: String, soft: bool) -> Result<()>;
}

#[async_trait::async_trait]
impl ClientStore for PgPool {
    async fn create_or_update_client(&self, tenant_id: String, id: String, _type: ProviderKind, token: String, relay: RelayMeta) -> Result<Client> {
        todo!()
    }

    async fn get_client(&self, tenant_id: String, id: String) -> Result<Client> {
        todo!()
    }

    async fn delete_client(&self, tenant_id: String, id: String, soft: bool) -> Result<()> {
        todo!()
    }
}
