use crate::providers::{Provider, ProviderPayload};
use crate::stores::tenant::TenantCredential;

pub enum ExampleProvider {
    ApiKey(ExampleApiKeyProvider),
    // ApiKey2(ExampleApiKey2Provider),
}

pub struct ExampleApiKeyProvider {}

// pub struct ExampleApiKey2Provider {}

#[async_trait::async_trait]
impl Provider for ExampleProvider {
    async fn from(credential: TenantCredential) -> crate::prelude::Result<Self> {
        todo!()
        /*
            Validate the provided credential and try to generate a provider from it.
         */
    }

    async fn generate_payload(&self) -> crate::prelude::Result<ProviderPayload> {
        todo!()
        /*
            Create the payload that will be sent to the push notification service.
            This should be tested!
        */
    }

    async fn send(&self, token: &str, payload: ProviderPayload) -> crate::prelude::Result<()> {
        todo!()
        /*
            Send the payload to the push notification service.
            This cannot be realistically tested.
         */
    }
}