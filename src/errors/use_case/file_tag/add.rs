#[derive(thiserror::Error, Debug)]
pub enum FileTagAddError {
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("TAG_NOT_EXIST")]
    TagNotExist,
    #[error("PATH_NOT_EXIST")]
    PathNotExist,
    #[error("ALREADY_ADDED")]
    AlreadyAdded,
}
