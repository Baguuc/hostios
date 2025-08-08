#[derive(Debug, serde::Serialize, Clone)]
#[serde(untagged)]
pub enum Entry {
    Directory { full_path: String },
    File(crate::File)
}
