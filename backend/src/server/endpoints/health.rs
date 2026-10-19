use actix_web::{get, web, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct HealthStatus {
    status: &'static str,
}

#[get("/api/health")]
pub async fn health_check() -> impl Responder {
    let response = HealthStatus { status: "ok" };

    web::Json(response)
}
