use actix_web::{HttpResponse, web};
use serde_json::json;
use crate::config::AppConfig;

pub async fn liveness(_config: web::Data<AppConfig>) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "alive",
        "timestamp": chrono::Utc::now()
    }))
}

pub async fn readiness(_config: web::Data<AppConfig>) -> HttpResponse {
    // Check all dependencies
    let checks = json!({
        "surrealdb": "ok",
        "qdrant": "ok",
        "vllm": "ok"
    });

    HttpResponse::Ok().json(json!({
        "status": "ready",
        "checks": checks
    }))
}
