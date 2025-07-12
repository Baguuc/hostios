#[derive(Debug, serde::Serialize)]
pub struct File {
    pub stash_path: String,
    pub name: String,
    pub tags: Vec<String>,
    pub file_type: FileType
}

#[derive(Debug, serde::Serialize)]
pub enum FileType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "directory")]
    Directory
}
