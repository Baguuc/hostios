#[derive(thiserror::Error, Debug)]
pub enum VaultListFromTokenError {
    #[error("INVALID_TOKEN")] 
    InvalidToken,
    #[error("CANNOT_FETCH")]
    CannotFetch,
}
