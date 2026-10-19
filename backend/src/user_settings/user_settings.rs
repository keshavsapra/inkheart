use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub telemetry_enabled: bool,
    pub telemetry_banner_shown: bool,
}

impl Default for UserSettings {
    fn default() -> Self {
        UserSettings {
            telemetry_enabled: true,
            telemetry_banner_shown: false,
        }
    }
}

impl UserSettings {
    pub fn load(config_dir: &str) -> io::Result<Self> {
        let path = Path::new(config_dir).join("user_settings.json");

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path)?;
        let settings: UserSettings =
            serde_json::from_str(&content).unwrap_or_else(|_| Self::default());

        Ok(settings)
    }

    pub fn save(&self, config_dir: &str) -> io::Result<()> {
        let path = Path::new(config_dir).join("user_settings.json");
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
}
