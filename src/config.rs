#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub base_system_dir: String,
    pub authios_url: String
}

impl Config {
    pub fn read(path: String) -> Result<Self, ConfigReadError> {
        use std::fs::read_to_string;
        use serde_json::from_str;

        let content = read_to_string(path)?;
        let parsed = from_str::<Self>(&content)?;

        return Ok(parsed);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigReadError {
    #[error("IO_ERROR:{0}")]
    IO(#[from] std::io::Error),
    
    #[error("DESERIALIZATION:{0}")]
    Deserialization(#[from] serde_json::Error)
}
