#[derive(thiserror::Error, Debug)]
pub enum TagDeleteError {
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("NOT_EXIST")]
    NotExist
}
