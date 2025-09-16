#[derive(thiserror::Error, Debug)]
pub enum DirectoryDeleteError {
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("NOT_EXIST")]
    NotExist,
    #[error("NOT_EMPTY")]
    NotEmpty,
}
