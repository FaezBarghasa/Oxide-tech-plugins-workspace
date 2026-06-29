#[cfg(not(target_arch = "wasm32"))]
use serde::{Serialize, Deserialize};
#[cfg(not(target_arch = "wasm32"))]
use surrealdb::engine::local::{Db, SurrealKv};
#[cfg(not(target_arch = "wasm32"))]
use surrealdb::Surreal;
#[cfg(not(target_arch = "wasm32"))]
use surrealdb::types::SurrealValue;


#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, surrealdb::types::SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct AutomationSkill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub script: String,
    pub type_: String,
    pub trigger: String,
    pub last_executed: Option<String>,
    pub status: String,
    pub execution_log: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, surrealdb::types::SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct McpExtension {
    pub id: String,
    pub name: String,
    pub description: String,
    pub url: String,
    pub status: String,
    pub capabilities: Vec<String>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, surrealdb::types::SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct JetBrainsExtension {
    pub id: String,
    pub name: String,
    pub type_: String,
    pub implementation_class: String,
    pub description: String,
    pub enabled: bool,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, surrealdb::types::SurrealValue)]
#[serde(rename_all = "camelCase")]
pub struct JetbrainsState {
    pub connected: bool,
    pub port: i32,
    pub ide_version: String,
    pub plugin_build: String,
    pub last_handshake: String,
    pub active_language: String,
    pub gradle_task_executing: bool,
    pub gradle_log: String,
}

#[cfg(not(target_arch = "wasm32"))]
pub static DB: std::sync::LazyLock<Surreal<Db>> = std::sync::LazyLock::new(Surreal::init);

#[cfg(not(target_arch = "wasm32"))]
fn default_skills() -> Vec<AutomationSkill> {
    vec![
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
    ]
}

#[cfg(not(target_arch = "wasm32"))]
fn default_mcp_extensions() -> Vec<McpExtension> {
    vec![
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
    ]
}

