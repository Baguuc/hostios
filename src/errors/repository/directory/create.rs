#[derive(thiserror::Error, Debug)]
pub enum DirectoryCreateError {
    #[error("ALREADY_EXIST")]
    AlreadyExist
}
