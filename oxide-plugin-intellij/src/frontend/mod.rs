#[cfg(target_arch = "wasm32")]
pub mod state;
#[cfg(target_arch = "wasm32")]
pub mod components;

#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use state::{IntellijState, AutomationSkill, McpExtension, JetBrainsExtension, JetbrainsState, Message};
#[cfg(target_arch = "wasm32")]
use components::MainLayout;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[cfg(target_arch = "wasm32")]
#[component]
pub fn App() -> Element {
    let skills = use_signal(|| Vec::<AutomationSkill>::new());
    let mcp_servers = use_signal(|| Vec::<McpExtension>::new());
    let jb_extensions = use_signal(|| Vec::<JetBrainsExtension>::new());
    let jb_state = use_signal(|| JetbrainsState {
        connected: false,
        port: 8080,
        ide_version: "2024.2 (IntelliJ IDEA Community)".to_string(),
        plugin_build: "v0.1.0-alpha.4".to_string(),
        last_handshake: "".to_string(),
        active_language: "Rust / Kotlin".to_string(),
        gradle_task_executing: false,
        gradle_log: "".to_string(),
    });
    let active_tab = use_signal(|| "skills".to_string());
    let selected_skill = use_signal(|| None::<AutomationSkill>);
    let chat_messages = use_signal(|| vec![
        Message {
            role: "Model".to_string(),
            content: "Hello! I am the Oxide-Tech Local Agent OS Model. Ask me to generate embedded Rust HAL wrappers, design Embassy async executors, specify RTIC resources, or resolve datasheet hardware constraints! How can I assist you with embedded firmware development today?".to_string(),
        }
    ]);
    let chat_mode = use_signal(|| "executing".to_string());
    let is_loading = use_signal(|| false);
    let error_msg = use_signal(|| None::<String>);

    let review_title = use_signal(|| "Fix circular telemetry buffer".to_string());
    let review_desc = use_signal(|| "Introduces direct heap FIFO queue metrics.".to_string());
    let review_code = use_signal(|| r#"#![no_std]
// Intended as premium ring telemetry driver
use std::collections::VecDeque;

pub struct TelemetryBuffer {
    data: VecDeque<u32>,
}

impl TelemetryBuffer {
    pub fn new() -> Self {
        Self { data: VecDeque::new() }
    }
}
"#.to_string());
    let review_result = use_signal(|| None);
    let review_submitting = use_signal(|| false);
    let selected_file = use_signal(|| "src/lib.rs".to_string());

    let mut state = IntellijState {
        skills,
        mcp_servers,
        jb_extensions,
        jb_state,
        active_tab,
        selected_skill,
        chat_messages,
        chat_mode,
        is_loading,
        error_msg,
        review_title,
        review_desc,
        review_code,
        review_result,
        review_submitting,
        selected_file,
    };

    use_context_provider(|| state);

    // Initial data fetch on mount
    use_effect(move || {
        spawn_local(async move {
            let client = reqwest::Client::new();

            // 1. Fetch Skills
            if let Ok(resp) = client.get("/api/skills").send().await {
                if resp.status().is_success() {
                    if let Ok(list) = resp.json::<Vec<AutomationSkill>>().await {
                        state.skills.set(list.clone());
                        if !list.is_empty() {
                            state.selected_skill.set(Some(list[0].clone()));
                        }
                    }
                }
            }

            // 2. Fetch MCP Servers
            if let Ok(resp) = client.get("/api/mcp").send().await {
                if resp.status().is_success() {
                    if let Ok(list) = resp.json::<Vec<McpExtension>>().await {
                        state.mcp_servers.set(list);
                    }
                }
            }

            // 3. Fetch JetBrains Extension Status
            if let Ok(resp) = client.get("/api/jetbrains").send().await {
                if resp.status().is_success() {
                    if let Ok(json_val) = resp.json::<serde_json::Value>().await {
                        if let Ok(st) = serde_json::from_value::<JetbrainsState>(json_val["state"].clone()) {
                            state.jb_state.set(st);
                        }
                        if let Ok(exts) = serde_json::from_value::<Vec<JetBrainsExtension>>(json_val["extensions"].clone()) {
                            state.jb_extensions.set(exts);
                        }
                    }
                }
            }
        });
    });

    rsx! {
        style {
            r#"
            @keyframes pulse {{
                0%, 100% {{ opacity: 1; }}
                50% {{ opacity: 0.5; }}
            }}
            .animate-pulse-subtle {{
                animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
            }}
            "#
        }
        MainLayout {}
    }
}
