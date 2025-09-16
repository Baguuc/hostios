#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct File {
    pub full_path: String,
    pub tags: Vec<crate::models::Tag>
}

impl File {
    pub fn full_path(&self) -> String { self.full_path.clone() }
    
    pub fn tags(&self) -> Vec<crate::models::Tag> { self.tags.clone() }
}