#[cfg(not(target_arch = "wasm32"))]
fn default_jetbrains_extensions() -> Vec<JetBrainsExtension> {
    vec![
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
    ]
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn init() -> surrealdb::Result<()> {
    DB.connect::<SurrealKv>("oxide_intellij.db").await?;
    DB.use_ns("oxide").use_db("intellij").await?;

    // Create default skills if they do not exist
    for skill in default_skills() {
        let mut response = DB.query("SELECT * FROM skills WHERE id = $id")
            .bind(("id", skill.id.clone()))
            .await?;
        let existing: Option<AutomationSkill> = response.take(0)?;
        if existing.is_none() {
            let mut create_resp = DB.query("CREATE skills CONTENT $skill")
                .bind(("skill", skill.clone()))
                .await?;
            let _: Option<AutomationSkill> = create_resp.take(0)?;
        }
    }

    // Create default mcp extensions if they do not exist
    for mcp in default_mcp_extensions() {
        let mut response = DB.query("SELECT * FROM mcp_servers WHERE id = $id")
            .bind(("id", mcp.id.clone()))
            .await?;
        let existing: Option<McpExtension> = response.take(0)?;
        if existing.is_none() {
            let mut create_resp = DB.query("CREATE mcp_servers CONTENT $mcp")
                .bind(("mcp", mcp.clone()))
                .await?;
            let _: Option<McpExtension> = create_resp.take(0)?;
        }
    }

    // Create default jetbrains extensions if they do not exist
    for ext in default_jetbrains_extensions() {
        let mut response = DB.query("SELECT * FROM jetbrains_extensions WHERE id = $id")
            .bind(("id", ext.id.clone()))
            .await?;
        let existing: Option<JetBrainsExtension> = response.take(0)?;
        if existing.is_none() {
            let mut create_resp = DB.query("CREATE jetbrains_extensions CONTENT $ext")
                .bind(("ext", ext.clone()))
                .await?;
            let _: Option<JetBrainsExtension> = create_resp.take(0)?;
        }
    }

    // Create default jetbrains state if it doesn't exist
    let mut state_response = DB.query("SELECT * FROM jetbrains_state:default").await?;
    let state: Option<JetbrainsState> = state_response.take(0)?;
    if state.is_none() {
        let default_state = JetbrainsState {
            connected: true,
            port: 8085,
            ide_version: "2024.2 (IntelliJ IDEA Community)".to_string(),
            plugin_build: "v0.1.0-alpha.4".to_string(),
            last_handshake: chrono::Local::now().to_rfc3339(),
            active_language: "Rust / Kotlin".to_string(),
            gradle_task_executing: false,
            gradle_log: "".to_string(),
        };
        let mut create_resp = DB.query("CREATE jetbrains_state:default CONTENT $state")
            .bind(("state", default_state))
            .await?;
        let _: Option<JetbrainsState> = create_resp.take(0)?;
    }

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn get_skills() -> surrealdb::Result<Vec<AutomationSkill>> {
    let mut response = DB.query("SELECT * FROM skills").await?;
    let list: Vec<AutomationSkill> = response.take(0)?;
    Ok(list)
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn get_skill(id: &str) -> surrealdb::Result<Option<AutomationSkill>> {
    let mut response = DB.query("SELECT * FROM skills WHERE id = $id")
        .bind(("id", id))
        .await?;
    let skill: Option<AutomationSkill> = response.take(0)?;
    Ok(skill)
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn add_skill(skill: AutomationSkill) -> surrealdb::Result<AutomationSkill> {
    let mut response = DB.query("CREATE skills CONTENT $skill")
        .bind(("skill", skill.clone()))
        .await?;
    let created: Option<AutomationSkill> = response.take(0)?;
    Ok(created.expect("Failed to create skill"))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn update_skill(skill: AutomationSkill) -> surrealdb::Result<AutomationSkill> {
    let mut response = DB.query("UPDATE skills CONTENT $skill WHERE id = $id")
        .bind(("skill", skill.clone()))
        .bind(("id", skill.id.clone()))
        .await?;
    let updated: Option<AutomationSkill> = response.take(0)?;
    Ok(updated.expect("Failed to update skill"))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn get_mcp_servers() -> surrealdb::Result<Vec<McpExtension>> {
    let mut response = DB.query("SELECT * FROM mcp_servers").await?;
    let list: Vec<McpExtension> = response.take(0)?;
    Ok(list)
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn add_mcp_server(mcp: McpExtension) -> surrealdb::Result<McpExtension> {
    let mut response = DB.query("CREATE mcp_servers CONTENT $mcp")
        .bind(("mcp", mcp.clone()))
        .await?;
    let created: Option<McpExtension> = response.take(0)?;
    Ok(created.expect("Failed to create mcp server"))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn update_mcp_server(mcp: McpExtension) -> surrealdb::Result<McpExtension> {
    let mut response = DB.query("UPDATE mcp_servers CONTENT $mcp WHERE id = $id")
        .bind(("mcp", mcp.clone()))
        .bind(("id", mcp.id.clone()))
        .await?;
    let updated: Option<McpExtension> = response.take(0)?;
    Ok(updated.expect("Failed to update mcp server"))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn get_jetbrains_state() -> surrealdb::Result<JetbrainsState> {
    let mut response = DB.query("SELECT * FROM jetbrains_state:default").await?;
    let state: Option<JetbrainsState> = response.take(0)?;
    Ok(state.expect("State should exist"))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn update_jetbrains_state(state: JetbrainsState) -> surrealdb::Result<JetbrainsState> {
    let mut response = DB.query("UPDATE jetbrains_state:default CONTENT $state")
        .bind(("state", state))
        .await?;
    let updated: Option<JetbrainsState> = response.take(0)?;
    Ok(updated.expect("Failed to update state"))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn get_jetbrains_extensions() -> surrealdb::Result<Vec<JetBrainsExtension>> {
    let mut response = DB.query("SELECT * FROM jetbrains_extensions").await?;
    let list: Vec<JetBrainsExtension> = response.take(0)?;
    Ok(list)
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn add_jetbrains_extension(ext: JetBrainsExtension) -> surrealdb::Result<JetBrainsExtension> {
    let mut response = DB.query("CREATE jetbrains_extensions CONTENT $ext")
        .bind(("ext", ext.clone()))
        .await?;
    let created: Option<JetBrainsExtension> = response.take(0)?;
    Ok(created.expect("Failed to create jetbrains extension"))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn update_jetbrains_extension(ext: JetBrainsExtension) -> surrealdb::Result<JetBrainsExtension> {
    let mut response = DB.query("UPDATE jetbrains_extensions CONTENT $ext WHERE id = $id")
        .bind(("ext", ext.clone()))
        .bind(("id", ext.id.clone()))
        .await?;
    let updated: Option<JetBrainsExtension> = response.take(0)?;
    Ok(updated.expect("Failed to update jetbrains extension"))
}
