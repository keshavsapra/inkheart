use actix_web::{delete, get, patch, post, web, Responder};
use serde::{Deserialize, Serialize};
use std::io;

use crate::{
    server::{auth::FirebaseUser, state::AppState},
    user_settings::CollectionsConfig,
};

#[derive(Deserialize)]
pub struct CreateCollectionRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct RenameCollectionRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct AddItemRequest {
    pub item_id: String,
}

#[derive(Serialize)]
pub struct CollectionsResponse {
    pub version: u32,
    pub collections: Vec<crate::user_settings::collections::EnrichedCollection>,
}

#[derive(Serialize)]
pub struct CreateCollectionResponse {
    pub id: String,
}

/// Get all collections with enriched item data
#[get("/api/collections")]
pub async fn get_collections(
    data: web::Data<AppState>,
    _user: FirebaseUser,
) -> io::Result<impl Responder> {
    let (collections, lookup) = {
        let state = data.lock().map_err(|_| io::ErrorKind::Other)?;
        let lookup = state.lookup_table.clone();
        drop(state);

        let config_dir = data.get_config_dir()?;
        let collections = CollectionsConfig::load(&config_dir)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        (collections, lookup)
    };

    let enriched = collections.enrich_with_lookup(&lookup);

    Ok(web::Json(CollectionsResponse {
        version: collections.version,
        collections: enriched,
    }))
}

/// Create a new collection
#[post("/api/collections")]
pub async fn create_collection(
    data: web::Data<AppState>,
    body: web::Json<CreateCollectionRequest>,
    _user: FirebaseUser,
) -> io::Result<impl Responder> {
    let config_dir = data.get_config_dir()?;

    let mut collections = CollectionsConfig::load(&config_dir)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let id = collections
        .create_collection(body.name.clone())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    collections
        .save(&config_dir)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(web::Json(CreateCollectionResponse { id }))
}

/// Rename a collection
#[patch("/api/collections/{id}")]
pub async fn rename_collection(
    data: web::Data<AppState>,
    id: web::Path<String>,
    body: web::Json<RenameCollectionRequest>,
    _user: FirebaseUser,
) -> io::Result<impl Responder> {
    let config_dir = data.get_config_dir()?;

    let mut collections = CollectionsConfig::load(&config_dir)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    collections
        .rename_collection(&id, body.name.clone())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    collections
        .save(&config_dir)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(web::Json(serde_json::json!({ "success": true })))
}

/// Delete a collection
#[delete("/api/collections/{id}")]
pub async fn delete_collection(
    data: web::Data<AppState>,
    id: web::Path<String>,
    _user: FirebaseUser,
) -> io::Result<impl Responder> {
    let config_dir = data.get_config_dir()?;

    let mut collections = CollectionsConfig::load(&config_dir)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    collections
        .delete_collection(&id)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    collections
        .save(&config_dir)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(web::Json(serde_json::json!({ "success": true })))
}

/// Add an item to a collection
#[post("/api/collections/{id}/items")]
pub async fn add_item_to_collection(
    data: web::Data<AppState>,
    id: web::Path<String>,
    body: web::Json<AddItemRequest>,
    _user: FirebaseUser,
) -> io::Result<impl Responder> {
    let (config_dir, item_clone) = {
        let state = data.lock().map_err(|_| io::ErrorKind::Other)?;

        // Get the item from lookup table
        let item = state
            .lookup_table
            .get_map_ref()
            .get(&body.item_id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Item not found"))?;

        // Validate it's a file (not a directory)
        if item.is_dir {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Only files can be added to collections",
            ));
        }

        let item_clone = item.clone();
        drop(state);

        let config_dir = data.get_config_dir()?;
        (config_dir, item_clone)
    };

    let mut collections = CollectionsConfig::load(&config_dir)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    collections
        .add_item(&id, &item_clone)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    collections
        .save(&config_dir)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(web::Json(serde_json::json!({ "success": true })))
}

/// Remove an item from a collection
#[delete("/api/collections/{id}/items/{item_id}")]
pub async fn remove_item_from_collection(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
    _user: FirebaseUser,
) -> io::Result<impl Responder> {
    let (collection_id, item_id) = path.into_inner();
    let config_dir = data.get_config_dir()?;

    let mut collections = CollectionsConfig::load(&config_dir)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    collections
        .remove_item(&collection_id, &item_id)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    collections
        .save(&config_dir)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(web::Json(serde_json::json!({ "success": true })))
}
