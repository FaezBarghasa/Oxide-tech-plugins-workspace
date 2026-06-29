#[cfg(not(target_arch = "wasm32"))]
use actix_web::{get, post, web, HttpResponse, Responder};
#[cfg(not(target_arch = "wasm32"))]
use serde::{Serialize, Deserialize};
#[cfg(not(target_arch = "wasm32"))]
use crate::db::{self, SchematicData, PCBData};
#[cfg(not(target_arch = "wasm32"))]
use crate::ai::{self, ChatMessage};
#[cfg(not(target_arch = "wasm32"))]
use surrealdb::types::SurrealValue;


#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatRequest {
    pub message: String,
    pub history: Option<Vec<ChatMessage>>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Debug)]
pub struct ChatResponse {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatasheetRequest {
    pub datasheet_text: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize, Serialize, Clone, Debug, surrealdb::types::SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct BoardSyncRequest {
    pub filename: String,
    pub layer_count: i32,
    pub thermal_metrics: serde_json::Value,
    pub dimensions: serde_json::Value,
}


#[cfg(not(target_arch = "wasm32"))]
#[post("/api/chat")]
pub async fn handle_chat(req: web::Json<ChatRequest>) -> impl Responder {
    let history = req.history.clone().unwrap_or_default();
    match ai::generate_chat_response(&req.message, &history).await {
        Ok(text) => HttpResponse::Ok().json(ChatResponse {
            text,
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ChatResponse {
            text: format!("Error processing AI chat: {:?}", e),
            error: Some(true),
        }),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[post("/api/datasheet-parser")]
pub async fn handle_datasheet_parser(req: web::Json<DatasheetRequest>) -> impl Responder {
    match ai::parse_datasheet(&req.datasheet_text).await {
        Ok(json_val) => HttpResponse::Ok().json(json_val),
        Err(e) => HttpResponse::InternalServerError().body(format!("AI datasheet parser error: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[get("/api/schematic")]
pub async fn get_schematic() -> impl Responder {
    match db::get_schematic().await {
        Ok(sch) => HttpResponse::Ok().json(sch),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[post("/api/schematic")]
pub async fn update_schematic(req: web::Json<SchematicData>) -> impl Responder {
    match db::update_schematic(req.into_inner()).await {
        Ok(sch) => HttpResponse::Ok().json(sch),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[get("/api/pcb")]
pub async fn get_pcb() -> impl Responder {
    match db::get_pcb().await {
        Ok(pcb) => HttpResponse::Ok().json(pcb),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[post("/api/pcb")]
pub async fn update_pcb(req: web::Json<PCBData>) -> impl Responder {
    match db::update_pcb(req.into_inner()).await {
        Ok(pcb) => HttpResponse::Ok().json(pcb),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[post("/api/board")]
pub async fn handle_board_sync(req: web::Json<BoardSyncRequest>) -> impl Responder {
    let sync_data = req.into_inner();
    println!("Received KiCad board synchronization update for: {}", sync_data.filename);
    
    // Save to SurrealDB board_sync table
    let db_res = db::DB.query("UPDATE board_sync:default CONTENT $data")
        .bind(("data", sync_data.clone()))
        .await;
        
    match db_res {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "synchronized",
            "filename": sync_data.filename
        })),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error saving sync state: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[get("/api/board")]
pub async fn get_board_sync() -> impl Responder {
    let db_res = db::DB.query("SELECT * FROM board_sync:default").await;
    match db_res {
        Ok(mut resp) => {
            let data: Option<BoardSyncRequest> = resp.take(0).unwrap_or(None);
            HttpResponse::Ok().json(data)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error getting sync state: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(handle_chat)
       .service(handle_datasheet_parser)
       .service(get_schematic)
       .service(update_schematic)
       .service(get_pcb)
       .service(update_pcb)
       .service(handle_board_sync)
       .service(get_board_sync);
}
