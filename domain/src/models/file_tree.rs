#[derive(Debug, serde::Serialize)]
pub struct FileTree {
    pub root: String,
    pub main_directory: crate::Directory
}
