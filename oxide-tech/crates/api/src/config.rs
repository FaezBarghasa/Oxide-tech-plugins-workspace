use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub surrealdb_url: String,
    pub surrealdb_user: String,
    pub surrealdb_pass: String,
    pub qdrant_url: String,
    pub vllm_endpoint: String,
    pub db_pool_size: u16,
    pub log_level: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            surrealdb_url: env::var("SURREALDB_URL")
                .unwrap_or_else(|_| "ws://127.0.0.1:8000".into()),
            surrealdb_user: env::var("SURREALDB_USER").unwrap_or_else(|_| "root".into()),
            surrealdb_pass: env::var("SURREALDB_PASS").unwrap_or_else(|_| "root".into()),
            qdrant_url: env::var("QDRANT_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:6333".into()),
            vllm_endpoint: env::var("VLLM_ENDPOINT")
                .unwrap_or_else(|_| "http://127.0.0.1:8000".into()),
            db_pool_size: env::var("DB_POOL_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            log_level: env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        })
    }
}
