#[derive(thiserror::Error, Debug)]
pub enum DirectoryDeleteError {
    #[error("NOT_FOUND")]
    NotFound,
    #[error("NOT_EMPTY")]
    NotEmpty
}
