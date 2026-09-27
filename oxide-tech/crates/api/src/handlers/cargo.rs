use actix_web::{HttpResponse, web};
use shared::{CargoCheckRequest, CargoCheckResponse, CompilationDiagnostic};
use std::process::Command;
use std::time::Instant;
use std::path::Path;

fn is_safe_workspace_path(path_str: &str) -> bool {
    let path = Path::new(path_str);
    
    // Resolve the path fully
    if let Ok(canonical) = path.canonicalize() {
        // Must exist and be a directory
        if !canonical.is_dir() {
            return false;
        }
        
        // As a security best practice, restrict path execution to user projects / workspace directories.
        // We enforce that it does not escape to root-level system folders like /etc, /usr, /var etc.
        let path_str = canonical.to_string_lossy();
        if path_str.starts_with("/etc") || path_str.starts_with("/usr") || path_str.starts_with("/var") || path_str.starts_with("/boot") {
            return false;
        }
        
        return true;
    }
    
    false
}

pub async fn check(req: web::Json<CargoCheckRequest>) -> HttpResponse {
    let start = Instant::now();
    
    if !is_safe_workspace_path(&req.workspace_path) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid or unauthorized workspace path provided"
        }));
    }

    let workspace_dir = req.workspace_path.clone();
    let output = web::block(move || {
        Command::new("cargo")
            .arg("check")
            .arg("--message-format=json")
            .current_dir(&workspace_dir)
            .output()
    })
    .await;

    match output {
        Ok(Ok(out)) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let diagnostics = parse_cargo_diagnostics(&stdout);
            
            HttpResponse::Ok().json(CargoCheckResponse {
                success: out.status.success(),
                diagnostics,
                elapsed_ms: start.elapsed().as_millis() as u64,
            })
        }
        Ok(Err(e)) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string()
            }))
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Task execution failed"
            }))
        }
    }
}

pub async fn clippy(req: web::Json<CargoCheckRequest>) -> HttpResponse {
    if !is_safe_workspace_path(&req.workspace_path) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid or unauthorized workspace path provided"
        }));
    }

    let workspace_dir = req.workspace_path.clone();
    let output = web::block(move || {
        Command::new("cargo")
            .arg("clippy")
            .arg("--message-format=json")
            .current_dir(&workspace_dir)
            .output()
    })
    .await;

    match output {
        Ok(Ok(out)) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let diagnostics = parse_cargo_diagnostics(&stdout);
            
            HttpResponse::Ok().json(CargoCheckResponse {
                success: out.status.success(),
                diagnostics,
                elapsed_ms: 0,
            })
        }
        _ => HttpResponse::InternalServerError().json(serde_json::json!({"error": "Clippy failed"}))
    }
}

fn parse_cargo_diagnostics(output: &str) -> Vec<CompilationDiagnostic> {
    let mut diagnostics = Vec::new();
    
    for line in output.lines() {
        if let Ok(msg) = serde_json::from_str::<serde_json::Value>(line) {
            if let Some(message) = msg.get("message") {
                if let Some(level) = message.get("level") {
                    if let Some(spans) = message.get("spans").and_then(|s| s.as_array()) {
                        if let Some(span) = spans.first() {
                            diagnostics.push(CompilationDiagnostic {
                                level: level.as_str().unwrap_or("note").to_string(),
                                message: message.get("message")
                                    .and_then(|m| m.as_str())
                                    .unwrap_or("Unknown error")
                                    .to_string(),
                                file_path: span.get("file_name")
                                    .and_then(|f| f.as_str())
                                    .map(|s| s.to_string()),
                                line: span.get("line_start").and_then(|l| l.as_u64()).map(|l| l as u32),
                                column: span.get("column_start").and_then(|c| c.as_u64()).map(|c| c as u32),
                            });
                        }
                    }
                }
            }
        }
    }
    
    diagnostics
}
