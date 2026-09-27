use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ApiError {}

impl ApiError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}

#[derive(Debug)]
pub enum OxideError {
    Api(ApiError),
    Database(String),
    NotFound(String),
    Validation(String),
    Timeout,
    Internal(String),
}

impl fmt::Display for OxideError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Api(e) => write!(f, "API Error: {}", e),
            Self::Database(e) => write!(f, "Database Error: {}", e),
            Self::NotFound(e) => write!(f, "Not Found: {}", e),
            Self::Validation(e) => write!(f, "Validation Error: {}", e),
            Self::Timeout => write!(f, "Request Timeout"),
            Self::Internal(e) => write!(f, "Internal Error: {}", e),
        }
    }
}

impl std::error::Error for OxideError {}
