#[derive(thiserror::Error, Debug)]
pub enum FileDeleteError {
    #[error("NotFound")]
    NotFound
}
