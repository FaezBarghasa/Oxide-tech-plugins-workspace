#[cfg(not(target_arch = "wasm32"))]
use serde::{Serialize, Deserialize};

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub role: String, // "user" or "model"
    pub text: String,
}

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
#[serde(rename_all = "camelCase")]
struct GenerationConfig {
    response_mime_type: Option<String>,
    response_schema: Option<serde_json::Value>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Debug)]
struct GeminiRequest {
    contents: Vec<ContentPart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "systemInstruction")]
    system_instruction: Option<SystemInstruction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "generationConfig")]
    generation_config: Option<GenerationConfig>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize, Debug)]
struct GeminiResponseCandidate {
    content: ContentPart,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize, Debug)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiResponseCandidate>>,
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn generate_chat_response(prompt: &str, history: &[ChatMessage]) -> Result<String, anyhow::Error> {
    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();
    if api_key.is_empty() || api_key == "MY_GEMINI_API_KEY" || api_key.trim().is_empty() {
        return Ok(
            "Hello! I am your EDA Assistant. However, the **GEMINI_API_KEY** is not configured.\n\nPlease set your key in the environment to start using the smart circuit design assistant! 🔌⚡".to_string()
        );
    }

    let system_instruction = "You are an expert EDA/PCB design assistant inside an elegant dark-themed EDA suite. Help users programmatically generate circuits, analyze netlists, review design rules, write SKiDL, or optimize track routing. Be concise, precise, and professional. Avoid markdown wrappers or blocks if answering simple commands, and output helpful hints about the workspace when asked.".to_string();

    let mut contents = Vec::new();
    for msg in history {
        contents.push(ContentPart {
            role: if msg.role == "user" { "user".to_string() } else { "model".to_string() },
            parts: vec![Part { text: msg.text.clone() }],
        });
    }
    contents.push(ContentPart {
        role: "user".to_string(),
        parts: vec![Part { text: prompt.to_string() }],
    });

    let client = reqwest::Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.5-flash:generateContent?key={}",
        api_key
    );

    let req_body = GeminiRequest {
        contents,
        system_instruction: Some(SystemInstruction {
            parts: vec![Part { text: system_instruction }],
        }),
        generation_config: None,
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
pub async fn parse_datasheet(datasheet_text: &str) -> Result<serde_json::Value, anyhow::Error> {
    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();
    if api_key.is_empty() || api_key == "MY_GEMINI_API_KEY" || api_key.trim().is_empty() {
        return Err(anyhow::anyhow!("GEMINI_API_KEY not configured."));
    }

    let system_instruction = "You are a professional silicon verification and component librarian. Extract mechanical packaging dimensions, pinouts, and electrical functions. Synthesize coordinates for footprint pads intelligently based on the package (e.g. DIP, SOIC, SOT, QFP). If dimensions are not explicitly listed, make expert approximations based on the standard package type (e.g. DIP pitch is 2.54mm, SOIC pitch is 1.27mm). Keep dimensions in millimeters.".to_string();

    let prompt = format!(
        "Analyze the following datasheet information or pinout block, and extract/synthesize the complete component properties, schematic pins, and PCB footprint dimensions. Spec:\n\n{}",
        datasheet_text
    );

    let schema = serde_json::json!({
        "type": "OBJECT",
        "properties": {
            "name": { "type": "STRING", "description": "Official part name, e.g. NE555, LM358, ESP32-WROOM-32" },
            "value": { "type": "STRING", "description": "Functional description, e.g. 8-Bit MCU, Linear LDO Regulator, Op-Amp" },
            "type": { "type": "STRING", "description": "General component type, e.g. MCU, Regulator, Op-Amp, Sensor, Connector" },
            "referencePrefix": { "type": "STRING", "description": "Standard schematic designator prefix, e.g. U, J, Q" },
            "package": { "type": "STRING", "description": "Synthesized package type, e.g. DIP-8, SOIC-8, SOT-23-5, QFP-32, ESP32" },
            "pins": {
                "type": "ARRAY",
                "items": {
                    "type": "OBJECT",
                    "properties": {
                        "num": { "type": "STRING", "description": "Pin ID number, e.g. '1', '2'" },
                        "name": { "type": "STRING", "description": "Pin formal name, e.g. 'GND', 'VCC', 'RXD'" },
                        "type": { "type": "STRING", "description": "Electrical type: input, output, power, gnd, passive" }
                    },
                    "required": ["num", "name", "type"]
                }
            },
            "dimensions": {
                "type": "OBJECT",
                "properties": {
                    "width": { "type": "NUMBER", "description": "Recommended CAD footprint grid width limit in mm, e.g. 10.0" },
                    "height": { "type": "NUMBER", "description": "Recommended CAD footprint grid height limit in mm, e.g. 8.0" },
                    "pitch": { "type": "NUMBER", "description": "Spacing index between adjacent pins in mm, e.g. 1.27 or 2.54" },
                    "bodyWidth": { "type": "NUMBER", "description": "Mechanical body X-width dimensions in mm, e.g. 6.3" },
                    "bodyLength": { "type": "NUMBER", "description": "Mechanical body Y-length dimensions in mm, e.g. 9.3" },
                    "bodyHeight": { "type": "NUMBER", "description": "Mechanical Z-envelope height limits in mm, e.g. 2.5" },
                    "color": { "type": "STRING", "description": "Hex value for the package body representation, e.g. '#1e1e24'" }
                },
                "required": ["width", "height", "pitch", "bodyWidth", "bodyLength", "bodyHeight", "color"]
            }
        },
        "required": ["name", "value", "type", "referencePrefix", "package", "pins", "dimensions"]
    });

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
            parts: vec![Part { text: system_instruction }],
        }),
        generation_config: Some(GenerationConfig {
            response_mime_type: Some("application/json".to_string()),
            response_schema: Some(schema),
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
                let parsed: serde_json::Value = serde_json::from_str(&part.text)?;
                return Ok(parsed);
            }
        }
    }

    Err(anyhow::anyhow!("No structured JSON response generated from Gemini"))
}
