use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::source_provider::{FetchMode, DynSource};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    #[serde(default)]
    fetch_mode: FetchMode,

    #[serde(default)]
    auto_select_lane: bool,

    #[serde(default="AppConfig::default_expired")]
    expired_days: i64,

    #[serde(default)]
    mini_mode: bool,

    #[serde(skip_deserializing)]
    data_path: PathBuf,
    #[serde(skip_deserializing)]
    cache_path: PathBuf,
    #[serde(skip_deserializing)]
    config_path: PathBuf,
}

impl AppConfig {
    pub fn new() -> Self {
        let proj_dir = directories::ProjectDirs::from("com", "Jinte", "Lola").unwrap();
        AppConfig {
            fetch_mode: FetchMode::Auto,
            auto_select_lane: false,
            expired_days: AppConfig::default_expired(),
            mini_mode: false,
            cache_path: proj_dir.cache_dir().to_owned(),
            config_path: proj_dir.config_dir().to_owned(),
            data_path: proj_dir.data_dir().to_owned(),
        }
    }

    pub fn default_expired()-> i64 {
        3
    }

    pub fn init_meta(&mut self) {
        let proj_dir = directories::ProjectDirs::from("com", "Jinte", "Lola").unwrap();
        self.cache_path = proj_dir.cache_dir().to_owned();
        self.config_path = proj_dir.config_dir().to_owned();
        self.data_path = proj_dir.data_dir().to_owned();
    }

    pub async fn from_local() -> Self {
        if let Some(proj_dir) = directories::ProjectDirs::from("com", "Jinte", "Lola") {
            let config_file = proj_dir.config_dir().join("config.json");
            fs::create_dir_all(proj_dir.config_dir()).unwrap();
            if config_file.is_file() {
                let mut file = File::open(config_file)
                    .await
                    .expect("Can't open to read the config file.");
                let mut json_text = String::new();
                file.read_to_string(&mut json_text).await.unwrap();
                let mut local: Self = serde_json::from_str(&json_text).unwrap();
                local.init_meta();
                local
            } else {
                let mut file = File::create(config_file)
                    .await
                    .expect("Can't create config file.");
                let config = AppConfig::new();
                file.write_all(serde_json::to_string_pretty(&config).unwrap().as_bytes())
                    .await
                    .unwrap();
                config
            }
        } else {
            panic!("Can't find config path.");
        }
    }

    pub async fn save_to_local(&self) {
        if let Some(proj_dir) = directories::ProjectDirs::from("com", "Jinte", "Lola") {
            let config_file = proj_dir.config_dir().join("config.json");
            let mut file = File::create(config_file).await.unwrap();
            file.write_all(serde_json::to_string_pretty(self).unwrap().as_bytes())
                .await
                .unwrap();
        } else {
            panic!("Can't find config path.");
        }
    }

    pub async fn invoke(&self, provider: &mut DynSource) {
        provider.set_fetch_mode(self.fetch_mode);
        provider.set_expired(self.expired_days).await;
    }
}
