mod models;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

use models::{AutomationSkill, McpExtension, JetBrainsExtension, JetbrainsState};

struct AppState {
    skills: Mutex<Vec<AutomationSkill>>,
    mcp_extensions: Mutex<Vec<McpExtension>>,
    jetbrains_extensions: Mutex<Vec<JetBrainsExtension>>,
    jetbrains_state: Mutex<JetbrainsState>,
}

#[get("/api/skills")]
async fn get_skills(data: web::Data<AppState>) -> impl Responder {
    let skills = data.skills.lock().unwrap();
    HttpResponse::Ok().json(&*skills)
}

#[get("/api/jetbrains")]
async fn get_jetbrains(data: web::Data<AppState>) -> impl Responder {
    let state = data.jetbrains_state.lock().unwrap();
    let extensions = data.jetbrains_extensions.lock().unwrap();
    HttpResponse::Ok().json(serde_json::json!({
        "state": &*state,
        "extensions": &*extensions,
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let initial_skills = vec![
        // ... (skills data)
    ];

    let initial_mcp_extensions = vec![
        // ... (mcp extensions data)
    ];

    let initial_jetbrains_extensions = vec![
        JetBrainsExtension {
            id: "no-std-compliance".to_string(),
            name: "No-Std Compliance Inspection".to_string(),
            type_: "Annotator".to_string(),
            implementation_class: "com.oxidetech.embedded.analysis.NoStdComplianceInspection".to_string(),
            description: "Inspects source files for bare-metal limits and highlights unpermitted imports from std.".to_string(),
            enabled: true,
        },
        // ... (rest of jetbrains extensions)
    ];

    let initial_jetbrains_state = JetbrainsState {
        connected: true,
        port: 8085,
        ide_version: "2024.2 (IntelliJ IDEA Community)".to_string(),
        plugin_build: "v0.1.0-alpha.4".to_string(),
        last_handshake: "now".to_string(), // Simplified
        active_language: "Rust / Kotlin".to_string(),
        gradle_task_executing: false,
        gradle_log: "".to_string(),
    };

    let app_state = web::Data::new(AppState {
        skills: Mutex::new(initial_skills),
        mcp_extensions: Mutex::new(initial_mcp_extensions),
        jetbrains_extensions: Mutex::new(initial_jetbrains_extensions),
        jetbrains_state: Mutex::new(initial_jetbrains_state),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(get_skills)
            .service(get_jetbrains)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
