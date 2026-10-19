use std::{io, path::Path};

use actix_files::NamedFile;
use actix_web::{get, web};

use crate::server::{state::AppState, utils::get_image_disposition};

#[get("/cover/{filename}")]
pub async fn image(data: web::Data<AppState>, path: web::Path<String>) -> io::Result<NamedFile> {
    let filename = path.into_inner();

    let data = data.clone();
    let data = data.lock().map_err(|_e| io::ErrorKind::Other)?;

    let cover_root = data.config.cover_root.clone();

    Ok(NamedFile::open(Path::new(&cover_root).join(&filename))?
        .set_content_disposition(get_image_disposition(&filename)))
}

#[get("/thumbnail/{filename}")]
pub async fn thumbnail(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> io::Result<NamedFile> {
    let filename = path.into_inner();

    let data = data.clone();
    let data = data.lock().map_err(|_e| io::ErrorKind::Other)?;

    let thumbnail_root = data.config.thumbnail_root.clone();

    Ok(NamedFile::open(Path::new(&thumbnail_root).join(&filename))?
        .set_content_disposition(get_image_disposition(&filename)))
}
