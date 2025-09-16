#[derive(thiserror::Error, Debug)]
pub enum FileMoveError {
    #[error("DatabaseConnection")]
    DatabaseConnection,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("NOT_EXIST")]
    NotExist,
    #[error("NEW_PARENT_NOT_EXIST")]
    NewParentNotExist
}
