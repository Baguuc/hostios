#[derive(thiserror::Error, Debug)]
pub enum FileMoveError {
    #[error("NEW_PATH_PARENT_NOT_EXIST")]
    NewPathParentNotExist
}
