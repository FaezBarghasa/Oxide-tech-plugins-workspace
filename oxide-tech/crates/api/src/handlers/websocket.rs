use actix_web::{HttpResponse, HttpRequest};

pub async fn compilation_stream(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("WebSocket endpoint")
}

pub async fn agent_progress_stream(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("WebSocket endpoint")
}
