use std::{fs, path::Path, sync::Arc};

use crate::{config::Config, pdfium::bindings::BindingsWrapper, server::state::AppState};

use super::{start_text_indexer, ChangeTracker, TextSearchIndex};

pub struct TextSearchComponents {
    pub text_index: Option<Arc<TextSearchIndex>>,
}

pub fn init_text_search(
    config: &Config,
    pdfium: Arc<BindingsWrapper>,
    state: &AppState,
) -> std::io::Result<TextSearchComponents> {
    if !config.text_index_enabled {
        println!("Text indexing disabled by configuration");
        return Ok(TextSearchComponents { text_index: None });
    }

    let config_dir = Path::new(&config.config_dir);
    let index_dir = config_dir.join("text_index");

    let text_index = match TextSearchIndex::open_or_create(&index_dir) {
        Ok(idx) => {
            println!("Text search index opened at {:?}", index_dir);
            Some(Arc::new(idx))
        }
        Err(e) => {
            eprintln!(
                "Failed to open text search index: {}. Wiping index and rebuilding.",
                e
            );

            if let Err(e) = fs::remove_dir_all(&index_dir) {
                eprintln!("Failed to remove text index directory: {}", e);
            }

            if let Err(e) = ChangeTracker::delete(config_dir) {
                eprintln!("{}", e);
            }

            match TextSearchIndex::open_or_create(&index_dir) {
                Ok(idx) => {
                    println!("Text search index rebuilt at {:?}", index_dir);
                    Some(Arc::new(idx))
                }
                Err(e) => {
                    eprintln!(
                        "Failed to open text search index after wipe: {}. Full-text search will be unavailable.",
                        e
                    );
                    None
                }
            }
        }
    };

    // Only start the worker thread if the index opened successfully.
    // The worker opens its own write handle to the same index directory;
    // Tantivy supports concurrent reader/writer handles.
    if text_index.is_some() {
        let sender = start_text_indexer(index_dir, config_dir.to_path_buf(), pdfium, state.clone());
        state.set_text_index_sender(sender)?;
    }

    Ok(TextSearchComponents { text_index })
}
