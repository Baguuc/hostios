#[derive(thiserror::Error, Debug)]
pub enum VaultReadError {
    #[error("INVALID_TOKEN")]
    InvalidToken,
    #[error("INVALID_PATH_SCOPE")]
    InvalidPathScope,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("VAULT_NOT_FOUND")]
    VaultNotFound,
    #[error("PATH_NOT_FOUND")]
    PathNotFound,
    #[error("AUTH_UNAVAIBLE")]
    AuthUnavaible
}
