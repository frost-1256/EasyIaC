use std::fs;
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "profile")]
    pub profile: Profile,
}


#[derive(Debug, Deserialize)]
pub struct Profile {
    pub server: String,
    pub user: String,
    pub passwd: String,
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
