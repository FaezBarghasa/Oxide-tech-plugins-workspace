#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use crate::frontend::state::KiCadState;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Debug, PartialEq)]
pub struct ChatMessage {
    pub role: String, // "user" or "model"
    pub text: String,
    pub timestamp: String,
}

#[cfg(target_arch = "wasm32")]
#[component]
pub fn ChatPanel() -> Element {
    let _state: KiCadState = use_context::<KiCadState>();
    
    let mut messages = use_signal(|| vec![
        ChatMessage {
            role: "model".to_string(),
            text: "Hello! I am your AI Design Assistant. Ask me to draft SKiDL code, optimize layer stackups, analyze connectivity, or review DRC constraints. 🔌⚡".to_string(),
            timestamp: "12:00 PM".to_string(),
        }
    ]);
    
    let mut input = use_signal(|| "".to_string());
    let mut is_loading = use_signal(|| false);

    let mut handle_send = move |text_to_send: String| {
        if text_to_send.trim().is_empty() || *is_loading.read() { return; }

        let user_msg = ChatMessage {
            role: "user".to_string(),
            text: text_to_send.clone(),
            timestamp: "Just now".to_string(),
        };

        messages.write().push(user_msg);
        input.set("".to_string());
        is_loading.set(true);

        // Map history to simplified format required by server
        let history_payload: Vec<serde_json::Value> = messages.read().iter().map(|m| {
            serde_json::json!({
                "role": m.role,
                "text": m.text
            })
        }).collect();

        spawn_local(async move {
            let client = reqwest::Client::new();
            let body = serde_json::json!({
                "message": text_to_send,
                "history": history_payload
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
                            let model_msg = ChatMessage {
                                role: "model".to_string(),
                                text,
                                timestamp: "Just now".to_string(),
                            };
                            messages.write().push(model_msg);
                        }
                    } else {
                        messages.write().push(ChatMessage {
                            role: "model".to_string(),
                            text: "Assistant offline. Response error.".to_string(),
                            timestamp: "Just now".to_string(),
                        });
                    }
                }
                Err(err) => {
                    messages.write().push(ChatMessage {
                        role: "model".to_string(),
                        text: format!("Error calling design agent: {:?}", err),
                        timestamp: "Just now".to_string(),
                    });
                }
            }
            is_loading.set(false);
        });
    };

    let suggestion_chips = vec![
        "Write SKiDL bypass circuit",
        "Design 4-layer stackup",
        "Identify unrouted traces",
        "Check power trace width"
    ];

    rsx! {
        div {
            style: "width: 320px; border-left: 1px solid #27272a; background-color: #18181b; display: flex; flex-direction: column; overflow: hidden; height: 100%; box-sizing: border-box;",
            
            // Header
            div {
                style: "padding: 12px 16px; border-bottom: 1px solid #27272a; display: flex; align-items: center; justify-content: space-between;",
                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    span { style: "font-size: 14px;", "✨" }
                    h3 { style: "font-size: 11px; font-weight: bold; margin: 0; color: #fff;", "Lattice Cohort AI" }
                }
                if messages.read().len() > 1 {
                    button {
                        onclick: move |_| {
                            let first_msg = messages.read()[0].clone();
                            messages.set(vec![first_msg]);
                        },
                        style: "background-color: transparent; border: none; color: #71717a; cursor: pointer; font-size: 10px; hover:color: #fff;",
                        "Clear"
                    }
                }
            }

            // Messages feed
            div {
                style: "flex-grow: 1; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 12px;",
                for msg in messages.read().iter() {
                    div {
                        style: format!("display: flex; flex-direction: column; max-w: 90%; align-self: {}; align-items: {};",
                            if msg.role == "user" { "flex-end" } else { "flex-start" },
                            if msg.role == "user" { "flex-end" } else { "flex-start" }
                        ),
                        span { style: "font-size: 8px; text-transform: uppercase; color: #71717a; font-family: monospace; margin-bottom: 4px;",
                            "{msg.role} • {msg.timestamp}"
                        }
                        div {
                            style: format!("padding: 10px; border-radius: 12px; font-size: 11px; line-height: 1.4; border: 1px solid {}; background-color: {}; color: {};",
                                if msg.role == "user" { "#14b8a6" } else { "#09090b" },
                                if msg.role == "user" { "#14b8a6" } else { "#09090b" },
                                if msg.role == "user" { "#fff" } else { "#fafafa" }
                            ),
                            div { style: "white-space: pre-wrap; word-break: break-word;", "{msg.text}" }
                        }
                    }
                }

                if *is_loading.read() {
                    div {
                        style: "align-self: flex-start; max-w: 90%; display: flex; flex-direction: column; align-items: flex-start;",
                        span { style: "font-size: 8px; text-transform: uppercase; color: #71717a; font-family: monospace; margin-bottom: 4px;", "Assistant • Thinking..." }
                        div {
                            style: "background-color: #09090b; border: 1px solid #27272a; padding: 8px 12px; border-radius: 12px; font-size: 11px; color: #a1a1aa; display: flex; align-items: center; gap: 6px;",
                            span { style: "display: flex; gap: 2px;",
                                span { class: "dot-bounce" }
                                span { class: "dot-bounce" }
                                span { class: "dot-bounce" }
                            }
                            "Processing specs..."
                        }
                    }
                }
            }

            // Suggestion chips
            if messages.read().len() == 1 && !*is_loading.read() {
                div {
                    style: "padding: 12px 16px; display: flex; flex-direction: column; gap: 6px;",
                    p { style: "font-size: 8px; text-transform: uppercase; color: #71717a; font-family: monospace; margin: 0;", "Suggested Tasks" }
                    div {
                        style: "display: grid; grid-template-columns: 1fr 1fr; gap: 6px;",
                        for prompt_text in suggestion_chips {
                            button {
                                style: "text-align: left; font-size: 10px; background-color: #09090b; border: 1px solid #27272a; color: #a1a1aa; padding: 6px; border-radius: 6px; cursor: pointer; transition: all 0.2s; max-width: 100%; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                                onclick: move |_| handle_send(prompt_text.to_string()),
                                "{prompt_text}"
                            }
                        }
                    }
                }
            }

            // Input bar
            div {
                style: "padding: 12px 16px; border-top: 1px solid #27272a; background-color: #0f0f11;",
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
                        placeholder: "Ask assistant...",
                        style: "flex-grow: 1; padding: 6px 10px; background-color: #09090b; border: 1px solid #27272a; border-radius: 8px; color: #fff; font-size: 11px; outline: none; width: 100%; box-sizing: border-box;"
                    }
                    button {
                        r#type: "submit",
                        style: "padding: 6px 10px; background-color: #14b8a6; color: #fff; font-size: 11px; font-weight: bold; border-radius: 8px; border: none; cursor: pointer;",
                        "Send"
                    }
                }
            }
        }
    }
}
