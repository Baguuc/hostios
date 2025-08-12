#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "path")]
pub enum Entry {
    Directory(crate::Path),
    File(crate::Path)
}
