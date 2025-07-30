#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Generic(String), 

    #[error("{0}")]
    IO(#[from] std::io::Error),
    
    #[error("{0}")]
    Sqlx(#[from] sqlx::Error),
}
