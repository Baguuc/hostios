#[derive(Debug, serde::Serialize)]
pub struct Directory {
    pub full_path: String,
    pub name: String,
    pub files: Vec<crate::File>
}
