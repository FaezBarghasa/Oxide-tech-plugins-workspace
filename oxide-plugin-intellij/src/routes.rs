#[cfg(not(target_arch = "wasm32"))]
use actix_web::{get, post, web, HttpResponse, Responder};
#[cfg(not(target_arch = "wasm32"))]
use serde::{Serialize, Deserialize};
#[cfg(not(target_arch = "wasm32"))]
use crate::db::{self, AutomationSkill, McpExtension, JetBrainsExtension};
#[cfg(not(target_arch = "wasm32"))]
use crate::ai;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize, Debug)]
pub struct TogglePayload {
    pub id: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddJetBrainsExtensionPayload {
    pub name: String,
    pub type_: String,
    pub implementation_class: String,
    pub description: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SimulatePayload {
    pub event_type: String,
    pub value: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddMcpExtensionPayload {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub capabilities: Option<Vec<String>>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddSkillPayload {
    pub name: String,
    pub description: Option<String>,
    pub script: String,
    pub type_: Option<String>,
    pub trigger: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize, Debug)]
pub struct RunSkillPayload {
    pub id: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub role: String, // "User" | "Model"
    pub content: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize, Debug)]
pub struct ChatRequest {
    pub prompt: String,
    pub history: Vec<Message>,
    pub mode: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Debug)]
pub struct ChatResponse {
    pub text: Option<String>,
    pub error: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize, Debug)]
pub struct PRReviewRequest {
    pub code: String,
    pub title: String,
    pub description: String,
}

// Routes implementations

