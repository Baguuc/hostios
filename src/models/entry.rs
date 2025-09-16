#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "path", rename_all = "lowercase")]
pub enum Entry {
    Directory(crate::models::Path),
    File(crate::models::Path)
}
