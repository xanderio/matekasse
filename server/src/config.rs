use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use serde::Deserialize;

const CONFIG_FILENAME: &str = "config.toml";

#[derive(Debug)]
pub enum ConfigError {
    IoError(String),
    Parser(String),
}

pub async fn load_config() -> Result<Config, ConfigError> {
    let path = PathBuf::new().join(CONFIG_FILENAME);

    if !path.is_file() {
        println!("config file not found, using defaults");
        return Ok(Config::default());
    }

    let data = tokio::fs::read_to_string(path)
        .await
        .map_err(|e| ConfigError::IoError(e.to_string()))?;

    toml::from_str(&data).map_err(|e| ConfigError::Parser(e.to_string()))
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub http: HttpConfig,
    #[serde(default)]
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct HttpConfig {
    #[serde(default = "default_listen")]
    pub listen: SocketAddr,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            listen: default_listen(),
        }
    }
}

fn default_listen() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000)
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct StorageConfig {
    #[serde(default = "default_database")]
    pub database: String,
}

fn default_database() -> String {
    "sqlite://./database.sqlite".to_owned()
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            database: default_database(),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    const DEFAULT_CONFIG: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.default.toml"));

    #[test]
    fn default_values_match_default_config_file() {
        let default_config =
            toml::from_str(DEFAULT_CONFIG).expect("unable to parse default config");

        assert_eq!(super::Config::default(), default_config);
    }
}
