use std::io;

use actix_web::{get, web, Responder};

use crate::{
    indexer::SafeItemRecord,
    server::{auth::FirebaseUser, state::AppState},
};

#[get("/api/item/{id}")]
pub async fn item(
    data: web::Data<AppState>,
    path: web::Path<String>,
    _user: FirebaseUser,
) -> io::Result<impl Responder> {
    let id = path.into_inner();
    library_path(data, id, _user).await
}

pub async fn library_path(
    data: web::Data<AppState>,
    id: String,
    _user: FirebaseUser,
) -> io::Result<impl Responder> {
    let data = data.clone();
    let mut data = data.lock().map_err(|_e| io::ErrorKind::Other)?;

    let record = data
        .lookup_table
        .get_map()
        .get(&id)
        .ok_or(io::ErrorKind::NotFound)?
        .clone();

    let media_root = data.config.media_root.clone();
    let ancestors = data.lookup_table.ancestors_of(&record.file_path, &media_root);

    let mut safe = SafeItemRecord::from(record);
    safe.ancestors = Some(ancestors);

    Ok(web::Json(safe))
}
