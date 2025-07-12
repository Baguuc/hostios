#[derive(Debug, serde::Serialize)]
pub struct Directory {
    pub name: String,
    pub files: Vec<crate::File>,
    pub dirs: Vec<Self>
}
