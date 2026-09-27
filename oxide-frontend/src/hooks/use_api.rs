use dioxus::prelude::*;
use shared::{CargoCheckRequest, CargoCheckResponse};

pub struct ApiClient {
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
    
    pub async fn cargo_check(&self, workspace_path: String) -> Result<CargoCheckResponse, String> {
        let url = format!("{}/api/cargo/check", self.base_url);
        
        // Mocking for now since shared might not have CargoCheckRequest yet, 
        // but according to prompt it expects it to be there.
        // Let's implement it.
        
        let response = reqwest::Client::new()
            .post(&url)
            .json(&CargoCheckRequest { workspace_path })
            .send()
            .await
            .map_err(|e| e.to_string())?;
        
        response.json().await.map_err(|e| e.to_string())
    }
}

pub fn use_api() -> ApiClient {
    let backend_url = std::option_env!("BACKEND_URL")
        .unwrap_or("http://127.0.0.1:8080")
        .to_string();
    
    ApiClient::new(backend_url)
}
