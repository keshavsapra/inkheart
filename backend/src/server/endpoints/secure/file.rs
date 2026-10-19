use std::{io, path::Path};

use actix_files::NamedFile;
use actix_web::{get, web};

use crate::server::{auth::FirebaseUser, state::AppState, utils::get_pdf_disposition};

#[get("/api/file/{id}")]
pub async fn file(
    data: web::Data<AppState>,
    path: web::Path<String>,
    _user: FirebaseUser,
) -> io::Result<NamedFile> {
    let id = path.into_inner();

    let data = data.clone();
    let mut data = data.lock().map_err(|_e| io::ErrorKind::Other)?;

    let record = data
        .lookup_table
        .get_map()
        .get(&id)
        .ok_or(io::ErrorKind::NotFound)?;

    Ok(NamedFile::open(Path::new(&record.file_path))?
        .set_content_disposition(get_pdf_disposition(&record.filename)))
}
