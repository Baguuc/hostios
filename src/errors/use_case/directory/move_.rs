#[derive(thiserror::Error, Debug)]
pub enum DirectoryMoveError {
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("NOT_EXIST")]
    NotExist,
    #[error("NEW_PARENT_PATH_NOT_EXIST")]
    NewParentPathNotExist,
}
