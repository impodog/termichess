use super::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Config {
    pub unicode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { unicode: false }
    }
}

pub(crate) fn get_config() -> Result<Config, impl serde::de::Error> {
    let path = std::path::PathBuf::from(".").join("termichess.toml");
    let data = std::fs::read_to_string(&path).unwrap_or_default();
    toml::from_str(&data)
}
