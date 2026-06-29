mod app;
mod models;

use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::Local;
use models::{
    AddJetBrainsExtensionPayload, AddMcpExtensionPayload, AddSkillPayload, AutomationSkill,
    JetBrainsExtension, JetbrainsState, McpExtension, RunSkillPayload, SimulatePayload,
    TogglePayload,
};
use std::sync::Mutex;
use tokio::time::{sleep, Duration};

struct AppState {
    skills: Mutex<Vec<AutomationSkill>>,
    mcp_extensions: Mutex<Vec<McpExtension>>,
    jetbrains_extensions: Mutex<Vec<JetBrainsExtension>>,
    jetbrains_state: Mutex<JetbrainsState>,
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

#[post("/api/jetbrains/connection")]
async fn toggle_jetbrains_connection(data: web::Data<AppState>) -> impl Responder {
    let mut state = data.jetbrains_state.lock().unwrap();
    state.connected = !state.connected;
    state.last_handshake = Local::now().to_rfc3339();
    let extensions = data.jetbrains_extensions.lock().unwrap();
    HttpResponse::Ok().json(serde_json::json!({
        "state": &*state,
        "extensions": &*extensions,
    }))
}

#[post("/api/jetbrains/toggle")]
async fn toggle_jetbrains_extension(
    data: web::Data<AppState>,
    payload: web::Json<TogglePayload>,
) -> impl Responder {
    let mut extensions = data.jetbrains_extensions.lock().unwrap();
    if let Some(ext) = extensions.iter_mut().find(|e| e.id == payload.id) {
        ext.enabled = !ext.enabled;
    }
    let state = data.jetbrains_state.lock().unwrap();
    HttpResponse::Ok().json(serde_json::json!({
        "state": &*state,
        "extensions": &*extensions,
    }))
}

#[post("/api/jetbrains/add")]
async fn add_jetbrains_extension(
    data: web::Data<AppState>,
    payload: web::Json<AddJetBrainsExtensionPayload>,
) -> impl Responder {
    let mut extensions = data.jetbrains_extensions.lock().unwrap();
    let new_ext = JetBrainsExtension {
        id: format!("ext-{}", Local::now().timestamp_millis()),
        name: payload.name.clone(),
        type_: payload.type_.clone(),
        implementation_class: payload.implementation_class.clone(),
        description: payload
            .description
            .clone()
            .unwrap_or_else(|| "Custom user-defined IDE extension point.".to_string()),
        enabled: true,
    };
    extensions.push(new_ext);
    let state = data.jetbrains_state.lock().unwrap();
    HttpResponse::Ok().json(serde_json::json!({
        "state": &*state,
        "extensions": &*extensions,
    }))
}

#[post("/api/jetbrains/compile")]
async fn compile_jetbrains(data: web::Data<AppState>) -> impl Responder {
    let mut state = data.jetbrains_state.lock().unwrap();
    if state.gradle_task_executing {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Gradle build task already running"
        }));
    }
    state.gradle_task_executing = true;
    state.gradle_log =
        "[GRADLE] Executing task: :buildPlugin\n[GRADLE] Loading project configurations...\n"
            .to_string();

    let state_clone = data.clone();
    tokio::spawn(async move {
        sleep(Duration::from_millis(1500)).await;
        let mut state = state_clone.jetbrains_state.lock().unwrap();
        state.gradle_log += "[GRADLE] :patchPluginXml SUCCESS\n[GRADLE] :compileKotlin SUCCESS\n[GRADLE] :buildSearchableOptions SUCCESS\n[GRADLE] :prepareSandbox SUCCESS\n[GRADLE] :buildPlugin SUCCESS\n[SUCCESS] Compiled JetBrains extension zip package: build/distributions/oxide-tech-jetbrains-plugin-0.1.0.zip\n";
        state.gradle_task_executing = false;
    });

    let extensions = data.jetbrains_extensions.lock().unwrap();
    HttpResponse::Ok().json(serde_json::json!({
        "state": &*state,
        "extensions": &*extensions,
    }))
}

