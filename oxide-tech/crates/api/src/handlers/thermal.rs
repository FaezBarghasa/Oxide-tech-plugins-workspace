use actix_web::{HttpResponse, web};
use shared::ThermalSimulationRequest;

pub async fn simulate(_req: web::Json<ThermalSimulationRequest>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "temperature_grid": vec![vec![25.0, 26.0], vec![25.5, 27.0]],
        "max_temperature": 27.0,
        "min_temperature": 25.0,
        "elapsed_ms": 125
    }))
}
