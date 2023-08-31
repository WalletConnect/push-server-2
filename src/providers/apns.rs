use crate::providers::{Provider, ProviderPayload};
use crate::stores::tenant::TenantCredential;

pub enum ApnsProvider {
    Token(ApnsTokenProvider),
    Certificate(ApnsCertificateProvider),
}

pub struct ApnsTokenProvider {}

pub struct ApnsCertificateProvider {}

#[async_trait::async_trait]
impl Provider for ApnsProvider {
    async fn from(credential: TenantCredential) -> crate::prelude::Result<Self> {
        todo!()
    }

    async fn generate_payload(&self) -> crate::prelude::Result<ProviderPayload> {
        todo!()
    }

    async fn send(&self, token: &str, payload: ProviderPayload) -> crate::prelude::Result<()> {
        todo!()
    }
}