#[post("/api/jetbrains/simulate")]
async fn simulate_jetbrains(
    data: web::Data<AppState>,
    payload: web::Json<SimulatePayload>,
) -> impl Responder {
    let state = data.jetbrains_state.lock().unwrap();
    let extensions = data.jetbrains_extensions.lock().unwrap();
    let mut log = format!(
        "[SIMULATION] Sending payload to JetBrains plugin endpoint at port {}...\n",
        state.port
    );
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
            result = serde_json::json!({ "status": "OK", "timestamp": Local::now().timestamp() });
        }
    }

    HttpResponse::Ok().json(serde_json::json!({
        "state": &*state,
        "extensions": &*extensions,
        "log": log,
        "result": result,
    }))
}

#[get("/api/mcp")]
async fn get_mcp(data: web::Data<AppState>) -> impl Responder {
    let extensions = data.mcp_extensions.lock().unwrap();
    HttpResponse::Ok().json(&*extensions)
}

#[post("/api/mcp/toggle")]
async fn toggle_mcp(
    data: web::Data<AppState>,
    payload: web::Json<TogglePayload>,
) -> impl Responder {
    let mut extensions = data.mcp_extensions.lock().unwrap();
    if let Some(item) = extensions.iter_mut().find(|m| m.id == payload.id) {
        item.status = if item.status == "Connected" {
            "Disconnected".to_string()
        } else {
            "Connected".to_string()
        };
    }
    HttpResponse::Ok().json(&*extensions)
}

#[post("/api/mcp/add")]
async fn add_mcp(
    data: web::Data<AppState>,
    payload: web::Json<AddMcpExtensionPayload>,
) -> impl Responder {
    let mut extensions = data.mcp_extensions.lock().unwrap();
    let new_item = McpExtension {
        id: format!("mcp-{}", Local::now().timestamp_millis()),
        name: payload.name.clone(),
        description: payload
            .description
            .clone()
            .unwrap_or_else(|| "Custom MCP Extension Service".to_string()),
        url: payload.url.clone(),
        status: "Connected".to_string(),
        capabilities: payload
            .capabilities
            .clone()
            .unwrap_or_else(|| vec!["tools/general_query".to_string()]),
    };
    extensions.push(new_item);
    HttpResponse::Ok().json(&*extensions)
}

#[post("/api/mcp/discover")]
async fn discover_mcp(data: web::Data<AppState>) -> impl Responder {
    let mut extensions = data.mcp_extensions.lock().unwrap();
    let discovered = McpExtension {
        id: "discovered-probe-mcp".to_string(),
        name: "Logic Analyzer Waveform Parser (Probed)".to_string(),
        description: "Automated logic analyzer wave mapping utility. Found locally over LAN."
            .to_string(),
        url: "http://127.0.0.1:8599/mcp/waveforms".to_string(),
        status: "Discovered".to_string(),
        capabilities: vec![
            "tools/parse_spi_wave".to_string(),
            "tools/parse_i2c_wave".to_string(),
        ],
    };
    if !extensions.iter().any(|m| m.id == discovered.id) {
        extensions.push(discovered);
    }
    HttpResponse::Ok().json(&*extensions)
}

#[get("/api/skills")]
async fn get_skills(data: web::Data<AppState>) -> impl Responder {
    let skills = data.skills.lock().unwrap();
    HttpResponse::Ok().json(&*skills)
}

#[post("/api/skills")]
async fn add_skill(
    data: web::Data<AppState>,
    payload: web::Json<AddSkillPayload>,
) -> impl Responder {
    let mut skills = data.skills.lock().unwrap();
    let new_skill = AutomationSkill {
        id: format!("skill-{}", Local::now().timestamp_millis()),
        name: payload.name.clone(),
        description: payload
            .description
            .clone()
            .unwrap_or_else(|| "Custom script-based automation task.".to_string()),
        script: payload.script.clone(),
        type_: payload.type_.clone().unwrap_or_else(|| "bash".to_string()),
        trigger: payload
            .trigger
            .clone()
            .unwrap_or_else(|| "Manual".to_string()),
        status: "Idle".to_string(),
        execution_log: Some("".to_string()),
        last_executed: None,
    };
    skills.push(new_skill.clone());
    HttpResponse::Ok().json(new_skill)
}

