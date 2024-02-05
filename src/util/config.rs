use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub unicode: bool,
    pub address: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            unicode: false,
            address: "http://127.0.0.1:8080".to_string(),
        }
    }
}

pub(crate) fn get_config() -> Result<Config, impl serde::de::Error> {
    let path = std::path::PathBuf::from(".").join("termichess.toml");
    let data = std::fs::read_to_string(&path).unwrap_or_default();
    toml::from_str(&data)
}
