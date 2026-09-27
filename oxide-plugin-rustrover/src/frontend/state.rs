#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use serde::{Serialize, Deserialize};

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct McpExtension {
    pub id: String,
    pub name: String,
    pub description: String,
    pub url: String,
    pub status: String,
    pub capabilities: Vec<String>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JetBrainsExtension {
    pub id: String,
    pub name: String,
    pub type_: String,
    pub implementation_class: String,
    pub description: String,
    pub enabled: bool,
}

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Message {
    pub role: String, // "User" | "Model"
    pub content: String,
}

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PRReviewIssue {
    pub severity: String, // "Critical" | "Warning" | "Info"
    pub line: Option<u32>,
    pub title: String,
    pub explanation: String,
    pub recommendation: String,
}

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PRReviewResult {
    pub approved: bool,
    pub score: u32,
    pub summary: String,
    pub issues: Vec<PRReviewIssue>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy)]
pub struct IntellijState {
    pub skills: Signal<Vec<AutomationSkill>>,
    pub mcp_servers: Signal<Vec<McpExtension>>,
    pub jb_extensions: Signal<Vec<JetBrainsExtension>>,
    pub jb_state: Signal<JetbrainsState>,
    pub active_tab: Signal<String>, // "skills" | "mcp" | "jetbrains" | "review" | "explorer"
    pub selected_skill: Signal<Option<AutomationSkill>>,
    pub chat_messages: Signal<Vec<Message>>,
    pub chat_mode: Signal<String>, // "planning" | "executing"
    pub is_loading: Signal<bool>,
    pub error_msg: Signal<Option<String>>,
    pub review_title: Signal<String>,
    pub review_desc: Signal<String>,
    pub review_code: Signal<String>,
    pub review_result: Signal<Option<PRReviewResult>>,
    pub review_submitting: Signal<bool>,
    pub selected_file: Signal<String>,
}
