// Default values defined as constants for testing
pub const DEFAULT_PORT: u16 = 3000;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Eq, PartialEq)]
pub struct EnvConfig {
    #[serde(default = "default_port")]
    pub port: u16,
    pub public_url: String,

    pub database_url: String,
}

fn default_port() -> u16 {
    DEFAULT_PORT
}

pub fn get_config() -> crate::prelude::Result<EnvConfig> {
    Ok(envy::from_env::<EnvConfig>()?)
}
