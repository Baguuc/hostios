#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "path", rename_all = "lowercase")]
pub enum Entry {
    Directory(crate::Path),
    File(crate::Path)
}
