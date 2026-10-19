use std::io;

use actix_web::{Responder, get, web};
use serde::{Deserialize, Serialize};

use crate::{
    indexer::{LookupMap, SafeItemRecord, utils::hash_string},
    server::{auth::FirebaseUser, state::AppState},
};

#[derive(Deserialize)]
struct SearchQuery {
    q: Option<String>,
    scope: Option<String>,
}

fn normalize_search_term(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect()
}

fn resolve_scope_dir(scope_id: &Option<String>, lookup: &LookupMap) -> Option<String> {
    let id = scope_id.as_ref()?;
    let item = lookup.get_map_ref().get(id)?;

    if !item.is_dir {
        return None;
    }

    Some(item.file_path.clone())
}

#[get("/api/search")]
pub async fn search(
    data: web::Data<AppState>,
    _user: FirebaseUser,
    query: web::Query<SearchQuery>,
) -> io::Result<impl Responder> {
    let config = data.get_config()?;
    let root_id = hash_string(&config.media_root);

    let data = data.clone();
    let data = data.lock().map_err(|_e| io::ErrorKind::Other)?;

    let map = data.lookup_table.get_map_ref();
    let normalized_query = query.q.as_deref().map(normalize_search_term);
    let scope_prefix =
        resolve_scope_dir(&query.scope, &data.lookup_table).map(|dir| format!("{}/", dir));

    let safe_vec: Vec<SafeItemRecord> = map
        .iter()
        .filter(|(id, _)| *id != &root_id)
        .filter(|(_, record)| {
            if let Some(ref prefix) = scope_prefix {
                if !record.file_path.starts_with(prefix.as_str()) {
                    return false;
                }
            }

            if let Some(ref q) = normalized_query {
                if q.is_empty() {
                    return true;
                }

                let normalized_title = normalize_search_term(&record.title);
                normalized_title.contains(q.as_str())
            } else {
                true
            }
        })
        .map(|(_, record)| {
            let mut safe: SafeItemRecord = record.into();
            safe.children.clear();
            safe
        })
        .collect();

    Ok(web::Json(safe_vec))
}

#[derive(Clone, Serialize, Debug)]
struct FullTextSearchResponse {
    doc_id: String,
    page_num: u64,
    title: String,
    cover_name: String,
    is_dir: bool,
    match_count: usize,
    pages: Option<u32>,
}

#[get("/api/search/fulltext")]
pub async fn search_fulltext(
    data: web::Data<AppState>,
    _user: FirebaseUser,
    query: web::Query<SearchQuery>,
) -> io::Result<impl Responder> {
    let q = match query.q.as_deref() {
        Some(q) if !q.trim().is_empty() => q,
        _ => return Ok(web::Json(Vec::<FullTextSearchResponse>::new())),
    };

    let text_index = data.get_text_index()?;
    let lookup = data.get_lookup()?;
    let scope = resolve_scope_dir(&query.scope, &lookup);

    let results = match text_index {
        Some(index) => {
            let raw = index.search(q, 50, scope.as_deref()).map_err(|e| {
                eprintln!("Fulltext search error: {}", e);
                io::Error::new(io::ErrorKind::Other, e)
            })?;

            let map = lookup.get_map_ref();

            raw.into_iter()
                .map(|r| {
                    let pages = map.get(&r.doc_id).and_then(|item| item.pages);

                    FullTextSearchResponse {
                        doc_id: r.doc_id,
                        page_num: r.page_num,
                        title: r.title,
                        cover_name: r.cover_name,
                        is_dir: false,
                        match_count: r.match_count,
                        pages,
                    }
                })
                .collect()
        }
        None => Vec::new(),
    };

    Ok(web::Json(results))
}
