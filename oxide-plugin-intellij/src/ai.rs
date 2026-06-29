#[cfg(not(target_arch = "wasm32"))]
use serde::{Serialize, Deserialize};

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Debug)]
struct Part {
    text: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Debug)]
struct ContentPart {
    role: String,
    parts: Vec<Part>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Debug)]
struct SystemInstruction {
    parts: Vec<Part>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Debug)]
struct GeminiRequest {
    contents: Vec<ContentPart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "systemInstruction")]
    system_instruction: Option<SystemInstruction>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize, Debug)]
struct GeminiResponseCandidate {
    content: ContentPartResponse,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize, Debug)]
struct ContentPartResponse {
    parts: Vec<Part>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize, Debug)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiResponseCandidate>>,
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn generate_chat_response(
    prompt: &str,
    history: &[(String, String)], // (role, content)
    mode: &str,
) -> Result<String, anyhow::Error> {
    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();
    if api_key.is_empty() || api_key == "MY_GEMINI_API_KEY" || api_key.trim().is_empty() {
        return Ok(
            "Hello! I am the Oxide-Tech Local Agent assistant. However, the **GEMINI_API_KEY** is not configured.\n\nPlease set your key in the **Secrets** panel in Google AI Studio to start using the smart CAD prompt design assistant! 💻🔧".to_string()
        );
    }

    let system_instruction = if mode == "planning" {
        "You are an expert embedded software architect and systems planner. Your role is to help the user plan firmware modules, design Embassy async executors, specify RTIC resources, design peripheral DMA pipelines, and map out register maps. Keep your tone highly analytical, and outline step-by-step implementation plans."
    } else {
        "You are an expert embedded software execution engineer. Your role is to write clean, bare-metal Rust code, optimize compiler flags, configure cargo targets, and debug OpenOCD register dumps. Provide ready-to-use production-grade code snippets targeting #![no_std] environments."
    };

    let client = reqwest::Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.5-flash:generateContent?key={}",
        api_key
    );

    let mut contents = Vec::new();
    for (role, text) in history {
        contents.push(ContentPart {
            role: if role == "User" { "user".to_string() } else { "model".to_string() },
            parts: vec![Part { text: text.clone() }],
        });
    }
    // Append current prompt
    contents.push(ContentPart {
        role: "user".to_string(),
        parts: vec![Part { text: prompt.to_string() }],
    });

    let req_body = GeminiRequest {
        contents,
        system_instruction: Some(SystemInstruction {
            parts: vec![Part { text: system_instruction.to_string() }],
        }),
    };

    let res = client
        .post(&url)
        .header("User-Agent", "aistudio-build")
        .json(&req_body)
        .send()
        .await?;

    if !res.status().is_success() {
        let err_text = res.text().await?;
        return Err(anyhow::anyhow!("Gemini API error: {}", err_text));
    }

    let gemini_res: GeminiResponse = res.json().await?;
    if let Some(candidates) = gemini_res.candidates {
        if let Some(candidate) = candidates.first() {
            if let Some(part) = candidate.content.parts.first() {
                return Ok(part.text.clone());
            }
        }
    }

    Err(anyhow::anyhow!("No text response generated from Gemini"))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn audit_code_pr(
    code: &str,
    title: &str,
    description: &str,
) -> Result<String, anyhow::Error> {
    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();
    if api_key.is_empty() || api_key == "MY_GEMINI_API_KEY" || api_key.trim().is_empty() {
        // Return a mock static check response if API key is missing
        return Ok(r#"{
            "approved": false,
            "score": 60,
            "summary": "Mock audit completed: GEMINI_API_KEY is missing. Code contains standard library imports which violate bare-metal requirements.",
            "issues": [
                {
                    "severity": "Critical",
                    "line": 3,
                    "title": "Standard Library Namespace Detected",
                    "explanation": "The import 'use std::collections::VecDeque;' references the standard library, which is unavailable in #![no_std] bare-metal systems.",
                    "recommendation": "Use alloc::collections::VecDeque or a fixed-size stack-allocated queue like heapless::Deque."
                }
            ]
        }"#.to_string());
    }

    let system_instruction = "You are an automated PR review bot for embedded bare-metal projects. Your job is to audit incoming Rust code snippets for compliance with #![no_std] constraints, blocking loops, memory leaks, and hardware constraints. You must respond ONLY with a raw, valid JSON object matching the requested schema. Do not output any markdown formatting like ```json or any other text before/after the JSON.";

    let prompt = format!(
        "Audit the following pull request code snippet.\n\
        PR Title: {}\n\
        PR Description: {}\n\n\
        Code:\n\
        ```rust\n\
        {}\n\
        ```\n\n\
        You must analyze it and output exactly a JSON object with this schema:\n\
        {{\n\
          \"approved\": bool,\n\
          \"score\": u32 (0-100),\n\
          \"summary\": \"Overall summary string\",\n\
          \"issues\": [\n\
            {{\n\
              \"severity\": \"Critical\" | \"Warning\" | \"Info\",\n\
              \"line\": u32 (or null),\n\
              \"title\": \"Issue Title\",\n\
              \"explanation\": \"Detailed explanation\",\n\
              \"recommendation\": \"How to fix it\"\n\
            }}\n\
          ]\n\
        }}",
        title, description, code
    );

    let client = reqwest::Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.5-flash:generateContent?key={}",
        api_key
    );

    let req_body = GeminiRequest {
        contents: vec![ContentPart {
            role: "user".to_string(),
            parts: vec![Part { text: prompt }],
        }],
        system_instruction: Some(SystemInstruction {
            parts: vec![Part { text: system_instruction.to_string() }],
        }),
    };

    let res = client
        .post(&url)
        .header("User-Agent", "aistudio-build")
        .json(&req_body)
        .send()
        .await?;

    if !res.status().is_success() {
        let err_text = res.text().await?;
        return Err(anyhow::anyhow!("Gemini API error: {}", err_text));
    }

    let gemini_res: GeminiResponse = res.json().await?;
    if let Some(candidates) = gemini_res.candidates {
        if let Some(candidate) = candidates.first() {
            if let Some(part) = candidate.content.parts.first() {
                let text = part.text.trim();
                // Strip markdown backticks if any
                let clean_json = if text.starts_with("```") {
                    let mut lines = text.lines().collect::<Vec<_>>();
                    if lines.first().unwrap().starts_with("```") {
                        lines.remove(0);
                    }
                    if lines.last().unwrap().starts_with("```") {
                        lines.pop();
                    }
                    lines.join("\n")
                } else {
                    text.to_string()
                };
                return Ok(clean_json);
            }
        }
    }

    Err(anyhow::anyhow!("No audit response generated from Gemini"))
}
