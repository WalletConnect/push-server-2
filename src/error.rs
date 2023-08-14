#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Envy(#[from] envy::Error),
}
