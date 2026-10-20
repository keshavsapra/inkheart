use config_file::{Config as ConfigFile, ConfigError, File, Source};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs, path::Path};

const CONFIG_FILE_PATH: &str = "./inkheart.toml";
const DEFAULT_FIREBASE_CONFIG_PATH: &str = "./firebase.json";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub media_root: String,
    pub cover_root: String,
    pub thumbnail_root: String,
    pub config_path: String,
    pub config_dir: String,
    pub bind_addr: String,
    pub bind_port: u16,
    pub scan_interval: u64,
    pub cover_width: u32,
    pub thumbnail_width: u32,
    pub firebase_config_path: String,
    pub firebase_project_id: Option<String>,
    pub whitelist: Option<String>,
    pub clacks_overhead_msg: String,
    pub basepath: Option<String>,
    pub text_index_enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigToml {
    #[serde(alias = "MEDIA_DIR")]
    pub media_root: Option<String>,
    #[serde(alias = "IMAGE_DIR")]
    pub cover_root: Option<String>,
    #[serde(alias = "THUMBNAIL_DIR")]
    pub thumbnail_root: Option<String>,
    #[serde(alias = "CONFIG_PATH")]
    pub config_path: Option<String>,
    #[serde(alias = "CONFIG_DIR")]
    pub config_dir: Option<String>,
    #[serde(alias = "BIND_ADDR")]
    pub bind_addr: Option<String>,
    #[serde(alias = "BIND_PORT")]
    pub bind_port: Option<u16>,
    #[serde(alias = "COVER_WIDTH")]
    pub cover_width: Option<u32>,
    #[serde(alias = "THUMBNAIL_WIDTH")]
    pub thumbnail_width: Option<u32>,
    #[serde(alias = "SCAN_INTERVAL")]
    pub scan_interval: Option<u64>,
    #[serde(alias = "FIREBASE_CONFIG_PATH")]
    pub firebase_config_path: Option<String>,
    #[serde(alias = "FIREBASE_WHITELIST")]
    pub whitelist: Option<String>,
    #[serde(alias = "CLACKS_OVERHEAD")]
    pub clacks_overhead_msg: Option<String>,
    #[serde(alias = "BASE_PATH")]
    pub basepath: Option<String>,
    #[serde(alias = "TEXT_INDEX_ENABLED")]
    pub text_index_enabled: Option<bool>,
}

impl Source for Config {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new(self.clone())
    }

    fn collect(&self) -> Result<config_file::Map<String, config_file::Value>, ConfigError> {
        let s = serde_json::to_string(self).map_err(|e| ConfigError::Message(e.to_string()))?;
        let map: HashMap<String, config_file::Value> =
            serde_json::from_str(&s).map_err(|e| ConfigError::Message(e.to_string()))?;

        Ok(map)
    }
}

