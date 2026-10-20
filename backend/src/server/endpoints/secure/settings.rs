use actix_web::{get, patch, HttpResponse, Responder};
use serde_json::json;

#[get("/api/settings")]
pub async fn get_user_settings() -> impl Responder {
    HttpResponse::Ok().json(json!({}))
}

#[patch("/api/settings")]
pub async fn update_user_settings() -> impl Responder {
    HttpResponse::Ok().json(json!({}))
}
