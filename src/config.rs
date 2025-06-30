// src/config.rs

use std::path::PathBuf;

use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StoreSettings {
    pub path: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub store: StoreSettings,
}

impl Settings {
    pub fn new(config_file: &str) -> Result<Self, ConfigError> {
        // Build the config from the specified file (e.g. "Settings.toml")
        let builder = Config::builder().add_source(File::with_name(config_file));

        let config = builder.build()?;
        config.try_deserialize()
    }

    pub fn db_path(&self) -> PathBuf {
        PathBuf::from(&self.store.path).join(&self.store.name)
    }
}