#[post("/api/skills/run")]
async fn run_skill(
    data: web::Data<AppState>,
    payload: web::Json<RunSkillPayload>,
) -> impl Responder {
    let mut skills = data.skills.lock().unwrap();
    let skill_index = skills.iter().position(|s| s.id == payload.id);

    if let Some(index) = skill_index {
        let mut skill = skills[index].clone();
        skill.status = "Running".to_string();
        skill.last_executed = Some(Local::now().to_rfc3339());
        skill.execution_log = Some(format!(
            "[SYSTEM INFO] Initiating automation script: \"{}\"...\n[SHELL] Executing: `{}`\n",
            skill.name, skill.script
        ));
        skills[index] = skill.clone();

        let data_clone = data.clone();
        let skill_id = skill.id.clone();
        tokio::spawn(async move {
            sleep(Duration::from_millis(1200)).await;
            let mut skills = data_clone.skills.lock().unwrap();
            if let Some(skill) = skills.iter_mut().find(|s| s.id == skill_id) {
                let output = match skill.id.as_str() {
                    "validate-pins" => "[VALIDATING] Scanning src/main.rs and components/peripherals.rs...\n[OK] Checked PB3 (SPI_SCK), PA1 (PWM_TIM), PA0 (GPIO_IN).\n[WARNING] Overlap overlap detected on PA1. Registering quick mitigation strategy inside IDE.\n[SUCCESS] Custom configuration verified cleanly!",
                    "enforce-no-std" => "[COMPILING] Verifying #![no_std] compliance checks...\n[SUCCESS] 0 instances of std namespace detected.\n[INFO] Checked 4 source units. No-alloc requirements perfectly followed.",
                    "openocd-flash" => "[GDB] Connecting to OpenOCD target remote port 3333...\n[GDB] Loading target ELF debug symbols...\n[GDB] Verification: Register RCC_AHB1ENR set to 0x1 (AHB1 Peripheral Clock Enabled)\n[SUCCESS] Flashed safely, system is running!",
                    _ => "[RUN] Script executed successfully.\n[LOG] Result state: Zero exit code returned.",
                };
                if let Some(log) = &mut skill.execution_log {
                    log.push_str(output);
                }
                skill.status = "Success".to_string();
            }
        });

        HttpResponse::Ok().json(skill)
    } else {
        HttpResponse::NotFound().json(serde_json::json!({"error": "Skill not found"}))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let initial_skills = vec![
        AutomationSkill {
            id: "validate-pins".to_string(),
            name: "Validate Hardware Pinouts".to_string(),
            description: "Scan source code references and cross-reference with STM32 datasheet layout pins for overlap conflicts.".to_string(),
            type_: "bash".to_string(),
            trigger: "OnPreCompile".to_string(),
            script: "cargo run --bin pin_validator -- --mcu=STM32H743VITx --rules=strict".to_string(),
            status: "Idle".to_string(),
            execution_log: Some("No execution logs yet.".to_string()),
            last_executed: None,
        },
        AutomationSkill {
            id: "enforce-no-std".to_string(),
            name: "Enforce no_std Compliance".to_string(),
            description: "Scan Cargo workspace for accidental alloc or std imports (e.g. std::vec::Vec vs core::prelude).".to_string(),
            type_: "rust-macro".to_string(),
            trigger: "OnPreCompile".to_string(),
            script: "#![deny(std_library_usage)]\nmacro_rules! reject_allocs { ... }".to_string(),
            status: "Idle".to_string(),
            execution_log: Some("No execution logs yet.".to_string()),
            last_executed: None,
        },
        AutomationSkill {
            id: "openocd-flash".to_string(),
            name: "GDB Simulator and Autocomplete Verification".to_string(),
            description: "Launches GDB debug loop with OpenOCD target matching to verify registers auto-injecting correct fields.".to_string(),
            type_: "bash".to_string(),
            trigger: "Manual".to_string(),
            script: "arm-none-eabi-gdb -ex \"target remote :3333\" -ex \"load\" -ex \"monitor reset halt\"".to_string(),
            status: "Idle".to_string(),
            execution_log: Some("No execution logs yet.".to_string()),
            last_executed: None,
        },
    ];

    let initial_mcp_extensions = vec![
        McpExtension {
            id: "datasheet-mcp".to_string(),
            name: "MCU Datasheet Registry (STM32/ESP32)".to_string(),
            description: "Model Context Protocol adapter providing fast offline lookup bounds, bitfields metadata, and PDF specs mapping.".to_string(),
            url: "http://localhost:8501/mcp/datasheets".to_string(),
            status: "Connected".to_string(),
            capabilities: vec!["tools/lookup_register".to_string(), "resources/datasheet_schemas".to_string()],
        },
        McpExtension {
            id: "gdb-telemetry-mcp".to_string(),
            name: "GDB Trace Monitor Service".to_string(),
            description: "MCP tool wrapper that queries on-chip memory directly via target debugger inside simulator state.".to_string(),
            url: "http://localhost:8502/mcp/gdb".to_string(),
            status: "Connected".to_string(),
            capabilities: vec!["tools/read_hardware_ram".to_string(), "tools/verify_clock_tree".to_string()],
        },
        McpExtension {
            id: "cargo-diagnostics-mcp".to_string(),
            name: "Cargo Core Diagnostics Assistant".to_string(),
            description: "Provides compiler AST analysis & crate resolution suggestions synchronously to LLM prompt flows.".to_string(),
            url: "http://localhost:8503/mcp/diagnostics".to_string(),
            status: "Disconnected".to_string(),
            capabilities: vec!["prompts/explain_error".to_string(), "tools/verify_crate_compat".to_string()],
        },
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
        JetBrainsExtension {
            id: "hw-constraint-validator".to_string(),
            name: "Hardware Constraint Validator".to_string(),
            type_: "Annotator".to_string(),
            implementation_class: "com.oxidetech.embedded.analysis.HardwareConstraintValidator".to_string(),
            description: "Evaluates STM32 dynamic pin layouts against MCU pin-configuration specs.".to_string(),
            enabled: true,
        },
        JetBrainsExtension {
            id: "mnemonic-completion".to_string(),
            name: "Register Mnemonic Auto-Completion".to_string(),
            type_: "Completion".to_string(),
            implementation_class: "com.oxidetech.embedded.codeCompletion.RegisterMnemonicCompletion".to_string(),
            description: "Enlists hardware register name mnemonics matching STM32 PAC specifications.".to_string(),
            enabled: true,
        },
        JetBrainsExtension {
            id: "register-doc-provider".to_string(),
            name: "Register Quick-Doc Provider".to_string(),
            type_: "QuickDoc".to_string(),
            implementation_class: "com.oxidetech.embedded.documentation.RegisterDocumentationProvider".to_string(),
            description: "Injects live hardware datasheet descriptions inline when hovering over register names.".to_string(),
            enabled: true,
        },
        JetBrainsExtension {
            id: "backend-api-service".to_string(),
            name: "Local Agent Compiler API Client".to_string(),
            type_: "ProjectService".to_string(),
            implementation_class: "com.oxidetech.embedded.services.BackendAPIService".to_string(),
            description: "Maintains a real-time websocket link with the local running rust-agent compiler.".to_string(),
            enabled: true,
        },
    ];

    let initial_jetbrains_state = JetbrainsState {
        connected: true,
        port: 8085,
        ide_version: "2024.2 (IntelliJ IDEA Community)".to_string(),
        plugin_build: "v0.1.0-alpha.4".to_string(),
        last_handshake: Local::now().to_rfc3339(),
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

    println!("Server running on port 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(get_jetbrains)
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
            .service(fs::Files::new("/", "./rust").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
