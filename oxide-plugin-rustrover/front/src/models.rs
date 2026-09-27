use serde::{Deserialize, Serialize};

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

#[derive(Deserialize)]
pub struct TogglePayload {
    pub id: String,
}

#[derive(Deserialize)]
pub struct AddJetBrainsExtensionPayload {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub implementation_class: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct SimulatePayload {
    #[serde(rename = "eventType")]
    pub event_type: String,
    pub value: Option<String>,
}

#[derive(Deserialize)]
pub struct AddMcpExtensionPayload {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub capabilities: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct AddSkillPayload {
    pub name: String,
    pub description: Option<String>,
    pub script: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub trigger: Option<String>,
}

#[derive(Deserialize)]
pub struct RunSkillPayload {
    pub id: String,
}
