use {
    crate::prelude::*,
    async_trait::async_trait,
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
    serde_json::json,
    sqlx::{types::JsonValue, PgPool},
    uuid::Uuid,
};

#[derive(sqlx::FromRow, Debug, Eq, PartialEq, Clone)]
pub struct Tenant {
    pub id: String,

    pub suspended: bool,
    pub suspended_reason: Option<String>,

    pub credentials: Vec<TenantCredential>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "tenant_credential_type")]
#[sqlx(rename_all = "lowercase")]
pub enum TenantCredentialType {
    ApnsToken,
    ApnsCertificate,
    /// Legacy API Key
    FcmKey,
    /// google-services.json
    FcmFile,
}

impl Into<String> for TenantCredentialType {
    fn into(self) -> String {
        match self {
            TenantCredentialType::ApnsToken => "apns_token",
            TenantCredentialType::ApnsCertificate => "apns_certificate",
            TenantCredentialType::FcmKey => "fcm_key",
            TenantCredentialType::FcmFile => "fcm_file",
        }
        .to_string()
    }
}

impl TryFrom<&str> for TenantCredentialType {
    type Error = crate::error::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "apns_token" => Ok(TenantCredentialType::ApnsToken),
            "apns_certificate" => Ok(TenantCredentialType::ApnsCertificate),
            "fcm_key" => Ok(TenantCredentialType::FcmKey),
            "fcm_file" => Ok(TenantCredentialType::FcmFile),
            t => Err(crate::error::Error::InvalidCredentialType(t.to_string())),
        }
    }
}

#[derive(sqlx::FromRow, Debug, Eq, PartialEq, Clone)]
pub struct TenantCredential {
    pub id: String,
    pub tenant_id: String,

    #[sqlx(rename = "type")]
    pub _type: TenantCredentialType,

    /// Encoded File
    pub file: Option<String>,
    /// Extra values as json column
    pub values: JsonValue,
}

impl TenantCredential {
    pub fn from_fcm_key(tenant_id: &str, key: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            _type: TenantCredentialType::FcmKey,
            file: None,
            values: json!({ "api_key": key }),
        }
    }

    pub fn from_fcm_file(tenant_id: &str, file_contents: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            _type: TenantCredentialType::FcmFile,
            file: Some(file_contents.to_string()),
            values: json!({}),
        }
    }

    pub fn from_apns_certificate(tenant_id: &str, certificate: &str, password: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            _type: TenantCredentialType::ApnsCertificate,
            file: Some(certificate.to_string()),
            values: json!({ "password": password }),
        }
    }

    pub fn from_apns_token(tenant_id: &str, pk8s_pem: &str, key_id: &str, team_id: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            _type: TenantCredentialType::ApnsToken,
            file: Some(pk8s_pem.to_string()),
            values: json!({ "key_id": key_id, "team_id": team_id }),
        }
    }
}

#[async_trait::async_trait]
pub trait TenantStore {
    async fn get_tenant(&self, id: &str) -> Result<Tenant>;
    async fn delete_tenant(&self, id: &str) -> Result<()>;
    async fn create_tenant(&self, id: &str) -> Result<Tenant>;

    async fn suspend_tenant(&self, id: &str, reason: &str) -> Result<()>;
    async fn unsuspend_tenant(&self, id: &str) -> Result<()>;

    async fn get_credentials_by_type(
        &self,
        id: &str,
        credential_type: TenantCredentialType,
    ) -> Result<Vec<TenantCredentialType>>;
    async fn create_credential(&self, id: &str, credential: TenantCredentialType) -> Result<()>;
    async fn remove_credential_by_id(&self, id: &str, credential_id: &str) -> Result<()>;
    async fn remove_credential_by_type(
        &self,
        id: &str,
        credential_type: TenantCredentialType,
    ) -> Result<()>;
}

#[derive(Clone, Debug)]
pub struct DefaultTenantStore(Tenant);

pub const DEFAULT_TENANT_ID: &str = "00000000-0000-0000-0000-000000000000";

impl DefaultTenantStore {
    pub fn new() -> Self {
        Self(Tenant {
            id: DEFAULT_TENANT_ID.to_string(),
            suspended: false,
            suspended_reason: None,
            credentials: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn add_credential(&self, cred: TenantCredential) -> Self {
        let mut new_credentials = self.0.credentials.clone();
        new_credentials.push(cred);

        Self(Tenant {
            credentials: new_credentials,
            ..self.0.clone()
        })
    }
}

#[async_trait::async_trait]
impl TenantStore for PgPool {
    async fn get_tenant(&self, id: &str) -> Result<Tenant> {
        todo!()
    }

    async fn delete_tenant(&self, id: &str) -> Result<()> {
        todo!()
    }

    async fn create_tenant(&self, id: &str) -> Result<Tenant> {
        todo!()
    }

    async fn suspend_tenant(&self, id: &str, reason: &str) -> Result<()> {
        todo!()
    }

    async fn unsuspend_tenant(&self, id: &str) -> Result<()> {
        todo!()
    }

    async fn get_credentials_by_type(
        &self,
        id: &str,
        credential_type: TenantCredentialType,
    ) -> Result<Vec<TenantCredentialType>> {
        todo!()
    }

    async fn create_credential(&self, id: &str, credential: TenantCredentialType) -> Result<()> {
        todo!()
    }

    async fn remove_credential_by_id(&self, id: &str, credential_id: &str) -> Result<()> {
        todo!()
    }

    async fn remove_credential_by_type(
        &self,
        id: &str,
        credential_type: TenantCredentialType,
    ) -> Result<()> {
        todo!()
    }
}

#[async_trait::async_trait]
impl TenantStore for DefaultTenantStore {
    async fn get_tenant(&self, id: &str) -> Result<Tenant> {
        panic!("TenantStore functions cannot be called on DefaultTenantStore")
    }

    async fn delete_tenant(&self, id: &str) -> Result<()> {
        panic!("TenantStore functions cannot be called on DefaultTenantStore")
    }

    async fn create_tenant(&self, id: &str) -> Result<Tenant> {
        panic!("TenantStore functions cannot be called on DefaultTenantStore")
    }

    async fn suspend_tenant(&self, id: &str, reason: &str) -> Result<()> {
        panic!("TenantStore functions cannot be called on DefaultTenantStore")
    }

    async fn unsuspend_tenant(&self, id: &str) -> Result<()> {
        panic!("TenantStore functions cannot be called on DefaultTenantStore")
    }

    async fn get_credentials_by_type(
        &self,
        id: &str,
        credential_type: TenantCredentialType,
    ) -> Result<Vec<TenantCredentialType>> {
        panic!("TenantStore functions cannot be called on DefaultTenantStore")
    }

    async fn create_credential(&self, id: &str, credential: TenantCredentialType) -> Result<()> {
        panic!("TenantStore functions cannot be called on DefaultTenantStore")
    }

    async fn remove_credential_by_id(&self, id: &str, credential_id: &str) -> Result<()> {
        panic!("TenantStore functions cannot be called on DefaultTenantStore")
    }

    async fn remove_credential_by_type(
        &self,
        id: &str,
        credential_type: TenantCredentialType,
    ) -> Result<()> {
        panic!("TenantStore functions cannot be called on DefaultTenantStore")
    }
}
