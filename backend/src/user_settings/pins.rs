use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::indexer::{ItemRecord, LookupMap};

const MAX_DISPLAY_NAME_LENGTH: usize = 100;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PinConfig {
    pub version: u32,
    pub pinned: Vec<PinnedItem>,
    pub missing: Vec<MissingPinnedItem>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PinnedItem {
    pub id: String,
    pub pinned_at: u64,
    pub display_name: Option<String>,
    pub last_seen_path: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MissingPinnedItem {
    pub id: String,
    pub pinned_at: u64,
    pub missing_since: u64,
    pub display_name: Option<String>,
    pub last_seen_path: String,
}

impl Default for PinConfig {
    fn default() -> Self {
        PinConfig {
            version: 1,
            pinned: Vec::new(),
            missing: Vec::new(),
        }
    }
}

fn current_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

impl PinConfig {
    /// Load pin configuration from file
    pub fn load(config_dir: &str) -> Result<Self, String> {
        let pins_file = Path::new(config_dir).join("pins.json");

        if !pins_file.exists() {
            return Ok(PinConfig::default());
        }

        let content = fs::read_to_string(&pins_file)
            .map_err(|e| format!("Failed to read pins.json: {}", e))?;

        let config: PinConfig = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse pins.json: {}", e))?;

        Ok(config)
    }

    /// Save pin configuration to file
    pub fn save(&self, config_dir: &str) -> Result<(), String> {
        let pins_file = Path::new(config_dir).join("pins.json");

        // Ensure config directory exists
        if let Some(parent) = pins_file.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize pins: {}", e))?;

        fs::write(&pins_file, content).map_err(|e| format!("Failed to write pins.json: {}", e))?;

        Ok(())
    }

    /// Add a new pinned folder
    pub fn add_pin(
        &mut self,
        item: &ItemRecord,
        custom_name: Option<String>,
    ) -> Result<(), String> {
        // Validate: Only allow directories
        if !item.is_dir {
            return Err("Only folders can be pinned".to_string());
        }

        // Check if already pinned
        if self.pinned.iter().any(|p| p.id == item.id) {
            return Err("Folder already pinned".to_string());
        }

        // Validate display name length if provided
        if let Some(ref name) = custom_name {
            if name.len() > MAX_DISPLAY_NAME_LENGTH {
                return Err(format!(
                    "Display name too long (max {} characters)",
                    MAX_DISPLAY_NAME_LENGTH
                ));
            }
        }

        let pinned = PinnedItem {
            id: item.id.clone(),
            pinned_at: current_unix_timestamp(),
            display_name: custom_name,
            last_seen_path: item.file_path.clone(),
        };

        self.pinned.push(pinned);
        Ok(())
    }

    /// Remove a pinned folder
    pub fn remove_pin(&mut self, id: &str) {
        self.pinned.retain(|p| p.id != id);
    }

    /// Remove a missing pinned folder
    pub fn remove_missing(&mut self, id: &str) {
        self.missing.retain(|p| p.id != id);
    }

    /// Reconcile pins with current index state
    /// - Move pinned items that no longer exist to missing
    /// - Restore missing items that reappear
    /// - Update last_seen_path for existing items
    pub fn reconcile_with_index(&mut self, lookup: &LookupMap) {
        let current_time = current_unix_timestamp();
        let map = lookup.get_map_ref();

        // Check pinned items
        let mut to_move = Vec::new();
        for (idx, pinned) in self.pinned.iter_mut().enumerate() {
            match map.get(&pinned.id) {
                Some(item) if item.is_dir => {
                    // Item still exists and is still a directory, update path
                    pinned.last_seen_path = item.file_path.clone();
                }
                _ => {
                    // Item missing or no longer a directory
                    to_move.push(idx);
                }
            }
        }

        // Move to missing list (in reverse to maintain indices)
        for idx in to_move.iter().rev() {
            let pinned = self.pinned.remove(*idx);
            self.missing.push(MissingPinnedItem {
                id: pinned.id,
                pinned_at: pinned.pinned_at,
                missing_since: current_time,
                display_name: pinned.display_name,
                last_seen_path: pinned.last_seen_path,
            });
        }

        // Check missing items for restoration
        let mut to_restore = Vec::new();
        for (idx, missing) in self.missing.iter().enumerate() {
            if let Some(item) = map.get(&missing.id) {
                if item.is_dir {
                    to_restore.push((idx, item.file_path.clone()));
                }
            }
        }

        // Restore missing items that reappeared (in reverse to maintain indices)
        for (idx, new_path) in to_restore.iter().rev() {
            let missing = self.missing.remove(*idx);
            self.pinned.push(PinnedItem {
                id: missing.id,
                pinned_at: missing.pinned_at,
                display_name: missing.display_name,
                last_seen_path: new_path.clone(),
            });
        }
    }
}
