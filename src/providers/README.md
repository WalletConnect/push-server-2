# Providers

> [!NOTE]
> This is an internal/contributor doc to explain the interfaces and how they should be used within Push Server. This cannot be accessed/used by end users.

## Overview
A provider is the abstraction to a service which can send push notifications. This can be a service like Firebase Cloud Messaging (FCM), Apple Push Notification Service (APNS), or any other service which can send push notifications.

## Provider Interface
```rs
#[async_trait::async_trait]
pub trait Provider {
    async fn from(credential: TenantCredential) -> Result<Self>;

    async fn generate_payload(&self) -> Result<ProviderPayload>;

    async fn send(&self, token: &str, payload: ProviderPayload) -> Result<()>;
}
```

### `async fn from(credential: TenantCredential) -> Result<Self>;`
This function is used to create a new instance of the provider. This is called when the server is starting up and will be called once per provider. This is where the provider should do any setup it needs to do to be able to send push notifications.

It should validate the credentials from the `TenantCredential` and return an error if they are invalid or are for a different provider.

### `async fn generate_payload(&self, payload: PushMessagePayload) -> Result<ProviderPayload>;`
This function is used to generate the payload which will be sent to the provider. This is called in preparation to send a notification to the user.

This function should have unit tests which validate that given a payload from the server, the correct payload is generated for the provider.

### `async fn send(&self, token: &str, payload: ProviderPayload) -> Result<()>;`
This function is used to send the push notification to the provider. This is called when a push notification is being sent to a user.

We cannot test this as it requires a device to be alive, registered and able to report if it received a notification.

## Current Providers
We currently support the following providers, each provider has an explanation of its auth methods.

### FCM
FCM currently only supports legacy API Keys, but has been scaffolded out to have support for google services as well.

### APNS
APNS support both Certificate and Token authentication, we use [a2](https://github.com/walletconnect/a2) which is a library for sending push notifications to APNS.
