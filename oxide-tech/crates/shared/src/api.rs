use serde::{Deserialize, Serialize};
use super::models::*;

// ============= CARGO API =============

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoCheckRequest {
    pub workspace_path: String,
    pub target_triple: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoCheckResponse {
    pub success: bool,
    pub diagnostics: Vec<CompilationDiagnostic>,
    pub elapsed_ms: u64,
}

// ============= TREE-SITTER API =============

#[derive(Debug, Serialize, Deserialize)]
pub struct AstParseRequest {
    pub file_path: String,
    pub source_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AstNode {
    pub node_type: String,
    pub text: String,
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
    pub children: Vec<AstNode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AstParseResponse {
    pub success: bool,
    pub root: Option<AstNode>,
    pub error: Option<String>,
}

// ============= KICAD API =============

#[derive(Debug, Serialize, Deserialize)]
pub struct KicadLoadBoardRequest {
    pub board_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PcbData {
    pub board_name: String,
    pub components: Vec<Component>,
    pub nets: Vec<Net>,
    pub traces: Vec<Trace>,
    pub board_bounds: BoardBounds,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BoardBounds {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

// ============= THERMAL API =============

#[derive(Debug, Serialize, Deserialize)]
pub struct ThermalSimulationRequest {
    pub board_path: String,
    pub power_map: Vec<(f64, f64, f64)>, // (x, y, power_watts)
    pub ambient_temp: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThermalResult {
    pub temperature_grid: Vec<Vec<f64>>,
    pub max_temperature: f64,
    pub min_temperature: f64,
    pub elapsed_ms: u64,
}

// ============= AGENT API =============

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentRequest {
    pub task: String,
    pub context: String,
    pub constraints: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentResponse {
    pub success: bool,
    pub result: String,
    pub artifacts: Vec<Artifact>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Artifact {
    pub artifact_type: String,
    pub content: String,
    pub path: Option<String>,
}
