#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Entry {
    Directory(crate::models::Directory),
    File(crate::models::File)
}
