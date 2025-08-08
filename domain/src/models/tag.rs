#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow, Clone)]
pub struct Tag {
    name: String,
    description: String
}

impl Tag {
    pub fn name(&self) -> String { self.name.clone() }

    pub fn description(&self) -> String { self.description.clone() }
}
