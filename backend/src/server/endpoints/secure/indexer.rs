use std::io;

use actix_web::{get, post, web, Responder};
use serde::Serialize;

use crate::{
    indexer::scan_library,
    server::{auth::FirebaseUser, state::AppState},
};

#[derive(Clone, Serialize, Debug)]
struct IndexerStatus {
    running: bool,
    text_indexing: bool,
    text_index_total: usize,
    text_index_processed: usize,
}

#[post("/api/indexer/start")]
pub async fn indexer_start(
    data: web::Data<AppState>,
    _user: FirebaseUser,
) -> io::Result<impl Responder> {
    let state = data.get_ref().clone();
    actix_web::rt::spawn(async move {
        if let Err(e) = scan_library(state) {
            eprintln!("Indexer task failed: {}", e);
        }
    });

    Ok(actix_web::HttpResponse::Accepted().finish())
}

#[get("/api/indexer/status")]
pub async fn indexer_status(
    data: web::Data<AppState>,
    _user: FirebaseUser,
) -> io::Result<impl Responder> {
    let status = data.get_indexer_status()?;
    let text_status = data.get_text_indexer_status()?;
    let (text_total, text_processed) = data.get_text_index_progress()?;
    let response = IndexerStatus {
        running: status,
        text_indexing: text_status,
        text_index_total: text_total,
        text_index_processed: text_processed,
    };

    Ok(web::Json(response))
}
