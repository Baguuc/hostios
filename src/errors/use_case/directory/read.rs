#[derive(thiserror::Error, Debug)]
pub enum DirectoryReadError {
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("NOT_EXIST")]
    NotExist,
}
