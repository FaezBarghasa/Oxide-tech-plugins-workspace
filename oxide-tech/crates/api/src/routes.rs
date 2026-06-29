use actix_web::web;
use crate::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        // Health checks
        .route("/health/live", web::get().to(handlers::health::liveness))
        .route("/health/ready", web::get().to(handlers::health::readiness))
        .route("/metrics", web::get().to(handlers::metrics::prometheus_metrics))
        
        // Compilation endpoints
        .service(
            web::scope("/api/cargo")
                .route("/check", web::post().to(handlers::cargo::check))
                .route("/clippy", web::post().to(handlers::cargo::clippy))
        )
        
        // Tree-sitter endpoints
        .service(
            web::scope("/api/tree-sitter")
                .route("/parse", web::post().to(handlers::tree_sitter::parse))
        )
        
        // KiCad endpoints
        .service(
            web::scope("/api/kicad")
                .route("/load-board", web::post().to(handlers::kicad::load_board))
                .route("/run-drc", web::post().to(handlers::kicad::run_drc))
        )
        
        // Thermal simulation
        .service(
            web::scope("/api/thermal")
                .route("/simulate", web::post().to(handlers::thermal::simulate))
        )
        
        // WebSocket endpoints
        .route("/ws/compilation", web::get().to(handlers::websocket::compilation_stream))
        .route("/ws/agent-progress", web::get().to(handlers::websocket::agent_progress_stream));
}
