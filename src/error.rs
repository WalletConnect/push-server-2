#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Envy(#[from] envy::Error),

    #[error("Invalid credential type {0} provided")]
    InvalidCredentialType(String),
}
