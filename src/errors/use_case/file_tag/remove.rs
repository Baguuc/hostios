#[derive(thiserror::Error, Debug)]
pub enum FileTagRemoveError {
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("TAG_NOT_EXIST")]
    TagNotExist,
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("PATH_NOT_EXIST")]
    PathNotExist,
    #[error("ALREADY_ADDED")]
    NotAddedYet
}
