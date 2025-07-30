#[derive(Debug, serde::Serialize)]
pub struct Directory {
    pub full_path: String,
    pub entries: Vec<crate::Entry>
}

impl Directory {
    pub fn full_path(&self) -> String { self.full_path.clone() }
    
    pub fn entries(&self) -> Vec<crate::Entry> { self.entries.clone() }
}
