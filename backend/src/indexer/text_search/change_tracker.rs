use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ChangeTracker {
    entries: HashMap<String, u64>,
    #[serde(skip)]
    file_path: PathBuf,
}

impl ChangeTracker {
    pub fn load(config_dir: &Path) -> Self {
        let file_path = config_dir.join("text_index_tracker.json");

        let entries = fs::read_to_string(&file_path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default();

        ChangeTracker { entries, file_path }
    }

    pub fn delete(config_dir: &Path) -> Result<(), String> {
        let file_path = config_dir.join("text_index_tracker.json");

        match fs::remove_file(&file_path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(format!("Failed to delete change tracker: {}", e)),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self.entries)
            .map_err(|e| format!("Failed to serialize change tracker: {}", e))?;

        fs::write(&self.file_path, json)
            .map_err(|e| format!("Failed to save change tracker: {}", e))?;

        Ok(())
    }

    pub fn needs_update(&self, doc_id: &str, last_modified: u64) -> bool {
        match self.entries.get(doc_id) {
            Some(&tracked_modified) => tracked_modified != last_modified,
            None => true,
        }
    }

    pub fn mark_indexed(&mut self, doc_id: String, last_modified: u64) {
        self.entries.insert(doc_id, last_modified);
    }

    pub fn remove(&mut self, doc_id: &str) {
        self.entries.remove(doc_id);
    }

    pub fn tracked_ids(&self) -> Vec<String> {
        self.entries.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn needs_update_unknown_doc() {
        let tmp = TempDir::new().unwrap();
        let tracker = ChangeTracker::load(tmp.path());

        assert!(tracker.needs_update("unknown_id", 12345));
    }

    #[test]
    fn needs_update_modified_changed() {
        let tmp = TempDir::new().unwrap();
        let mut tracker = ChangeTracker::load(tmp.path());

        tracker.mark_indexed("doc1".to_string(), 100);

        assert!(tracker.needs_update("doc1", 200));
    }

    #[test]
    fn needs_update_modified_unchanged() {
        let tmp = TempDir::new().unwrap();
        let mut tracker = ChangeTracker::load(tmp.path());

        tracker.mark_indexed("doc1".to_string(), 100);

        assert!(!tracker.needs_update("doc1", 100));
    }

    #[test]
    fn remove_makes_doc_unknown() {
        let tmp = TempDir::new().unwrap();
        let mut tracker = ChangeTracker::load(tmp.path());

        tracker.mark_indexed("doc1".to_string(), 100);
        assert!(!tracker.needs_update("doc1", 100));

        tracker.remove("doc1");
        assert!(tracker.needs_update("doc1", 100));
    }

    #[test]
    fn load_nonexistent_returns_empty() {
        let tmp = TempDir::new().unwrap();
        let tracker = ChangeTracker::load(tmp.path());

        assert!(tracker.tracked_ids().is_empty());
    }

    #[test]
    fn save_then_load_roundtrips() {
        let tmp = TempDir::new().unwrap();

        {
            let mut tracker = ChangeTracker::load(tmp.path());
            tracker.mark_indexed("doc1".to_string(), 100);
            tracker.mark_indexed("doc2".to_string(), 200);
            tracker.save().unwrap();
        }

        let tracker = ChangeTracker::load(tmp.path());

        assert!(!tracker.needs_update("doc1", 100));
        assert!(!tracker.needs_update("doc2", 200));
        assert!(tracker.needs_update("doc1", 999));
        assert!(tracker.needs_update("doc3", 300));
    }

    #[test]
    fn delete_removes_file_and_ignores_missing() {
        let tmp = TempDir::new().unwrap();

        ChangeTracker::delete(tmp.path()).unwrap();

        let mut tracker = ChangeTracker::load(tmp.path());
        tracker.mark_indexed("doc1".to_string(), 100);
        tracker.save().unwrap();

        assert!(tmp.path().join("text_index_tracker.json").exists());

        ChangeTracker::delete(tmp.path()).unwrap();

        assert!(!tmp.path().join("text_index_tracker.json").exists());
        assert!(ChangeTracker::load(tmp.path()).tracked_ids().is_empty());
    }
}
