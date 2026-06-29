#[cfg(not(target_arch = "wasm32"))]
use serde::{Serialize, Deserialize};
#[cfg(not(target_arch = "wasm32"))]
use crate::db::EnclosureParams;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Debug)]
struct Part {
    text: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Debug)]
struct ContentPart {
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
    content: ContentPart,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize, Debug)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiResponseCandidate>>,
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn generate_chat_response(prompt: &str, current_params: &EnclosureParams) -> Result<String, anyhow::Error> {
    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();
    if api_key.is_empty() || api_key == "MY_GEMINI_API_KEY" || api_key.trim().is_empty() {
        return Ok(
            "Hello! I would love to help you design your enclosure. However, the **GEMINI_API_KEY** is not configured.\n\nPlease set your key in the **Secrets** panel in Google AI Studio to start using the smart CAD prompt design assistant! 💻🔧".to_string()
        );
    }

    let system_instruction = format!(
        "You are an expert mechanical design AI Assistant integrated directly into the Tauri CAD Control Plane for Blender.\n\
        Your goal is to help user design parametric 3D printed/milled electronics enclosures.\n\
        The user is working with the following parameters:\n\
        - Width: {}mm\n\
        - Height: {}mm\n\
        - Depth: {}mm\n\
        - Wall Thickness: {}mm\n\
        - Material: {}\n\
        - Vent hole diameter: {}mm\n\
        - Vent hole spacing: {}mm\n\
        - Vent hole quantity: {}\n\n\
        You can suggest specific design changes. To apply parameters automatically, you can respond with a JSON block in your answer containing the new exact parameters so the frontend can parse it and update the model instantly!\n\
        JSON block format:\n\
        ```json\n\
        {{\n\
          \"dimensions\": {{ \"width\": 120, \"height\": 90, \"depth\": 50 }},\n\
          \"wallThickness\": 3.0,\n\
          \"material\": \"pla\",\n\
          \"ventConfig\": {{ \"holeDiameter\": 4, \"spacing\": 6, \"quantity\": 16 }}\n\
        }}\n\
        ```\n\
        Make sure any parameters you propose are mathematically valid:\n\
        - Width, height, depth must be between 50mm and 500mm\n\
        - Wall thickness between 1.5mm and 10mm\n\
        - Vent hole diameter between 2mm and 50mm\n\n\
        Keep your response friendly, concise, and professional. Explain *why* you suggest the sizes (e.g., thermal airflow, mechanical strength, structure fitting, material efficiency, PLA vs Aluminum).",
        current_params.dimensions.width,
        current_params.dimensions.height,
        current_params.dimensions.depth,
        current_params.wall_thickness,
        current_params.material,
        current_params.vent_config.hole_diameter,
        current_params.vent_config.spacing,
        current_params.vent_config.quantity
    );

    let client = reqwest::Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.5-flash:generateContent?key={}",
        api_key
    );

    let req_body = GeminiRequest {
        contents: vec![ContentPart {
            parts: vec![Part {
                text: prompt.to_string(),
            }],
        }],
        system_instruction: Some(SystemInstruction {
            parts: vec![Part {
                text: system_instruction,
            }],
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
