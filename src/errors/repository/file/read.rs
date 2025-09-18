#[derive(thiserror::Error, Debug)]
pub enum FileReadError {
    #[error("NOT_FOUND")]
    NotFound
}
