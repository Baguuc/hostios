#[derive(Debug, serde::Serialize)]
pub struct File {
    pub name: String,
    pub tags: Vec<String>
}

