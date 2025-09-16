#[derive(thiserror::Error, Debug)]
pub enum FileDeleteError {
    #[error("DatabaseConnection")]
    DatabaseConnection,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("NOT_EXIST")]
    NotExist
}
