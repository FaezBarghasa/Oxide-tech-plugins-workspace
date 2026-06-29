use serde::{Deserialize, Serialize};
use dioxus::prelude::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditorState {
    pub current_file: Option<String>,
    pub content: String,
    pub cursor_line: u32,
    pub cursor_column: u32,
    pub unsaved: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompilationState {
    pub status: String,
    pub diagnostics: Vec<CompilationDiagnostic>,
    pub output: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompilationDiagnostic {
    pub level: String,
    pub message: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub file_path: Option<String>,
}

#[derive(Debug, Clone)]
pub enum EditorSignal {
    Open(String),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SchematicComponent {
    pub reference: String,
    pub value: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub source: String,
    pub target: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Net {
    pub name: String,
    pub connections: Vec<Connection>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SchematicData {
    pub components: Vec<SchematicComponent>,
    pub nets: Vec<Net>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Pad {
    pub name: String,
    pub net: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Footprint {
    pub reference: String,
    pub value: String,
    pub x: f64,
    pub y: f64,
    pub orientation: f64,
    pub layer: String,
    pub pads: Vec<Pad>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Trace {
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
    pub net: String,
    pub width: f64,
    pub layer: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PCBBounds {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

impl Default for PCBBounds {
    fn default() -> Self {
        Self {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 120.0,
            max_y: 120.0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PCBData {
    pub board_name: String,
    pub footprints: Vec<Footprint>,
    pub traces: Vec<Trace>,
    pub board_bounds: PCBBounds,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BoardSyncRequest {
    pub filename: String,
    pub layer_count: i32,
    pub thermal_metrics: serde_json::Value,
    pub dimensions: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct DesignCheckError {
    pub r#type: String,
    pub severity: String,
    pub net_name: Option<String>,
    pub details: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct DesignCheckReport {
    pub timestamp: String,
    pub error_count: i32,
    pub warning_count: i32,
    pub errors: Vec<DesignCheckError>,
}

#[derive(Clone, Copy)]
pub struct KiCadState {
    pub schematic: Signal<SchematicData>,
    pub pcb: Signal<PCBData>,
    pub board_sync: Signal<Option<BoardSyncRequest>>,
    pub drc_report: Signal<Option<DesignCheckReport>>,
    pub active_panel: Signal<String>,
    pub selected_ref: Signal<Option<String>>,
    pub search_query: Signal<String>,
    pub is_loading: Signal<bool>,
    pub error_msg: Signal<Option<String>>,
    pub visible_layers: Signal<std::collections::HashSet<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VentConfig {
    pub hole_diameter: f64,
    pub spacing: f64,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnclosureParams {
    pub dimensions: Dimensions,
    pub wall_thickness: f64,
    pub material: String,
    pub vent_config: VentConfig,
}

impl Default for EnclosureParams {
    fn default() -> Self {
        Self {
            dimensions: Dimensions {
                width: 100.0,
                height: 80.0,
                depth: 60.0,
            },
            wall_thickness: 2.5,
            material: "pla".to_string(),
            vent_config: VentConfig {
                hole_diameter: 3.0,
                spacing: 5.0,
                quantity: 12,
            },
        }
    }
}

#[derive(Clone, Copy)]
pub struct CADState {
    pub params: Signal<EnclosureParams>,
    pub history: Signal<Vec<EnclosureParams>>,
    pub history_idx: Signal<usize>,
    pub is_generating: Signal<bool>,
    pub error_msg: Signal<Option<String>>,
    pub is_modal_open: Signal<bool>,
}
