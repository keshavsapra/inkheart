use std::{
    collections::HashMap,
    fs,
    path::Path,
    time::{Duration, UNIX_EPOCH},
};

use serde::Serialize;

use super::utils::{hash_string, sanitize_filename_component};

//implements serialize so it can be sent as json response
#[derive(Clone, Serialize, Debug)]
pub struct ItemRecord {
    pub id: String,
    pub filename: String,
    pub title: String,
    pub cover_name: String,
    pub cover_name_legacy: String,
    pub file_path: String,
    pub is_dir: bool,
    pub dir_previews: Vec<String>,
    pub children: Vec<SafeItemRecord>,
    pub last_modified: u64,
    pub created: u64,
    pub pages: Option<u32>,
}

#[derive(Clone, Serialize, Debug)]
pub struct AncestorRecord {
    pub id: String,
    pub title: String,
}

//implements serialize so it can be sent as json response
#[derive(Clone, Serialize, Debug)]
pub struct SafeItemRecord {
    pub id: String,
    pub filename: String,
    pub title: String,
    pub cover_name: String,
    pub is_dir: bool,
    pub dir_previews: Vec<String>,
    pub children: Vec<SafeItemRecord>,
    pub last_modified: u64,
    pub created: u64,
    pub pages: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestors: Option<Vec<AncestorRecord>>,
}

impl From<ItemRecord> for SafeItemRecord {
    fn from(item: ItemRecord) -> Self {
        let ItemRecord {
            id,
            filename,
            title,
            cover_name,
            cover_name_legacy: _,
            file_path: _,
            is_dir,
            dir_previews,
            children,
            last_modified,
            created,
            pages,
        } = item;

        SafeItemRecord {
            id,
            filename,
            title,
            cover_name,
            is_dir,
            dir_previews,
            children,
            last_modified,
            created,
            pages,
            ancestors: None,
        }
    }
}

impl From<&ItemRecord> for SafeItemRecord {
    fn from(item: &ItemRecord) -> Self {
        let ItemRecord {
            id,
            filename,
            title,
            cover_name,
            cover_name_legacy: _,
            file_path: _,
            is_dir,
            dir_previews,
            children,
            last_modified,
            created,
            pages,
        } = item.clone();

        SafeItemRecord {
            id,
            filename,
            title,
            cover_name,
            is_dir,
            dir_previews,
            children,
            last_modified,
            created,
            pages,
            ancestors: None,
        }
    }
}

impl ItemRecord {
    pub fn from_path(path: &Path) -> ItemRecord {
        let file_path = path.to_string_lossy().to_string();
        let hash = hash_string(&file_path);
        let is_dir = path.is_dir();
        let filename = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or("".to_string());

        let title = if is_dir {
            filename.clone()
        } else {
            path.file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or("".to_string())
        };

        let cover_name_legacy = format!("{}-{}.jpg", hash, title);
        let cover_name = format!("{}-{}.jpg", hash, sanitize_filename_component(&title));

        let (last_modified, created) = {
            let meta = fs::metadata(&file_path);
            match meta {
                Ok(meta) => {
                    let last_modified: u64 = meta
                        .modified()
                        .map(|st| {
                            st.duration_since(UNIX_EPOCH)
                                .unwrap_or(Duration::from_secs(0))
                                .as_secs()
                        })
                        .unwrap_or(0);
                    let created = meta
                        .created()
                        .map(|st| {
                            st.duration_since(UNIX_EPOCH)
                                .unwrap_or(Duration::from_secs(0))
                                .as_secs()
                        })
                        .unwrap_or(0);
                    (last_modified, created)
                }
                Err(_) => (0, 0),
            }
        };

        ItemRecord {
            id: hash,
            filename,
            title,
            cover_name,
            cover_name_legacy,
            file_path,
            is_dir,
            dir_previews: vec![],
            children: vec![],
            last_modified,
            created,
            pages: None,
        }
    }
}

//implements serialize so it can be sent as json response
#[derive(Clone, Serialize)]
pub struct LookupMap(HashMap<String, ItemRecord>);

impl LookupMap {
    pub fn from_dir(dir: &Path) -> LookupMap {
        let mut map: HashMap<String, ItemRecord> = HashMap::new();
        build_map(dir, &mut map);

        LookupMap(map)
    }

    pub fn get_map(&mut self) -> &mut HashMap<String, ItemRecord> {
        &mut self.0
    }

    pub fn get_map_ref(&self) -> &HashMap<String, ItemRecord> {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn ancestors_of(&self, file_path: &str, media_root: &str) -> Vec<AncestorRecord> {
        let root_path = Path::new(media_root);
        let root_key = hash_string(&media_root.to_string());

        let mut chain: Vec<AncestorRecord> = Vec::new();
        let mut current = Path::new(file_path);

        while let Some(parent) = current.parent() {
            // root may differ from the walked path by a trailing slash, so match by path components
            let at_root = parent == root_path;
            let key = if at_root {
                root_key.clone()
            } else {
                hash_string(&parent.to_string_lossy().to_string())
            };

            match self.0.get(&key) {
                Some(record) => {
                    chain.push(AncestorRecord {
                        id: record.id.clone(),
                        title: record.title.clone(),
                    });
                }
                None => break,
            }

            if at_root {
                break;
            }

            current = parent;
        }

        chain.reverse();
        chain
    }
}

fn build_map(dir: &Path, mut map: &mut HashMap<String, ItemRecord>) -> () {
    let mut dir_record = ItemRecord::from_path(dir);
    if dir.is_dir() {
        match fs::read_dir(dir) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_dir() {
                            let record = ItemRecord::from_path(&path);
                            dir_record
                                .children
                                .push(SafeItemRecord::from(record.clone()));
                            build_map(&path, &mut map);
                        } else {
                            if let Some(extension) = path.extension() {
                                let ext = extension.to_ascii_lowercase().to_string_lossy().to_string();
                                if ext == "pdf" || ext == "epub" {
                                    let record = ItemRecord::from_path(&path);
                                    dir_record
                                        .children
                                        .push(SafeItemRecord::from(record.clone()));
                                    map.insert(record.id.clone(), record);
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to read media directory '{:?}': {}", dir, e);
            }
        }
    } else {
        eprintln!(
            "Warning: Media path '{:?}' does not exist or is not a directory",
            dir
        );
    }
    map.insert(dir_record.id.clone(), dir_record);
}
