use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AutomationSkill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub script: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub trigger: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_executed: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_log: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct McpExtension {
    pub id: String,
    pub name: String,
    pub description: String,
    pub url: String,
    pub status: String,
    pub capabilities: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JetBrainsExtension {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub implementation_class: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Deserialize, Serialize, Clone)]
pub struct TogglePayload {
    pub id: String,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddJetBrainsExtensionPayload {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub implementation_class: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SimulatePayload {
    #[serde(rename = "eventType")]
    pub event_type: String,
    pub value: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddMcpExtensionPayload {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub capabilities: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddSkillPayload {
    pub name: String,
    pub description: Option<String>,
    pub script: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub trigger: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RunSkillPayload {
    pub id: String,
}

// Frontend specific models
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum MessageRole {
    User,
    Model,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatRequest {
    pub prompt: String,
    pub history: Vec<Message>,
    pub mode: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ChatResponse {
    pub text: Option<String>,
    pub error: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum PRReviewSeverity {
    Critical,
    Warning,
    Info,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PRReviewIssue {
    pub severity: PRReviewSeverity,
    pub line: Option<u32>,
    pub title: String,
    pub explanation: String,
    pub recommendation: String,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PRReviewResult {
    pub approved: bool,
    pub score: u32,
    pub summary: String,
    pub issues: Vec<PRReviewIssue>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PRReviewRequest {
    pub code: String,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(untagged)] // To handle both direct result and {state, extensions, log, result}
pub enum SimulateResponse {
    Full(SimulateFullResponse),
    Direct(serde_json::Value),
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SimulateFullResponse {
    pub state: JetbrainsState,
    pub extensions: Vec<JetBrainsExtension>,
    pub log: String,
    pub result: serde_json::Value,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JetbrainsApiResponse {
    pub state: JetbrainsState,
    pub extensions: Vec<JetBrainsExtension>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillApiResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub script: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub trigger: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_executed: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_log: Option<String>,
}
