#[cfg(not(target_arch = "wasm32"))]
use actix_web::{get, post, web, HttpResponse, Responder};
#[cfg(not(target_arch = "wasm32"))]
use serde::{Serialize, Deserialize};
#[cfg(not(target_arch = "wasm32"))]
use crate::db::{self, EnclosureParams};
#[cfg(not(target_arch = "wasm32"))]
use crate::ai;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatRequest {
    pub prompt: String,
    pub current_params: Option<EnclosureParams>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Debug)]
pub struct ChatResponse {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>,
}

#[cfg(not(target_arch = "wasm32"))]
#[post("/api/chat")]
pub async fn handle_chat(req: web::Json<ChatRequest>) -> impl Responder {
    let params = match &req.current_params {
        Some(p) => p.clone(),
        None => match db::get_parameters().await {
            Ok(p) => p,
            Err(e) => {
                return HttpResponse::InternalServerError().json(ChatResponse {
                    text: format!("Database error fetching fallback parameters: {:?}", e),
                    error: Some(true),
                });
            }
        }
    };

    match ai::generate_chat_response(&req.prompt, &params).await {
        Ok(text) => HttpResponse::Ok().json(ChatResponse {
            text,
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ChatResponse {
            text: format!("Error processing AI: {:?}", e),
            error: Some(true),
        }),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[get("/api/parameters")]
pub async fn get_parameters() -> impl Responder {
    match db::get_parameters().await {
        Ok(params) => HttpResponse::Ok().json(params),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[post("/api/parameters")]
pub async fn update_parameters(req: web::Json<EnclosureParams>) -> impl Responder {
    match db::update_parameters(req.into_inner()).await {
        Ok(params) => HttpResponse::Ok().json(params),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(handle_chat)
       .service(get_parameters)
       .service(update_parameters);
}
