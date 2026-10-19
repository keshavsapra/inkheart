use actix_web::{get, patch, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::io;

use crate::server::{auth::FirebaseUser, state::AppState};

#[derive(Debug, Deserialize)]
pub struct UpdateUserSettings {
    pub telemetry_enabled: Option<bool>,
    pub telemetry_banner_shown: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct UserSettingsResponse {
    pub telemetry_enabled: bool,
    pub telemetry_banner_shown: bool,
    pub telemetry_config_override: bool,
}

#[get("/api/settings")]
pub async fn get_user_settings(
    data: web::Data<AppState>,
    _user: FirebaseUser,
) -> io::Result<impl Responder> {
    let config = data.get_config()?;
    let user_settings = data.get_user_settings()?;

    let telemetry_config_override = !config.telemetry_enabled;
    let effective_telemetry = config.telemetry_enabled && user_settings.telemetry_enabled;

    let response = UserSettingsResponse {
        telemetry_enabled: effective_telemetry,
        telemetry_banner_shown: user_settings.telemetry_banner_shown,
        telemetry_config_override,
    };

    Ok(web::Json(response))
}

#[patch("/api/settings")]
pub async fn update_user_settings(
    data: web::Data<AppState>,
    updates: web::Json<UpdateUserSettings>,
    _user: FirebaseUser,
) -> io::Result<impl Responder> {
    let mut settings = data.get_user_settings()?;

    if let Some(enabled) = updates.telemetry_enabled {
        settings.telemetry_enabled = enabled;
    }
    if let Some(shown) = updates.telemetry_banner_shown {
        settings.telemetry_banner_shown = shown;
    }

    data.save_user_settings(&settings)?;

    Ok(HttpResponse::Ok().json(settings))
}
