use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use common::DefaultProduct;
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
    #[serde(default, rename = "")]
    pub default_product: DefaultProductConfig,
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
    "sqlite://database.sqlite".to_owned()
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            database: default_database(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DefaultProductConfig {
    #[serde(default = "default_price")]
    pub price: i32,
    pub package_size: Option<String>,
    pub caffeine: Option<i32>,
    pub alcohol: Option<i32>,
    pub energy: Option<i32>,
    pub sugar: Option<i32>,
    #[serde(default = "default_active")]
    pub active: bool,
}

fn default_price() -> i32 {
    150
}

fn default_active() -> bool {
    true
}

impl Default for DefaultProductConfig {
    fn default() -> Self {
        Self {
            price: default_price(),
            active: default_active(),

            caffeine: Default::default(),
            alcohol: Default::default(),
            energy: Default::default(),
            sugar: Default::default(),
            package_size: Default::default(),
        }
    }
}

impl From<DefaultProductConfig> for DefaultProduct {
    fn from(config: DefaultProductConfig) -> Self {
        DefaultProduct {
            price: config.price,
            package_size: config.package_size,
            caffine: config.caffeine,
            alcohol: config.alcohol,
            energy: config.energy,
            sugar: config.sugar,
            active: config.active,
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