impl Config {
    pub fn from_env() -> Config {
        println!("Loading from environment...");

        let config_path = env::var_os("CONFIG_PATH")
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or(CONFIG_FILE_PATH.to_string());
        let media_root = env::var_os("MEDIA_DIR")
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or("./media".to_string());
        let cover_root = env::var_os("IMAGE_DIR")
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or("./covers".to_string());
        let thumbnail_root = env::var_os("THUMBNAIL_DIR")
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or("./thumbnails".to_string());
        let bind_addr = env::var_os("BIND_ADDR")
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or("127.0.0.1".to_string());
        let bind_port = env::var_os("BIND_PORT")
            .and_then(|s| s.to_string_lossy().parse().ok())
            .unwrap_or(8080);
        let scan_interval = env::var_os("SCAN_INTERVAL")
            .and_then(|s| s.to_string_lossy().parse().ok())
            .unwrap_or(600);
        let cover_width = env::var_os("COVER_WIDTH")
            .and_then(|s| s.to_string_lossy().parse().ok())
            .unwrap_or(900);
        let thumbnail_width = env::var_os("THUMBNAIL_WIDTH")
            .and_then(|s| s.to_string_lossy().parse().ok())
            .unwrap_or(300);
        let firebase_config_path = env::var_os("FIREBASE_CONFIG_PATH")
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or(DEFAULT_FIREBASE_CONFIG_PATH.to_string());
        let whitelist = env::var_os("FIREBASE_WHITELIST")
            .map(|s| s.to_string_lossy().into_owned())
            .filter(|s| !s.is_empty());

        let clacks_overhead_msg = env::var_os("CLACKS_OVERHEAD")
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or("GNU Terry Pratchet".to_string());

        let basepath = env::var_os("BASE_PATH")
            .map(|s| s.to_string_lossy().into_owned())
            .filter(|s| !s.is_empty());

        let text_index_enabled = env::var_os("TEXT_INDEX_ENABLED")
            .and_then(|s| s.to_string_lossy().parse().ok())
            .unwrap_or(true);

        let config_dir = env::var_os("CONFIG_DIR")
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| {
                // Backward compatibility: derive from config_path parent directory
                Path::new(&config_path)
                    .parent()
                    .and_then(|p| p.to_str())
                    .unwrap_or(".")
                    .to_string()
            });

        let mut config = Config {
            config_path,
            config_dir,
            media_root,
            cover_root,
            thumbnail_root,
            bind_addr,
            bind_port,
            scan_interval,
            cover_width,
            thumbnail_width,
            firebase_config_path,
            firebase_project_id: None,
            whitelist,
            clacks_overhead_msg,
            basepath,
            text_index_enabled,
        };

        // Try to load Firebase Project ID from config file
        config.load_firebase_project_id();

        config
    }

    pub fn load_config() -> Config {
        println!("Loading Config...");
        let base = Config::from_env();
        if !Path::new(&base.config_path).exists() {
            println!(
                "path not found: {}, falling back to environment",
                CONFIG_FILE_PATH
            );
            return base;
        }

        println!("Loading from {}", &base.config_path);

        // there has to be a better way of doing this but for now this is how we're doing it.
        let conf = (|| {
            // build optional version of config from config file
            let conf = ConfigFile::builder()
                .add_source(File::with_name(&base.config_path).required(false))
                .build()
                .and_then(|c| c.try_deserialize::<ConfigToml>())?;

            // convert struct to HashMap using serde, I'm not sorry
            let s =
                serde_json::to_string(&conf).map_err(|e| ConfigError::Message(e.to_string()))?;
            let override_map: HashMap<String, Option<config_file::Value>> =
                serde_json::from_str(&s).map_err(|e| ConfigError::Message(e.to_string()))?;

            // Build config again using our optional config as overrides.
            // Because if we're being dumb why not go all in?
            let mut merged_builder = ConfigFile::builder().add_source(base.clone());

            for (key, value) in &override_map {
                merged_builder = merged_builder
                    .set_override_option::<&String, config_file::Value>(key, value.clone())?;
            }

            merged_builder
                .build()
                .and_then(|c| c.try_deserialize::<Config>())
        })();

        match conf {
            Ok(mut conf) => {
                // Try to load Firebase Project ID from config file
                conf.load_firebase_project_id();
                println!("{:?}", conf);
                conf
            }
            Err(e) => {
                println!(
                    "config error: {}, falling back to environment",
                    e.to_string()
                );
                base
            }
        }
    }

    pub fn load_firebase_project_id(&mut self) {
        if Path::new(&self.firebase_config_path).exists() {
            if let Ok(content) = fs::read_to_string(&self.firebase_config_path) {
                if let Ok(config) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(project_id) = config.get("projectId").and_then(|v| v.as_str()) {
                        self.firebase_project_id = Some(project_id.to_string());
                    }
                }
            }
        }
    }

    pub fn get_firebase_config(&self) -> Option<String> {
        if Path::new(&self.firebase_config_path).exists() {
            match fs::read_to_string(&self.firebase_config_path) {
                Ok(content) => Some(content),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}
