use crate::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub database: DatabaseConfig,
    pub data_dir: String,
    pub authios_url: String,
    pub service_permission: String
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String
}

impl Config {
    pub fn read(path: String) -> Result<Self> {
        use std::fs::read_to_string;
        use serde_json::from_str;

        let content = read_to_string(path)?;
        let mut parsed = from_str::<Self>(&content)?;

        parsed.data_dir = parsed.data_dir.trim_end_matches('/').to_string();

        return Ok(parsed);
    }
}
