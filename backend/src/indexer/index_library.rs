use std::{collections::HashMap, path::Path, sync::Arc};

use crate::{
    indexer::{
        LookupMap,
        epub_extract::{count_epub_chapters, extract_epub_cover},
        pdf_extract::{count_pdf_pages, extract_pdf_cover},
        text_search::queue_text_indexing,
    },
    pdfium::bindings::BindingsWrapper,
    server::state::AppState,
    user_settings::PinConfig,
};

struct IndexerStatusGuard(AppState);

impl Drop for IndexerStatusGuard {
    fn drop(&mut self) {
        let _ = self.0.set_indexer_status(false);
    }
}

pub fn scan_library(state: AppState) -> Result<(), String> {
    {
        let mut data = state
            .lock()
            .map_err(|e| format!("Failed to lock state: {}", e))?;

        if data.is_indexer_running {
            println!("Indexer already running; skipping this scan.");
            return Ok(());
        }

        data.is_indexer_running = true;
    }

    let guard = IndexerStatusGuard(state.clone());

    println!("Running indexer...");

    let (mut lookup, cover_root, pdfium, cover_width) = update_lookup_table(&state)?;
    let config_dir = state.get_config_dir().map_err(|e| e.to_string())?;

    reconcile_pins(&config_dir, &lookup);
    extract_covers(&mut lookup, &cover_root, &pdfium, cover_width);
    fill_page_counts(&mut lookup, &pdfium);
    queue_text_indexing(&state, &lookup, &config_dir);

    if let Err(e) = state.set_lookup(lookup) {
        eprintln!("Failed to save lookup table: {}", e);
    }

    drop(guard);
    println!("Indexer finished.");

    Ok(())
}

fn update_lookup_table(
    state: &AppState,
) -> Result<(LookupMap, String, Arc<BindingsWrapper>, i32), String> {
    let mut data = state
        .lock()
        .map_err(|e| format!("Failed to acquire lock: {}", e))?;

    println!("Update lookup table...");

    let lookup_table = LookupMap::from_dir(Path::new(&data.config.media_root));
    let cover_root = data.config.cover_root.clone();
    let cover_width = data.config.cover_width as i32;
    let pdfium = data.pdfium.clone();

    data.lookup_table = lookup_table.clone();

    drop(data);

    println!("Lookup table updated...");

    Ok((lookup_table, cover_root, pdfium, cover_width))
}

fn reconcile_pins(config_dir: &str, lookup: &LookupMap) {
    println!("Reconciling pinned folders...");

    match PinConfig::load(config_dir) {
        Ok(mut pins) => {
            pins.reconcile_with_index(lookup);

            if let Err(e) = pins.save(config_dir) {
                eprintln!("Failed to save pins after reconciliation: {}", e);
            } else {
                println!("Pinned folders reconciled");
            }
        }
        Err(e) => {
            eprintln!("Failed to load pins for reconciliation: {}", e);
        }
    }
}

fn extract_covers(
    lookup: &mut LookupMap,
    cover_root: &str,
    pdfium: &Arc<BindingsWrapper>,
    cover_width: i32,
) {
    let mut failed_docs = Vec::new();
    let mut success_count = 0;
    let total_items = lookup.get_map().len();

    for (_, item) in lookup.get_map() {
        if !item.is_dir {
            let is_epub = item.file_path.to_ascii_lowercase().ends_with(".epub");
            let extract_result = if is_epub {
                extract_epub_cover(
                    &cover_root.to_string(),
                    &item.cover_name,
                    &item.file_path,
                    Some(&item.cover_name_legacy),
                    cover_width,
                )
            } else {
                extract_pdf_cover(
                    &cover_root.to_string(),
                    &item.cover_name,
                    &item.file_path,
                    pdfium.clone(),
                    Some(&item.cover_name_legacy),
                    cover_width,
                )
            };

            match extract_result {
                Ok(_) => {
                    success_count += 1;

                    if success_count % 10 == 0 {
                        println!(
                            "Progress: {}/{} covers extracted",
                            success_count, total_items
                        );
                    }
                }
                Err(e) => {
                    eprintln!("Failed to extract cover from '{}': {}", item.file_path, e);
                    failed_docs.push((item.file_path.clone(), e));
                }
            }
        }
    }

    println!("\n=== Indexer Results ===");
    println!("Successfully processed: {} documents", success_count);
    println!("Failed: {} documents", failed_docs.len());

    if !failed_docs.is_empty() {
        println!("\nFailed documents:");

        for (path, error) in &failed_docs {
            println!("  - {}: {}", path, error);
        }
    }
}

// Get page counts for basic metadata.
fn fill_page_counts(lookup: &mut LookupMap, pdfium: &Arc<BindingsWrapper>) {
    let mut counts: HashMap<String, u32> = HashMap::new();

    // first pass: count pages for all non-directory items
    for (id, item) in lookup.get_map().iter_mut() {
        if item.is_dir {
            continue;
        }

        let file_path = item.file_path.clone();
        let is_epub = file_path.to_ascii_lowercase().ends_with(".epub");

        let pages_result = if is_epub {
            count_epub_chapters(&file_path)
        } else {
            count_pdf_pages(&file_path, pdfium.clone())
        };

        match pages_result {
            Ok(pages) => {
                item.pages = Some(pages);
                counts.insert(id.clone(), pages);
            }
            Err(e) => {
                eprintln!("Failed to count pages for '{}': {}", file_path, e);
            }
        }
    }

    // apply page counts to directory children
    for (_, item) in lookup.get_map().iter_mut() {
        if !item.is_dir {
            continue;
        }

        for child in item.children.iter_mut() {
            if let Some(pages) = counts.get(&child.id) {
                child.pages = Some(*pages);
            }
        }
    }
}
