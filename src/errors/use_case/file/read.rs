#[derive(thiserror::Error, Debug)]
pub enum FileReadError {
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("INVALID_PATH")]
    NotExist
}
