#[derive(Debug, serde::Serialize)]
pub struct Entry {
    pub full_path: String,
    pub name: String,
    pub tags: Vec<String>,
    pub entry_type: EntryType
}

#[derive(Debug, serde::Serialize)]
pub enum EntryType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "directory")]
    Directory
}
