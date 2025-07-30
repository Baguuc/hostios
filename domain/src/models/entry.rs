#[derive(Debug, serde::Serialize, Clone)]
pub struct Entry {
    pub full_path: String,
    pub tags: Vec<crate::Tag>,
    pub entry_type: EntryType
}

#[derive(Debug, serde::Serialize, Clone)]
pub enum EntryType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "directory")]
    Directory
}

impl Entry {
    pub fn full_path(&self) -> String { self.full_path.clone() }
    
    pub fn tags(&self) -> Vec<crate::Tag> { self.tags.clone() }
    
    pub fn entry_type(&self) -> EntryType { self.entry_type.clone() }
}
