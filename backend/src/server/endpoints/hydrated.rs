use std::{
    cmp,
    fs::{self},
    io,
    path::Path,
};

use actix_web::{HttpResponse, Responder, http::StatusCode, routes, web};
use serde::Deserialize;

use crate::{
    indexer::{LookupMap, extract_pdf_thumbnail},
    server::{
        state::AppState,
        utils::{escape_html_attribute, hydrate_index_html},
    },
};

#[derive(Deserialize)]
pub struct Info {
    preview: Option<String>,
    page: Option<i32>,
}

// TODO: add setting to make this a secure route
// this route deliberately leaks the title and thumbnail paths
// in order for OpenGraph previews to work
// future improvment would use a setting to block this behaviour
#[routes]
#[get("/reader/{id}")]
#[get("/fullscreen/{id}")]
#[get("/embed/{id}")]
pub async fn multihandler(
    data: web::Data<AppState>,
    path: web::Path<String>,
    info: web::Query<Info>,
) -> io::Result<impl Responder> {
    let id = path.into_inner();
    let html = fs::read_to_string("./public/index.html")?;

    let config = data.get_config()?;
    let pdfium = data.get_pdfium()?;

    let mut lookup = None;
    if let Ok(data) = data.lock() {
        let lookup_table = LookupMap::from_dir(Path::new(&data.config.media_root));
        lookup = Some(lookup_table.clone());

        drop(data);
    }

    let mut title = String::new();
    let mut img = String::new();

    if let Some(mut lookup) = lookup {
        let record = lookup.get_map().get(&id).ok_or(io::ErrorKind::NotFound)?;
        title = record.title.clone();
        let page = info.page.map(|n| cmp::max(n - 1, 0)).unwrap_or(0);

        let is_epub = record.file_path.to_ascii_lowercase().ends_with(".epub");

        if is_epub {
            img = format!("/cover/{}", record.cover_name);
        } else {
            let filename = extract_pdf_thumbnail(
                &config.thumbnail_root.clone(),
                &id,
                page,
                &record.file_path,
                pdfium,
                config.thumbnail_width as i32,
            )
            .await
            .ok();

            img = match filename {
                Some(f) => format!("/thumbnail/{}", f),
                None => "/static/inkheart.png".to_string(),
            };
        }
    }

    if let Some(preview) = info.preview.clone() {
        title = preview
    }

    let use_auth = match config.firebase_project_id.is_some() {
        true => "true",
        false => "false",
    };

    let firebase_config = config
        .get_firebase_config()
        .unwrap_or_else(|| "null".to_string());

    let escaped_firebase_config = escape_html_attribute(&firebase_config);

    let basepath = config.basepath.clone().unwrap_or_else(|| "".to_string());

    let hydrated = hydrate_index_html(
        &html,
        &title,
        &img,
        use_auth,
        &escaped_firebase_config,
        &basepath,
    );

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .insert_header(("X-Clacks-Overhead", config.clacks_overhead_msg.clone()))
        .body(hydrated))
}
