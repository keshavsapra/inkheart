mod config;
mod indexer;
mod pdfium;
mod server;
mod user_settings;

use std::{fs, path::Path};

use config::Config;
use firebase_admin_auth_rs::jwk_auth::JwkAuth;

use indexer::{start_scheduled_indexer, text_search, LookupMap};
use server::{start_server, state::AppData};

use crate::{pdfium::bindings::init_pdfium, server::state::AppState};

/* import version data from build script artifact */
/* literally just testing cache hit or not */
include!(concat!(env!("OUT_DIR"), "/version.rs"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Inkheart v{} ({})", VERSION, CODENAME);

    let config = Config::load_config();
    let pdfium = init_pdfium();

    // Ensure required storage directories exist
    let _ = fs::create_dir_all(&config.cover_root);
    let _ = fs::create_dir_all(&config.thumbnail_root);
    let _ = fs::create_dir_all(&config.config_dir);

    let auth = match config.firebase_project_id.clone() {
        Some(project_id) => Some(JwkAuth::new(project_id).await),
        None => None,
    };

    let state = AppState::new(AppData {
        lookup_table: LookupMap::from_dir(&Path::new(&config.media_root.clone())),
        config: config.clone(),
        auth,
        pdfium: pdfium.clone(),
        is_indexer_running: false,
        text_index: None,
        text_index_sender: None,
        is_text_indexer_running: false,
        text_index_total: 0,
        text_index_processed: 0,
    });

    let text_search = text_search::init_text_search(&config, pdfium, &state)?;

    if let Some(text_index) = text_search.text_index {
        let mut data = state.lock().map_err(|_| std::io::ErrorKind::Other)?;
        data.text_index = Some(text_index);
    }

    match start_scheduled_indexer(&state) {
        Ok(_) => (),
        Err(e) => println!("Failed to start scan scheduler: {}", e.to_string()),
    };

    start_server(&state).await
}
