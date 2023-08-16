#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Envy(#[from] envy::Error),

    #[error("Invalid credential type {0} provided")]
    InvalidCredentialType(String),

    #[error(transparent)]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
