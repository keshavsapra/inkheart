use std::{fs, io};

use actix_web::{http::StatusCode, web, HttpResponse, Responder};

use crate::server::{
    state::AppState,
    utils::{escape_html_attribute, hydrate_index_html},
};

pub async fn index(data: web::Data<AppState>) -> io::Result<impl Responder> {
    let html = fs::read_to_string("./public/index.html")?;
    let config = data.get_config()?;

    let title = "Inkheart".to_string();
    let img = "/static/inkheart.png".to_string();

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
