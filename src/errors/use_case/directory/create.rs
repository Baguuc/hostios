#[derive(thiserror::Error, Debug)]
pub enum DirectoryCreateError {
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("ALREADY_EXIST")]
    AlreadyExist,
    #[error("INVALID_PATH")]
    InvalidPath
}
