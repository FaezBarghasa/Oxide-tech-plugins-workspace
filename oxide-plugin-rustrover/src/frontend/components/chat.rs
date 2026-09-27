#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use crate::frontend::state::{IntellijState, Message};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[cfg(target_arch = "wasm32")]
#[component]
pub fn ChatPanel() -> Element {
    let mut state: IntellijState = use_context::<IntellijState>();

    let mut input_text = use_signal(|| "".to_string());
    let mut mode = state.chat_mode;
    let messages = state.chat_messages.read().clone();
    let loading = *state.is_loading.read();

    let mut send_message = move || {
        let text = input_text.read().clone();
        if text.trim().is_empty() || loading {
            return;
        }

        // Add user message to history
        let mut history = state.chat_messages.read().clone();
        history.push(Message {
            role: "User".to_string(),
            content: text.clone(),
        });
        state.chat_messages.set(history.clone());
        input_text.set("".to_string());
        state.is_loading.set(true);

        let chat_m = mode.read().clone();
        spawn_local(async move {
            let client = reqwest::Client::new();
            let payload = serde_json::json!({
                "prompt": text,
                "history": history,
                "mode": chat_m
            });

            if let Ok(resp) = client.post("/api/chat").json(&payload).send().await {
                if resp.status().is_success() {
                    if let Ok(json_res) = resp.json::<serde_json::Value>().await {
                        if let Some(reply_text) = json_res["text"].as_str() {
                            let mut current_history = state.chat_messages.read().clone();
                            current_history.push(Message {
                                role: "Model".to_string(),
                                content: reply_text.to_string(),
                            });
                            state.chat_messages.set(current_history);
                        }
                    }
                }
            }
            state.is_loading.set(false);
        });
    };

    rsx! {
        div {
            style: "background: rgba(17, 21, 32, 0.7); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 16px; overflow: hidden; display: flex; flex-direction: column; flex-grow: 1; min-height: 480px; shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5); height: 100%; max-height: calc(100vh - 120px);",
            
            // Header
            div {
                style: "background: rgba(15, 19, 34, 0.6); border-bottom: 1px solid rgba(255, 255, 255, 0.06); padding: 12px 16px; display: flex; justify-content: space-between; align-items: center;",
                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    span { style: "width: 8px; height: 8px; border-radius: 50%; background: #06b6d4; display: inline-block; box-shadow: 0 0 8px #06b6d4;" }
                    h2 { style: "font-size: 13px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: #f1f5f9; margin: 0;", "Gemini assistant" }
                }
                
                // Mode Select
                div {
                    style: "display: flex; align-items: center; gap: 6px;",
                    span { style: "font-size: 10px; color: #64748b; font-family: monospace;", "MODE:" }
                    select {
                        style: "background: #0f1322; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 4px 8px; font-size: 10px; color: #22d3ee; font-weight: 600; font-family: monospace; cursor: pointer;",
                        value: "{mode}",
                        onchange: move |e| mode.set(e.value().clone()),
                        option { value: "executing", "EX-CODING (Code snippet generation)" }
                        option { value: "planning", "PLANNING (Architectural outline)" }
                    }
                }
            }

            // Chat Messages Container
            div {
                style: "flex-grow: 1; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 14px; background: rgba(7, 9, 15, 0.2);",
                
                {messages.iter().map(|msg| {
                    let is_model = msg.role == "Model";
                    let bubble_align = if is_model { "align-self: flex-start; text-align: left;" } else { "align-self: flex-end; text-align: left;" };
                    let bubble_bg = if is_model { "background: rgba(15, 19, 34, 0.5); border: 1px solid rgba(255, 255, 255, 0.04); color: #cbd5e1;" } else { "background: rgba(6, 182, 212, 0.12); border: 1px solid rgba(6, 182, 212, 0.25); color: #e2e8f0;" };
                    let role_name = if is_model { "CO-PILOT" } else { "USER" };
                    let role_color = if is_model { "#38bdf8" } else { "#14b8a6" };

                    rsx! {
                        div {
                            key: "{msg.content}",
                            style: "{bubble_align} max-width: 85%; display: flex; flex-direction: column; gap: 4px;",
                            div {
                                style: "font-family: monospace; font-size: 8px; color: {role_color}; font-weight: bold; letter-spacing: 0.05em;",
                                "{role_name}"
                            }
                            div {
                                style: "{bubble_bg} padding: 12px 14px; border-radius: 12px; font-size: 11px; line-height: 1.5; white-space: pre-wrap;",
                                "{msg.content}"
                            }
                        }
                    }
                })}

                if loading {
                    div {
                        style: "align-self: flex-start; display: flex; flex-direction: column; gap: 4px;",
                        div { style: "font-family: monospace; font-size: 8px; color: #38bdf8; font-weight: bold;", "CO-PILOT" }
                        div {
                            style: "background: rgba(15, 19, 34, 0.5); border: 1px solid rgba(255, 255, 255, 0.04); color: #94a3b8; padding: 12px 14px; border-radius: 12px; font-size: 11px; display: flex; align-items: center; gap: 6px;",
                            span { style: "font-family: monospace; animate-pulse-subtle", "Thinking..." }
                        }
                    }
                }
            }

            // Input Area
            div {
                style: "background: rgba(15, 19, 34, 0.8); border-top: 1px solid rgba(255, 255, 255, 0.06); padding: 12px; display: flex; gap: 8px; align-items: center;",
                input {
                    style: "background: #07090f; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 8px; padding: 10px; font-size: 11px; color: #ffffff; flex-grow: 1;",
                    placeholder: "Ask to generate async drivers, embassy schedules, review register mnemonics...",
                    value: "{input_text}",
                    oninput: move |e| input_text.set(e.value().clone()),
                    onkeydown: move |e| {
                        if e.key() == Key::Enter {
                            send_message();
                        }
                    }
                }
                button {
                    style: "background: #0891b2; color: #ffffff; border: none; padding: 10px 16px; border-radius: 8px; font-size: 11px; font-weight: 700; cursor: pointer; transition: all 0.2s;",
                    disabled: loading,
                    onclick: move |_| send_message(),
                    "Send"
                }
            }
        }
    }
}
