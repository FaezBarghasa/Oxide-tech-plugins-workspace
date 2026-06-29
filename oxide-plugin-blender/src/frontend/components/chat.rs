#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use crate::frontend::state::CADState;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;
#[cfg(target_arch = "wasm32")]
use serde_json::Value;

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Debug, PartialEq)]
pub struct Message {
    pub sender: String,
    pub text: String,
    pub timestamp: String,
    pub proposed_params: Option<Value>,
}

#[cfg(target_arch = "wasm32")]
fn parse_json_block(text: &str) -> Option<Value> {
    if let Some(start) = text.find("```json") {
        let content_start = start + 7;
        if let Some(end) = text[content_start..].find("```") {
            let json_str = &text[content_start..content_start + end];
            if let Ok(val) = serde_json::from_str::<Value>(json_str.trim()) {
                return Some(val);
            }
        }
    }
    None
}

#[cfg(target_arch = "wasm32")]
#[component]
pub fn ChatPanel() -> Element {
    let mut state: CADState = use_context::<CADState>();
    
    let mut messages = use_signal(|| vec![
        Message {
            sender: "assistant".to_string(),
            text: "Hello! I am your Blender CAD integration assistant. Tell me what kind of electronics you are housing (e.g., Raspberry Pi, custom PCB, Arduino) or the environmental constraints, and I will recommend optimal parametric dimensions and ventilation for your enclosure. 🔧✨".to_string(),
            timestamp: "12:00 PM".to_string(), // placeholder or dynamic
            proposed_params: None,
        }
    ]);

    let mut input = use_signal(|| "".to_string());
    let mut is_typing = use_signal(|| false);

    let mut handle_send = move |text_to_send: String| {
        if text_to_send.trim().is_empty() { return; }

        let user_msg = Message {
            sender: "user".to_string(),
            text: text_to_send.clone(),
            timestamp: "Just now".to_string(),
            proposed_params: None,
        };

        messages.write().push(user_msg);
        input.set("".to_string());
        is_typing.set(true);

        let current_params = state.params.read().clone();
        
        spawn_local(async move {
            let client = reqwest::Client::new();
            let body = serde_json::json!({
                "prompt": text_to_send,
                "currentParams": current_params
            });

            match client.post("/api/chat")
                .json(&body)
                .send()
                .await 
            {
                Ok(resp) => {
                    if resp.status().is_success() {
                        if let Ok(res_data) = resp.json::<serde_json::Value>().await {
                            let text = res_data["text"].as_str().unwrap_or("No response text").to_string();
                            let proposed = parse_json_block(&text);
                            
                            let assistant_msg = Message {
                                sender: "assistant".to_string(),
                                text,
                                timestamp: "Just now".to_string(),
                                proposed_params: proposed,
                            };
                            messages.write().push(assistant_msg);
                        }
                    } else {
                        let assistant_msg = Message {
                            sender: "assistant".to_string(),
                            text: "Server error processing your request.".to_string(),
                            timestamp: "Just now".to_string(),
                            proposed_params: None,
                        };
                        messages.write().push(assistant_msg);
                    }
                }
                Err(err) => {
                    let assistant_msg = Message {
                        sender: "assistant".to_string(),
                        text: format!("Error calling AI client: {:?}", err),
                        timestamp: "Just now".to_string(),
                        proposed_params: None,
                    };
                    messages.write().push(assistant_msg);
                }
            }
            is_typing.set(false);
        });
    };

    let mut apply_proposed_params = move |proposed: Value| {
        let mut new_params = state.params.read().clone();

        if let Some(dims) = proposed.get("dimensions") {
            if let Some(w) = dims.get("width").and_then(|v| v.as_f64()) {
                new_params.dimensions.width = w;
            }
            if let Some(h) = dims.get("height").and_then(|v| v.as_f64()) {
                new_params.dimensions.height = h;
            }
            if let Some(d) = dims.get("depth").and_then(|v| v.as_f64()) {
                new_params.dimensions.depth = d;
            }
        }

        if let Some(wt) = proposed.get("wallThickness").and_then(|v| v.as_f64()) {
            new_params.wall_thickness = wt;
        }

        if let Some(mat) = proposed.get("material").and_then(|v| v.as_str()) {
            new_params.material = mat.to_string();
        }

        if let Some(v_cfg) = proposed.get("ventConfig") {
            if let Some(hd) = v_cfg.get("holeDiameter").and_then(|v| v.as_f64()) {
                new_params.vent_config.hole_diameter = hd;
            }
            if let Some(sp) = v_cfg.get("spacing").and_then(|v| v.as_f64()) {
                new_params.vent_config.spacing = sp;
            }
            if let Some(q) = v_cfg.get("quantity").and_then(|v| v.as_i64()) {
                new_params.vent_config.quantity = q as i32;
            }
        }

        state.update_params(new_params);
    };

    let suggested_prompts = vec![
        "Optimize design for Raspberry Pi 5 thermal airflow",
        "Adjust enclosure to be compact and lightweight",
        "Design a heavy-duty Aluminum heat sink casing",
        "Add moderate ventilation array with high spacing"
    ];

    rsx! {
        div {
            style: "width: 320px; border-left: 1px solid #27272a; background-color: #18181b; display: flex; flex-direction: column; overflow: hidden; height: 100%; box-sizing: border-box;",
            
            // Header
            div {
                style: "padding: 12px 16px; border-bottom: 1px solid #27272a; display: flex; align-items: center; gap: 8px;",
                div {
                    style: "background-color: #27272a; padding: 6px; border-radius: 8px; border: 1px solid #3f3f46; display: flex;",
                    span { style: "font-size: 14px;", "✨" }
                }
                div {
                    h3 { style: "font-size: 12px; font-weight: bold; margin: 0; color: #fff;", "AI CAD Copilot" }
                    p { style: "font-size: 9px; color: #a1a1aa; margin: 2px 0 0 0; font-family: monospace;", "Blender Local Assistant Model" }
                }
            }

            // Messages feed
            div {
                style: "flex-grow: 1; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 12px;",
                for msg in messages.read().iter() {
                    div {
                        style: format!("display: flex; flex-direction: column; max-w: 90%; align-self: {}; align-items: {};",
                            if msg.sender == "user" { "flex-end" } else { "flex-start" },
                            if msg.sender == "user" { "flex-end" } else { "flex-start" }
                        ),
                        span { style: "font-size: 8px; text-transform: uppercase; color: #71717a; font-family: monospace; margin-bottom: 4px;",
                            "{msg.sender} • {msg.timestamp}"
                        }
                        div {
                            style: format!("padding: 10px; border-radius: 12px; font-size: 12px; line-height: 1.4; border: 1px solid {}; background-color: {}; color: {};",
                                if msg.sender == "user" { "#fff" } else { "#09090b" },
                                if msg.sender == "user" { "#fff" } else { "#09090b" },
                                if msg.sender == "user" { "#000" } else { "#fafafa" }
                            ),
                            div { style: "white-space: pre-wrap; word-break: break-word;", "{msg.text}" }
                            
                            // Proposal block
                            if let Some(ref proposed) = msg.proposed_params {
                                div {
                                    style: "margin-top: 8px; padding: 8px; background-color: rgba(255,255,255,0.05); border: 1px solid #27272a; border-radius: 8px; display: flex; flex-direction: column; gap: 4px;",
                                    div { style: "font-size: 9px; color: #fff; font-weight: bold; display: flex; align-items: center; gap: 4px;",
                                        span { "💡" } "OPTIMAL PARAMS FOUND"
                                    }
                                    div { style: "font-size: 8px; font-family: monospace; color: #a1a1aa;",
                                        {
                                            if let Some(dims) = proposed.get("dimensions") {
                                                format!("Size: {}x{}x{}mm", dims.get("width").unwrap_or(&Value::Null), dims.get("height").unwrap_or(&Value::Null), dims.get("depth").unwrap_or(&Value::Null))
                                            } else { "".to_string() }
                                        }
                                    }
                                    if let Some(wt) = proposed.get("wallThickness") {
                                        div { style: "font-size: 8px; font-family: monospace; color: #a1a1aa;", {format!("Wall: {}mm", wt)} }
                                    }
                                    if let Some(vc) = proposed.get("ventConfig") {
                                        div { style: "font-size: 8px; font-family: monospace; color: #a1a1aa;", {format!("Vents: {}x{}", vc.get("quantity").unwrap_or(&Value::Null), vc.get("holeDiameter").unwrap_or(&Value::Null))} }
                                    }
                                    button {
                                        style: "margin-top: 6px; padding: 4px 8px; background-color: #fff; color: #000; border-radius: 6px; font-size: 9px; font-weight: bold; border: none; cursor: pointer;",
                                        onclick: {
                                            let proposed = proposed.clone();
                                            move |_| apply_proposed_params(proposed.clone())
                                        },
                                        "Apply Parameters →"
                                    }
                                }
                            }
                        }
                    }
                }

                if *is_typing.read() {
                    div {
                        style: "align-self: flex-start; max-w: 90%; display: flex; flex-direction: column; align-items: flex-start;",
                        span { style: "font-size: 8px; text-transform: uppercase; color: #71717a; font-family: monospace; margin-bottom: 4px;", "Assistant • Thinking..." }
                        div {
                            style: "background-color: #09090b; border: 1px solid #27272a; padding: 8px 12px; border-radius: 12px; font-size: 12px; color: #a1a1aa; display: flex; align-items: center; gap: 4px;",
                            span { style: "display: flex; gap: 2px;",
                                span { class: "dot-bounce" }
                                span { class: "dot-bounce" }
                                span { class: "dot-bounce" }
                            }
                        }
                    }
                }
            }

            // Suggested prompts
            if messages.read().len() == 1 {
                div {
                    style: "padding: 12px 16px; display: flex; flex-direction: column; gap: 6px;",
                    p { style: "font-size: 8px; text-transform: uppercase; color: #71717a; font-family: monospace; margin: 0;", "Suggested Prompts" }
                    div {
                        style: "display: flex; flex-direction: column; gap: 4px; max-height: 120px; overflow-y: auto;",
                        for prompt_text in suggested_prompts {
                            button {
                                style: "text-align: left; font-size: 10px; background-color: #09090b; border: 1px solid #27272a; color: #a1a1aa; padding: 6px; border-radius: 6px; cursor: pointer; transition: background-color 0.2s;",
                                onclick: move |_| handle_send(prompt_text.to_string()),
                                "{prompt_text}"
                            }
                        }
                    }
                }
            }

            // Input bar
            div {
                style: "padding: 12px 16px; border-top: 1px solid #27272a;",
                form {
                    style: "display: flex; gap: 8px; width: 100%;",
                    prevent_default: "onsubmit",
                    onsubmit: move |_| {
                        let text = input.read().clone();
                        handle_send(text);
                    },
                    input {
                        r#type: "text",
                        value: "{input}",
                        oninput: move |e| input.set(e.value()),
                        placeholder: "Ask AI assistant...",
                        style: "flex-grow: 1; padding: 6px 10px; background-color: #09090b; border: 1px solid #27272a; border-radius: 8px; color: #fff; font-size: 12px; outline: none; width: 100%; box-sizing: border-box;"
                    }
                    button {
                        r#type: "submit",
                        style: "padding: 6px 10px; background-color: #fff; color: #000; font-size: 12px; border-radius: 8px; border: none; cursor: pointer;",
                        "Send"
                    }
                }
            }
        }
    }
}
