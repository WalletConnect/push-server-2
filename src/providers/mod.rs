pub mod apns;
pub mod fcm;
pub mod noop;

use std::sync::Arc;
use crate::prelude::*;
use crate::stores::tenant::TenantCredential;

pub enum ProviderPayload {}

pub type ProviderArc = Arc<dyn Provider + Send + Sync>;

#[async_trait::async_trait]
/// A provider is the abstraction of a push notification service e.g. APNS, FCM, etc.
///
/// Providers are responsible for generating the payload to be sent to the push notification service as well as actually sending them to the users device by using a token.
pub trait Provider {
    /// Create a new provider from a tenant credential.
    async fn from(credential: TenantCredential) -> Result<Self>;

    /// Generate the payload to be sent to the push notification service.
    async fn generate_payload(&self) -> Result<ProviderPayload>;

    /// Send the payload to the push notification service by using the device token.
    async fn send(&self, token: &str, payload: ProviderPayload) -> Result<()>;
}