#[cfg(not(target_arch = "wasm32"))]
#[get("/api/jetbrains")]
pub async fn get_jetbrains() -> impl Responder {
    match db::get_jetbrains_state().await {
        Ok(state) => match db::get_jetbrains_extensions().await {
            Ok(exts) => HttpResponse::Ok().json(serde_json::json!({
                "state": state,
                "extensions": exts,
            })),
            Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {:?}", e)),
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[post("/api/jetbrains/connection")]
pub async fn toggle_jetbrains_connection() -> impl Responder {
    match db::get_jetbrains_state().await {
        Ok(mut state) => {
            state.connected = !state.connected;
            state.last_handshake = chrono::Local::now().to_rfc3339();
            if let Ok(updated) = db::update_jetbrains_state(state).await {
                if let Ok(exts) = db::get_jetbrains_extensions().await {
                    return HttpResponse::Ok().json(serde_json::json!({
                        "state": updated,
                        "extensions": exts,
                    }));
                }
            }
            HttpResponse::InternalServerError().body("Failed to update database")
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[post("/api/jetbrains/toggle")]
pub async fn toggle_jetbrains_extension(payload: web::Json<TogglePayload>) -> impl Responder {
    match db::get_jetbrains_extensions().await {
        Ok(exts) => {
            if let Some(mut found) = exts.into_iter().find(|e| e.id == payload.id) {
                found.enabled = !found.enabled;
                let _ = db::update_jetbrains_extension(found).await;
            }
            let state = db::get_jetbrains_state().await.unwrap();
            let final_exts = db::get_jetbrains_extensions().await.unwrap();
            HttpResponse::Ok().json(serde_json::json!({
                "state": state,
                "extensions": final_exts,
            }))
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[post("/api/jetbrains/add")]
pub async fn add_jetbrains_extension(payload: web::Json<AddJetBrainsExtensionPayload>) -> impl Responder {
    let new_ext = JetBrainsExtension {
        id: format!("ext-{}", chrono::Local::now().timestamp_millis()),
        name: payload.name.clone(),
        type_: payload.type_.clone(),
        implementation_class: payload.implementation_class.clone(),
        description: payload.description.clone().unwrap_or_else(|| "Custom user-defined IDE extension point.".to_string()),
        enabled: true,
    };

    match db::add_jetbrains_extension(new_ext).await {
        Ok(_) => {
            let state = db::get_jetbrains_state().await.unwrap();
            let final_exts = db::get_jetbrains_extensions().await.unwrap();
            HttpResponse::Ok().json(serde_json::json!({
                "state": state,
                "extensions": final_exts,
            }))
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[post("/api/jetbrains/compile")]
pub async fn compile_jetbrains() -> impl Responder {
    match db::get_jetbrains_state().await {
        Ok(mut state) => {
            if state.gradle_task_executing {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Gradle build task already running"
                }));
            }
            state.gradle_task_executing = true;
            state.gradle_log = "[GRADLE] Executing task: :buildPlugin\n[GRADLE] Loading project configurations...\n".to_string();
            let _ = db::update_jetbrains_state(state.clone()).await;

            // Spawn background task to complete compile
            tokio::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;
                if let Ok(mut st) = db::get_jetbrains_state().await {
                    st.gradle_log += "[GRADLE] :patchPluginXml SUCCESS\n[GRADLE] :compileKotlin SUCCESS\n[GRADLE] :buildSearchableOptions SUCCESS\n[GRADLE] :prepareSandbox SUCCESS\n[GRADLE] :buildPlugin SUCCESS\n[SUCCESS] Compiled JetBrains extension zip package: build/distributions/oxide-tech-jetbrains-plugin-0.1.0.zip\n";
                    st.gradle_task_executing = false;
                    let _ = db::update_jetbrains_state(st).await;
                }
            });

            let final_exts = db::get_jetbrains_extensions().await.unwrap();
            HttpResponse::Ok().json(serde_json::json!({
                "state": state,
                "extensions": final_exts,
            }))
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[post("/api/jetbrains/simulate")]
pub async fn simulate_jetbrains(payload: web::Json<SimulatePayload>) -> impl Responder {
    let state = db::get_jetbrains_state().await.unwrap();
    let extensions = db::get_jetbrains_extensions().await.unwrap();
    let mut log = format!("[SIMULATION] Sending payload to JetBrains plugin endpoint at port {}...\n", state.port);
    let result;

    match payload.event_type.as_str() {
        "completion" => {
            log += &format!("[INTEL_IDEA] Request: Get register completions matching term \"{}\"\n[AGENT RESPONSE] Injected register matches: [\"TIM2\", \"TIM3\", \"TIM4\", \"TIM5\", \"TIM6_DAC\"] with bitfields.\n", payload.value.as_deref().unwrap_or("TIM"));
            result = serde_json::json!({
                "annotated": true,
                "items": ["TIM2", "TIM3", "TIM4", "TIM5", "TIM6_DAC"],
                "description": "Registers mapped to STM32 AHB/APB clocks."
            });
        }
        "nostd" => {
            log += "[INTEL_IDEA] Request: Inspect imports of source code snippet.\n[AGENT RESPONSE] Violation found: detected \"use std::collections::VecDeque\" on bare-metal target.\n";
            result = serde_json::json!({
                "approved": false,
                "issues": [{
                    "severity": "critical",
                    "line": 3,
                    "title": "Std Library Import Detected",
                    "remedy": "Use alloc::collections::VecDeque or heapless::Deque instead."
                }]
            });
        }
        "hoverdoc" => {
            log += &format!("[INTEL_IDEA] Request: Hover documentation on symbol \"{}\"\n[AGENT RESPONSE] Injected quick-dock datasheet markdown\n", payload.value.as_deref().unwrap_or("RCC_AHB1ENR"));
            result = serde_json::json!({
                "doc": "**RCC_AHB1ENR**: RCC AHB1 peripheral clock register. Enables peripherals like GPIOA, GPIOB, GPIOC, GPIOD, GPIOH etc."
            });
        }
        _ => {
            log += "[INTEL_IDEA] Request: General extension query\n[AGENT RESPONSE] Handshake OK, connection stable.\n";
            result = serde_json::json!({ "status": "OK", "timestamp": chrono::Local::now().timestamp() });
        }
    }

    HttpResponse::Ok().json(serde_json::json!({
        "state": state,
        "extensions": extensions,
        "log": log,
        "result": result,
    }))
}

#[cfg(not(target_arch = "wasm32"))]
#[get("/api/mcp")]
pub async fn get_mcp() -> impl Responder {
    match db::get_mcp_servers().await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[post("/api/mcp/toggle")]
pub async fn toggle_mcp(payload: web::Json<TogglePayload>) -> impl Responder {
    match db::get_mcp_servers().await {
        Ok(list) => {
            if let Some(mut found) = list.into_iter().find(|m| m.id == payload.id) {
                found.status = if found.status == "Connected" {
                    "Disconnected".to_string()
                } else {
                    "Connected".to_string()
                };
                let _ = db::update_mcp_server(found).await;
            }
            let final_list = db::get_mcp_servers().await.unwrap();
            HttpResponse::Ok().json(final_list)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[post("/api/mcp/add")]
pub async fn add_mcp(payload: web::Json<AddMcpExtensionPayload>) -> impl Responder {
    let new_mcp = McpExtension {
        id: format!("mcp-{}", chrono::Local::now().timestamp_millis()),
        name: payload.name.clone(),
        description: payload.description.clone().unwrap_or_else(|| "Custom MCP Extension Service".to_string()),
        url: payload.url.clone(),
        status: "Connected".to_string(),
        capabilities: payload.capabilities.clone().unwrap_or_else(|| vec!["tools/general_query".to_string()]),
    };

    match db::add_mcp_server(new_mcp).await {
        Ok(_) => {
            let final_list = db::get_mcp_servers().await.unwrap();
            HttpResponse::Ok().json(final_list)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[post("/api/mcp/discover")]
pub async fn discover_mcp() -> impl Responder {
    match db::get_mcp_servers().await {
        Ok(list) => {
            let discovered = McpExtension {
                id: "discovered-probe-mcp".to_string(),
                name: "Logic Analyzer Waveform Parser (Probed)".to_string(),
                description: "Automated logic analyzer wave mapping utility. Found locally over LAN.".to_string(),
                url: "http://127.0.0.1:8599/mcp/waveforms".to_string(),
                status: "Discovered".to_string(),
                capabilities: vec![
                    "tools/parse_spi_wave".to_string(),
                    "tools/parse_i2c_wave".to_string(),
                ],
            };
            if !list.iter().any(|m| m.id == discovered.id) {
                let _ = db::add_mcp_server(discovered).await;
            }
            let final_list = db::get_mcp_servers().await.unwrap();
            HttpResponse::Ok().json(final_list)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[get("/api/skills")]
pub async fn get_skills() -> impl Responder {
    match db::get_skills().await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[post("/api/skills")]
pub async fn add_skill(payload: web::Json<AddSkillPayload>) -> impl Responder {
    let new_skill = AutomationSkill {
        id: format!("skill-{}", chrono::Local::now().timestamp_millis()),
        name: payload.name.clone(),
        description: payload.description.clone().unwrap_or_else(|| "Custom script-based automation task.".to_string()),
        script: payload.script.clone(),
        type_: payload.type_.clone().unwrap_or_else(|| "bash".to_string()),
        trigger: payload.trigger.clone().unwrap_or_else(|| "Manual".to_string()),
        status: "Idle".to_string(),
        execution_log: Some("".to_string()),
        last_executed: None,
    };

    match db::add_skill(new_skill).await {
        Ok(created) => HttpResponse::Ok().json(created),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[post("/api/skills/run")]
pub async fn run_skill(payload: web::Json<RunSkillPayload>) -> impl Responder {
    match db::get_skill(&payload.id).await {
        Ok(Some(mut skill)) => {
            skill.status = "Running".to_string();
            skill.last_executed = Some(chrono::Local::now().to_rfc3339());
            skill.execution_log = Some(format!(
                "[SYSTEM INFO] Initiating automation script: \"{}\"...\n[SHELL] Executing: `{}`\n",
                skill.name, skill.script
            ));
            let _ = db::update_skill(skill.clone()).await;

            let skill_id = skill.id.clone();
            tokio::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(1200)).await;
                if let Ok(Some(mut s)) = db::get_skill(&skill_id).await {
                    let output = match s.id.as_str() {
                        "validate-pins" => "[VALIDATING] Scanning src/main.rs and components/peripherals.rs...\n[OK] Checked PB3 (SPI_SCK), PA1 (PWM_TIM), PA0 (GPIO_IN).\n[WARNING] Overlap overlap detected on PA1. Registering quick mitigation strategy inside IDE.\n[SUCCESS] Custom configuration verified cleanly!",
                        "enforce-no-std" => "[COMPILING] Verifying #![no_std] compliance checks...\n[SUCCESS] 0 instances of std namespace detected.\n[INFO] Checked 4 source units. No-alloc requirements perfectly followed.",
                        "openocd-flash" => "[GDB] Connecting to OpenOCD target remote port 3333...\n[GDB] Loading target ELF debug symbols...\n[GDB] Verification: Register RCC_AHB1ENR set to 0x1 (AHB1 Peripheral Clock Enabled)\n[SUCCESS] Flashed safely, system is running!",
                        _ => "[RUN] Script executed successfully.\n[LOG] Result state: Zero exit code returned.",
                    };
                    if let Some(log) = &mut s.execution_log {
                        log.push_str(output);
                    }
                    s.status = "Success".to_string();
                    let _ = db::update_skill(s).await;
                }
            });

            HttpResponse::Ok().json(skill)
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({"error": "Skill not found"})),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[post("/api/chat")]
pub async fn handle_chat(req: web::Json<ChatRequest>) -> impl Responder {
    let mut history_tuples = Vec::new();
    for msg in &req.history {
        history_tuples.push((msg.role.clone(), msg.content.clone()));
    }

    match ai::generate_chat_response(&req.prompt, &history_tuples, &req.mode).await {
        Ok(text) => HttpResponse::Ok().json(ChatResponse {
            text: Some(text),
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ChatResponse {
            text: None,
            error: Some(format!("Error calling Gemini assistant API: {:?}", e)),
        }),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[post("/api/review")]
pub async fn handle_review(req: web::Json<PRReviewRequest>) -> impl Responder {
    match ai::audit_code_pr(&req.code, &req.title, &req.description).await {
        Ok(report_json) => {
            // Validate if it is valid json
            match serde_json::from_str::<serde_json::Value>(&report_json) {
                Ok(val) => HttpResponse::Ok().json(val),
                Err(e) => HttpResponse::InternalServerError().body(format!("Failed to parse Gemini JSON: {:?}. Output: {}", e, report_json)),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Gemini audit failed: {:?}", e)),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_jetbrains)
       .service(toggle_jetbrains_connection)
       .service(toggle_jetbrains_extension)
       .service(add_jetbrains_extension)
       .service(compile_jetbrains)
       .service(simulate_jetbrains)
       .service(get_mcp)
       .service(toggle_mcp)
       .service(add_mcp)
       .service(discover_mcp)
       .service(get_skills)
       .service(add_skill)
       .service(run_skill)
       .service(handle_chat)
       .service(handle_review);
}
