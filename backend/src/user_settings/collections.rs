use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::indexer::{utils::hash_string, ItemRecord, LookupMap, SafeItemRecord};

const MAX_NAME_LENGTH: usize = 100;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CollectionsConfig {
    pub version: u32,
    pub collections: Vec<Collection>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub items: Vec<CollectionItem>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CollectionItem {
    pub id: String,
    pub added_at: u64,
    pub last_seen_path: String,
}

#[derive(Clone, Serialize, Debug)]
pub struct EnrichedCollection {
    pub id: String,
    pub name: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub items: Vec<CollectionItemWithDetails>,
}

#[derive(Clone, Serialize, Debug)]
pub struct CollectionItemWithDetails {
    pub id: String,
    pub added_at: u64,
    pub last_seen_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_item: Option<SafeItemRecord>,
}

impl Default for CollectionsConfig {
    fn default() -> Self {
        CollectionsConfig {
            version: 1,
            collections: Vec::new(),
        }
    }
}

fn current_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

impl CollectionsConfig {
    /// Load collections configuration from file
    pub fn load(config_dir: &str) -> Result<Self, String> {
        let collections_file = Path::new(config_dir).join("collections.json");

        if !collections_file.exists() {
            return Ok(CollectionsConfig::default());
        }

        let content = fs::read_to_string(&collections_file)
            .map_err(|e| format!("Failed to read collections.json: {}", e))?;

        let config: CollectionsConfig = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse collections.json: {}", e))?;

        Ok(config)
    }

    /// Save collections configuration to file
    pub fn save(&self, config_dir: &str) -> Result<(), String> {
        let collections_file = Path::new(config_dir).join("collections.json");

        // Ensure config directory exists
        if let Some(parent) = collections_file.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize collections: {}", e))?;

        fs::write(&collections_file, content)
            .map_err(|e| format!("Failed to write collections.json: {}", e))?;

        Ok(())
    }

    /// Create a new collection
    pub fn create_collection(&mut self, name: String) -> Result<String, String> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err("Collection name cannot be empty".to_string());
        }
        if trimmed.len() > MAX_NAME_LENGTH {
            return Err(format!(
                "Collection name too long (max {} characters)",
                MAX_NAME_LENGTH
            ));
        }

        let now = current_unix_timestamp();
        // Generate unique ID by hashing name + timestamp
        let id = hash_string(&format!("{}_{}", name, now));

        let collection = Collection {
            id: id.clone(),
            name,
            created_at: now,
            updated_at: now,
            items: Vec::new(),
        };

        self.collections.push(collection);
        Ok(id)
    }

    /// Rename a collection
    pub fn rename_collection(&mut self, id: &str, name: String) -> Result<(), String> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err("Collection name cannot be empty".to_string());
        }
        if trimmed.len() > MAX_NAME_LENGTH {
            return Err(format!(
                "Collection name too long (max {} characters)",
                MAX_NAME_LENGTH
            ));
        }

        let collection = self
            .collections
            .iter_mut()
            .find(|c| c.id == id)
            .ok_or_else(|| "Collection not found".to_string())?;

        collection.name = name;
        collection.updated_at = current_unix_timestamp();

        Ok(())
    }

    /// Delete a collection
    pub fn delete_collection(&mut self, id: &str) -> Result<(), String> {
        let initial_len = self.collections.len();
        self.collections.retain(|c| c.id != id);

        if self.collections.len() == initial_len {
            return Err("Collection not found".to_string());
        }

        Ok(())
    }

    /// Add an item to a collection
    pub fn add_item(&mut self, collection_id: &str, item: &ItemRecord) -> Result<(), String> {
        // Validate: Only allow files (not directories)
        if item.is_dir {
            return Err("Only files can be added to collections".to_string());
        }

        let collection = self
            .collections
            .iter_mut()
            .find(|c| c.id == collection_id)
            .ok_or_else(|| "Collection not found".to_string())?;

        // Check if item already exists in collection
        if collection.items.iter().any(|i| i.id == item.id) {
            return Err("Item already in collection".to_string());
        }

        let collection_item = CollectionItem {
            id: item.id.clone(),
            added_at: current_unix_timestamp(),
            last_seen_path: item.file_path.clone(),
        };

        collection.items.push(collection_item);
        collection.updated_at = current_unix_timestamp();

        Ok(())
    }

    /// Remove an item from a collection
    pub fn remove_item(&mut self, collection_id: &str, item_id: &str) -> Result<(), String> {
        let collection = self
            .collections
            .iter_mut()
            .find(|c| c.id == collection_id)
            .ok_or_else(|| "Collection not found".to_string())?;

        let initial_len = collection.items.len();
        collection.items.retain(|i| i.id != item_id);

        if collection.items.len() == initial_len {
            return Err("Item not found in collection".to_string());
        }

        collection.updated_at = current_unix_timestamp();

        Ok(())
    }

    /// Enrich collections with current item data from lookup table
    pub fn enrich_with_lookup(&self, lookup: &LookupMap) -> Vec<EnrichedCollection> {
        let map = lookup.get_map_ref();

        self.collections
            .iter()
            .map(|collection| {
                let items = collection
                    .items
                    .iter()
                    .map(|item| {
                        let current_item = map
                            .get(&item.id)
                            .filter(|record| !record.is_dir) // Only files
                            .map(|record| SafeItemRecord::from(record));

                        CollectionItemWithDetails {
                            id: item.id.clone(),
                            added_at: item.added_at,
                            last_seen_path: item.last_seen_path.clone(),
                            current_item,
                        }
                    })
                    .collect();

                EnrichedCollection {
                    id: collection.id.clone(),
                    name: collection.name.clone(),
                    created_at: collection.created_at,
                    updated_at: collection.updated_at,
                    items,
                }
            })
            .collect()
    }
}
