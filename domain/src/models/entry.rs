#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Entry {
    Directory(crate::Path),
    File(crate::Path)
}
