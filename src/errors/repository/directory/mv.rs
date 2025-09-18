#[derive(thiserror::Error, Debug)]
pub enum DirectoryMoveError {
    #[error("NEW_PATH_PARENT_NOT_EXIST")]
    NewPathParentNotExist
}
