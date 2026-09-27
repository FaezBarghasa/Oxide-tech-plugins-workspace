use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Pad {
    pub name: String,
    pub net: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Footprint {
    pub reference: String,
    pub value: String,
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
    pub layer: String,
    pub pads: Vec<Pad>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Trace {
    pub start: Point,
    pub end: Point,
    pub width: f64,
    pub layer: String,
    pub net: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Net {
    pub name: String,
    pub pad_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PCBData {
    pub success: bool,
    pub board_name: String,
    pub footprints: Vec<Footprint>,
    pub traces: Vec<Trace>,
    pub nets: Vec<Net>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    pub level: String,
    pub message: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CompilationResult {
    pub success: bool,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CompilationEvent {
    pub event_type: String,
    pub data: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ThermalConfig {
    pub ambient_temp: f64,
    pub simulation_time: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ThermalResult {
    pub temperature_grid: Vec<Vec<f64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BlenderConfig {
    pub resolution_x: u32,
    pub resolution_y: u32,
    pub samples: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DRCReport {
    pub success: bool,
    pub violation_count: usize,
    pub violations: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AltiumProject {
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectHandle {
    pub handle_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FileContent {
    pub path: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CargoCheckRequest {
    pub workspace_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CargoCheckResponse {
    pub status: String,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub mod types {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;
    
    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub enum Plan {
        Free,
        Professional,
        Enterprise,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub enum Visibility {
        Public,
        Private,
        Internal,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub struct Message {
        pub role: String,
        pub content: String,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub struct Tenant {
        pub id: Uuid,
        pub name: String,
        pub slug: String,
        pub plan: Plan,
    }
    
    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub struct Project {
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub name: String,
        pub visibility: Visibility,
    }
    
    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    pub enum InferenceRequest {
        Chat { messages: Vec<Message> },
        Completion { prefix: String, suffix: String },
        Tool { tool_name: String, args: serde_json::Value },
    }
}

