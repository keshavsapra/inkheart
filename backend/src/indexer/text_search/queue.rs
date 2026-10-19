use std::{collections::HashSet, path::Path, sync::mpsc};

use crate::{indexer::LookupMap, server::state::AppState};

use super::{ChangeTracker, TextIndexJob};

pub fn queue_text_indexing(state: &AppState, lookup: &LookupMap, config_dir: &str) {
    if let Ok(true) = state.get_text_indexer_status() {
        println!("Text indexer already running, skipping queue");
        return;
    }

    let sender = match state.get_text_index_sender() {
        Ok(Some(sender)) => sender,
        _ => return,
    };

    let _ = state.set_text_indexer_status(true);

    let config_dir_path = Path::new(config_dir);
    let mut tracker = ChangeTracker::load(config_dir_path);

    let current_ids: HashSet<String> = lookup
        .get_map_ref()
        .iter()
        .filter(|(_, item)| !item.is_dir)
        .map(|(id, _)| id.clone())
        .collect();

    let removed_count = remove_deleted_docs(&sender, &mut tracker, &current_ids, state);
    let (queued_count, send_failed) = queue_changed_docs(&sender, &mut tracker, lookup);

    let _ = state.set_text_index_progress(queued_count, 0);

    if !send_failed {
        if sender.send(TextIndexJob::Flush).is_err() {
            eprintln!("Text indexer worker disconnected, failed to flush");
            let _ = state.set_text_indexer_status(false);
        }
    }

    println!(
        "Text indexing: queued {} new/changed, {} removed",
        queued_count, removed_count
    );
}

fn remove_deleted_docs(
    sender: &mpsc::Sender<TextIndexJob>,
    tracker: &mut ChangeTracker,
    current_ids: &HashSet<String>,
    state: &AppState,
) -> usize {
    let tracked = tracker.tracked_ids();
    let mut removed_count = 0;

    for tracked_id in &tracked {
        if !current_ids.contains(tracked_id) {
            if sender
                .send(TextIndexJob::RemoveDoc {
                    doc_id: tracked_id.clone(),
                })
                .is_err()
            {
                eprintln!("Text indexer worker disconnected, skipping text indexing");
                let _ = state.set_text_indexer_status(false);
                break;
            }

            tracker.remove(tracked_id);
            removed_count += 1;
        }
    }

    removed_count
}

fn queue_changed_docs(
    sender: &mpsc::Sender<TextIndexJob>,
    tracker: &mut ChangeTracker,
    lookup: &LookupMap,
) -> (usize, bool) {
    let mut queued_count = 0;

    for (id, item) in lookup.get_map_ref() {
        if !item.is_dir && tracker.needs_update(id, item.last_modified) {
            if sender
                .send(TextIndexJob::IndexPdf {
                    doc_id: id.clone(),
                    title: item.title.clone(),
                    cover_name: item.cover_name.clone(),
                    file_path: item.file_path.clone(),
                    last_modified: item.last_modified,
                })
                .is_err()
            {
                eprintln!("Text indexer worker disconnected, skipping text indexing");
                return (queued_count, true);
            }

            queued_count += 1;
        }
    }

    (queued_count, false)
}
