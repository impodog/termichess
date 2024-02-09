use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub unicode: bool,
    pub address: String,
    pub spacing: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            unicode: false,
            address: "http://127.0.0.1:8080".to_string(),
            spacing: 3,
        }
    }
}

impl Config {
    pub fn get_spaces(&self) -> String {
        let mut s = String::new();
        for _ in 0..self.spacing - 1 {
            s.push(' ');
        }
        s
    }

    pub fn modify(mut self) -> Self {
        if !self.unicode {
            self.spacing = 3;
        }
        self
    }
}

pub(crate) fn get_config() -> Result<Config, impl serde::de::Error> {
    let path = std::path::PathBuf::from(".").join("termichess.toml");
    let data = std::fs::read_to_string(path).unwrap_or_default();
    toml::from_str(&data)
}
