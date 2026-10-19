use actix_web::{web, App, HttpServer};

use crate::server::endpoints::{
    add_item_to_collection, add_pin, create_collection, delete_collection, file, get_collections,
    get_pins, get_user_settings, health_check, image, index, indexer_start, indexer_status, item,
    mediaroot, multihandler, remove_item_from_collection, remove_missing_pin, remove_pin,
    rename_collection, search, search_fulltext, thumbnail, update_user_settings,
};

use super::state::AppState;

pub async fn start_server(state: &AppState) -> std::io::Result<()> {
    println!("Server started");

    let config = state.get_config()?;

    let state = state.clone();

    let basepath = config.basepath.clone().unwrap_or_else(|| "".to_string());

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(state.clone())).service(
            web::scope(&basepath)
                .service(health_check)
                .service(indexer_start)
                .service(indexer_status)
                .service(mediaroot)
                .service(item)
                .service(file)
                .service(search)
                .service(search_fulltext)
                .service(get_pins)
                .service(add_pin)
                .service(remove_pin)
                .service(remove_missing_pin)
                .service(get_collections)
                .service(create_collection)
                .service(rename_collection)
                .service(delete_collection)
                .service(add_item_to_collection)
                .service(remove_item_from_collection)
                .service(image)
                .service(thumbnail)
                .service(multihandler)
                .service(get_user_settings)
                .service(update_user_settings)
                .service(
                    actix_files::Files::new("/static", "./public/static")
                        .default_handler(web::get().to(index)),
                )
                .route("/", web::get().to(index))
                .default_service(web::get().to(index)),
        )
    })
    .bind((config.bind_addr.clone(), config.bind_port.clone()))?
    .run()
    .await
}
