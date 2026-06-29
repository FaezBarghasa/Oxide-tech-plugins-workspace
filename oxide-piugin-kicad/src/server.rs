#[cfg(not(target_arch = "wasm32"))]
use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
#[cfg(not(target_arch = "wasm32"))]
use crate::db;
#[cfg(not(target_arch = "wasm32"))]
use crate::routes;

#[cfg(not(target_arch = "wasm32"))]
#[get("/{tail:.*}")]
pub async fn serve_static(path: web::Path<String>) -> impl Responder {
    let tail = path.into_inner();
    let mut file_path = PathBuf::from("dist");
    
    if tail.is_empty() || tail == "/" {
        file_path.push("index.html");
    } else {
        file_path.push(&tail);
    }
    
    if !file_path.exists() {
        file_path = PathBuf::from("dist/index.html");
    }
    
    match std::fs::read(&file_path) {
        Ok(content) => {
            let mime_type = match file_path.extension().and_then(|s| s.to_str()) {
                Some("html") => "text/html",
                Some("js") => "application/javascript",
                Some("wasm") => "application/wasm",
                Some("css") => "text/css",
                Some("svg") => "image/svg+xml",
                Some("png") => "image/png",
                _ => "application/octet-stream",
            };
            HttpResponse::Ok()
                .content_type(mime_type)
                .body(content)
        }
        Err(_) => HttpResponse::NotFound().body("Not Found"),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn run() -> std::io::Result<()> {
    if let Err(e) = db::init().await {
        eprintln!("Failed to initialize SurrealDB for KiCad: {:?}", e);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()));
    }
    
    println!("SurrealDB v3 local engine started for KiCad.");
    println!("Starting KiCad Actix Web server on http://localhost:8085");
    
    HttpServer::new(|| {
        App::new()
            .configure(routes::configure)
            .service(serve_static)
    })
    .bind(("0.0.0.0", 8085))?
    .run()
    .await
}
