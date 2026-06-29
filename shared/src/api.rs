use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoCheckRequest {
    pub workspace_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoCheckResponse {
    pub status: String,
    pub diagnostics: Vec<CompilationDiagnostic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationDiagnostic {
    pub level: String,
    pub message: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub file_path: Option<String>,
}
