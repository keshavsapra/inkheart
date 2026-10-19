use actix_web::{delete, get, post, web, Responder};
use serde::{Deserialize, Serialize};
use std::io;

use crate::{
    indexer::SafeItemRecord,
    server::{auth::FirebaseUser, state::AppState},
    user_settings::PinConfig,
};

#[derive(Deserialize)]
pub struct AddPinRequest {
    pub id: String,
    pub display_name: Option<String>,
}

#[derive(Serialize)]
pub struct PinWithDetails {
    pub id: String,
    pub pinned_at: u64,
    pub display_name: Option<String>,
    pub last_seen_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_item: Option<SafeItemRecord>,
}

#[derive(Serialize)]
pub struct MissingPinWithDetails {
    pub id: String,
    pub pinned_at: u64,
    pub missing_since: u64,
    pub display_name: Option<String>,
    pub last_seen_path: String,
}

#[derive(Serialize)]
pub struct PinsResponse {
    pub version: u32,
    pub pinned: Vec<PinWithDetails>,
    pub missing: Vec<MissingPinWithDetails>,
}

/// Get all pinned folders with current item details
#[get("/api/pins")]
pub async fn get_pins(
    data: web::Data<AppState>,
    _user: FirebaseUser,
) -> io::Result<impl Responder> {
    let (pins, lookup) = {
        let state = data.lock().map_err(|_| io::ErrorKind::Other)?;
        let lookup = state.lookup_table.clone();
        drop(state);

        let config_dir = data.get_config_dir()?;
        let pins =
            PinConfig::load(&config_dir).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        (pins, lookup)
    };

    // Enrich pinned items with current data from lookup table
    let pinned_with_details: Vec<PinWithDetails> = pins
        .pinned
        .iter()
        .map(|pin| {
            let current_item = lookup
                .get_map_ref()
                .get(&pin.id)
                .filter(|item| item.is_dir)
                .map(|item| SafeItemRecord::from(item));

            PinWithDetails {
                id: pin.id.clone(),
                pinned_at: pin.pinned_at,
                display_name: pin.display_name.clone(),
                last_seen_path: pin.last_seen_path.clone(),
                current_item,
            }
        })
        .collect();

    let missing_with_details: Vec<MissingPinWithDetails> = pins
        .missing
        .iter()
        .map(|missing| MissingPinWithDetails {
            id: missing.id.clone(),
            pinned_at: missing.pinned_at,
            missing_since: missing.missing_since,
            display_name: missing.display_name.clone(),
            last_seen_path: missing.last_seen_path.clone(),
        })
        .collect();

    Ok(web::Json(PinsResponse {
        version: pins.version,
        pinned: pinned_with_details,
        missing: missing_with_details,
    }))
}

/// Add a new pinned folder
#[post("/api/pins")]
pub async fn add_pin(
    data: web::Data<AppState>,
    body: web::Json<AddPinRequest>,
    _user: FirebaseUser,
) -> io::Result<impl Responder> {
    let (config_dir, item_clone) = {
        let state = data.lock().map_err(|_| io::ErrorKind::Other)?;

        // Get the item from lookup table
        let item = state
            .lookup_table
            .get_map_ref()
            .get(&body.id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Item not found"))?;

        // Validate it's a directory
        if !item.is_dir {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Only folders can be pinned",
            ));
        }

        let item_clone = item.clone();
        drop(state);

        let config_dir = data.get_config_dir()?;
        (config_dir, item_clone)
    };

    // Load pins, add, save
    let mut pins =
        PinConfig::load(&config_dir).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    pins.add_pin(&item_clone, body.display_name.clone())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    pins.save(&config_dir)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(web::Json(serde_json::json!({ "success": true })))
}

/// Remove a pinned folder
#[delete("/api/pins/{id}")]
pub async fn remove_pin(
    data: web::Data<AppState>,
    id: web::Path<String>,
    _user: FirebaseUser,
) -> io::Result<impl Responder> {
    let config_dir = data.get_config_dir()?;

    let mut pins =
        PinConfig::load(&config_dir).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    pins.remove_pin(&id);

    pins.save(&config_dir)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(web::Json(serde_json::json!({ "success": true })))
}

/// Remove a missing pinned folder
#[delete("/api/pins/missing/{id}")]
pub async fn remove_missing_pin(
    data: web::Data<AppState>,
    id: web::Path<String>,
    _user: FirebaseUser,
) -> io::Result<impl Responder> {
    let config_dir = data.get_config_dir()?;

    let mut pins =
        PinConfig::load(&config_dir).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    pins.remove_missing(&id);

    pins.save(&config_dir)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(web::Json(serde_json::json!({ "success": true })))
}
