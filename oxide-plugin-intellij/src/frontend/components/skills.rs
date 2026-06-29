#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use crate::frontend::state::{IntellijState, AutomationSkill};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[cfg(target_arch = "wasm32")]
#[component]
pub fn SkillsPanel() -> Element {
    let mut state: IntellijState = use_context::<IntellijState>();
    
    // Form signals
    let mut is_registering = use_signal(|| false);
    let mut new_name = use_signal(|| "".to_string());
    let mut new_desc = use_signal(|| "".to_string());
    let mut new_script = use_signal(|| "".to_string());
    let mut new_type = use_signal(|| "bash".to_string());
    let mut new_trigger = use_signal(|| "Manual".to_string());

    let skills_list = state.skills.read().clone();
    let selected = state.selected_skill.read().clone();

    let run_selected_skill = move |_| {
        let current_skill = state.selected_skill.read().clone();
        if let Some(mut sk) = current_skill {
            sk.status = "Running".to_string();
            sk.execution_log = Some(format!(
                "[SYSTEM INFO] Initiating automation script: \"{}\"...\n[SHELL] Executing: `{}`\n",
                sk.name, sk.script
            ));
            state.selected_skill.set(Some(sk.clone()));
            
            // Update in list
            let mut list = state.skills.read().clone();
            if let Some(idx) = list.iter().position(|s| s.id == sk.id) {
                list[idx] = sk.clone();
                state.skills.set(list);
            }

            spawn_local(async move {
                let client = reqwest::Client::new();
                let payload = serde_json::json!({ "id": sk.id });
                if let Ok(resp) = client.post("/api/skills/run").json(&payload).send().await {
                    if resp.status().is_success() {
                        // Wait slightly and fetch final skill status
                        gloo_timers::future::TimeoutFuture::new(1300).await;
                        if let Ok(resp2) = client.get("/api/skills").send().await {
                            if let Ok(final_list) = resp2.json::<Vec<AutomationSkill>>().await {
                                state.skills.set(final_list.clone());
                                if let Some(updated_sk) = final_list.into_iter().find(|s| s.id == sk.id) {
                                    state.selected_skill.set(Some(updated_sk));
                                }
                            }
                        }
                    }
                }
            });
        }
    };

    let submit_new_skill = move |_| {
        let name = new_name.read().clone();
        let desc = new_desc.read().clone();
        let script = new_script.read().clone();
        let t_type = new_type.read().clone();
        let trigger = new_trigger.read().clone();

        if name.trim().is_empty() || script.trim().is_empty() {
            state.error_msg.set(Some("Name and script are required.".to_string()));
            return;
        }

        spawn_local(async move {
            let client = reqwest::Client::new();
            let payload = serde_json::json!({
                "name": name,
                "description": if desc.trim().is_empty() { None } else { Some(desc) },
                "script": script,
                "type": Some(t_type),
                "trigger": Some(trigger),
            });

            if let Ok(resp) = client.post("/api/skills").json(&payload).send().await {
                if resp.status().is_success() {
                    if let Ok(new_sk) = resp.json::<AutomationSkill>().await {
                        let mut list = state.skills.read().clone();
                        list.push(new_sk.clone());
                        state.skills.set(list);
                        state.selected_skill.set(Some(new_sk));
                        is_registering.set(false);
                        state.error_msg.set(None);
                        // Reset form
                        new_name.set("".to_string());
                        new_desc.set("".to_string());
                        new_script.set("".to_string());
                    }
                }
            }
        });
    };

    rsx! {
        div {
            style: "background: rgba(17, 21, 32, 0.7); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 16px; overflow: hidden; display: flex; flex-direction: column; flex-grow: 1; padding: 16px; gap: 16px;",
            
            // Title & Buttons bar
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",
                h2 {
                    style: "font-size: 14px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: #f1f5f9; margin: 0;",
                    "Automation Task Plane"
                }
                button {
                    style: "background: #0891b2; color: #ffffff; border: none; padding: 6px 12px; border-radius: 8px; font-size: 11px; font-weight: 600; cursor: pointer; transition: all 0.2s;",
                    onclick: move |_| {
                        let cur = *is_registering.read();
                        is_registering.set(!cur);
                    },
                    if *is_registering.read() { "View List" } else { "+ Register Skill" }
                }
            }

            {if *is_registering.read() {
                rsx! {
                    div {
                        style: "display: flex; flex-direction: column; gap: 12px; animation: fadeIn 0.2s;",
                        h3 { style: "font-size: 12px; font-weight: 600; color: #94a3b8; margin: 0;", "Register Automation Task" }
                        
                        div {
                            style: "display: flex; flex-direction: column; gap: 4px;",
                            label { style: "font-size: 10px; color: #64748b; font-family: monospace;", "Task Name *" }
                            input {
                                style: "background: #0f1322; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 8px; font-size: 11px; color: #ffffff;",
                                placeholder: "e.g., Run linker boundary check",
                                value: "{new_name}",
                                oninput: move |e| new_name.set(e.value().clone()),
                            }
                        }

                        div {
                            style: "display: flex; flex-direction: column; gap: 4px;",
                            label { style: "font-size: 10px; color: #64748b; font-family: monospace;", "Description" }
                            input {
                                style: "background: #0f1322; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 8px; font-size: 11px; color: #ffffff;",
                                placeholder: "Briefly explain the task capabilities...",
                                value: "{new_desc}",
                                oninput: move |e| new_desc.set(e.value().clone()),
                            }
                        }

                        div {
                            style: "display: flex; gap: 12px;",
                            div {
                                style: "flex: 1; display: flex; flex-direction: column; gap: 4px;",
                                label { style: "font-size: 10px; color: #64748b; font-family: monospace;", "Type" }
                                select {
                                    style: "background: #0f1322; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 8px; font-size: 11px; color: #ffffff; cursor: pointer;",
                                    value: "{new_type}",
                                    onchange: move |e| new_type.set(e.value().clone()),
                                    option { value: "bash", "Shell/CLI Crate" }
                                    option { value: "rust-macro", "Rust Analyzer Macro" }
                                    option { value: "python", "Python Script" }
                                }
                            }
                            div {
                                style: "flex: 1; display: flex; flex-direction: column; gap: 4px;",
                                label { style: "font-size: 10px; color: #64748b; font-family: monospace;", "Trigger Trigger" }
                                select {
                                    style: "background: #0f1322; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 8px; font-size: 11px; color: #ffffff; cursor: pointer;",
                                    value: "{new_trigger}",
                                    onchange: move |e| new_trigger.set(e.value().clone()),
                                    option { value: "Manual", "Manual Trigger" }
                                    option { value: "OnPreCompile", "On Pre-Compile" }
                                    option { value: "OnSave", "On File Save" }
                                }
                            }
                        }

                        div {
                            style: "display: flex; flex-direction: column; gap: 4px;",
                            label { style: "font-size: 10px; color: #64748b; font-family: monospace;", "Script/Command *" }
                            textarea {
                                style: "background: #07090f; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 8px; font-size: 11px; color: #ffffff; font-family: monospace; min-height: 80px; resize: vertical;",
                                placeholder: "e.g., cargo check --target thumbv7em-none-eabihf",
                                value: "{new_script}",
                                oninput: move |e| new_script.set(e.value().clone()),
                            }
                        }

                        {state.error_msg.read().as_ref().map(|err| rsx! {
                            div { style: "font-size: 11px; color: #f87171;", "{err}" }
                        })}

                        button {
                            style: "background: #14b8a6; color: #ffffff; border: none; padding: 10px; border-radius: 8px; font-size: 11px; font-weight: 600; cursor: pointer; transition: all 0.2s; margin-top: 4px;",
                            onclick: submit_new_skill,
                            "Register Task"
                        }
                    }
                }
            } else {
                rsx! {
                    div {
                        style: "display: flex; flex-direction: column; gap: 12px; flex-grow: 1;",
                        
                        // Tasks Row Selectors
                        div {
                            style: "display: flex; gap: 8px; overflow-x: auto; padding-bottom: 4px;",
                            {skills_list.iter().map(|sk| {
                                let is_sel = selected.as_ref().map_or(false, |s| s.id == sk.id);
                                let card_style = if is_sel {
                                    "flex-shrink: 0; background: rgba(6, 182, 212, 0.1); border: 1px solid rgba(6, 182, 212, 0.3); padding: 8px 12px; border-radius: 8px; text-align: left; cursor: pointer;"
                                } else {
                                    "flex-shrink: 0; background: #0f1322; border: 1px solid rgba(255, 255, 255, 0.04); padding: 8px 12px; border-radius: 8px; text-align: left; cursor: pointer;"
                                };
                                let status_color = match sk.status.as_str() {
                                    "Running" => "#38bdf8",
                                    "Success" => "#4ade80",
                                    "Failure" => "#f87171",
                                    _ => "#94a3b8"
                                };
                                let sk_clone = sk.clone();

                                rsx! {
                                    button {
                                        key: "{sk.id}",
                                        style: "{card_style}",
                                        onclick: move |_| state.selected_skill.set(Some(sk_clone.clone())),
                                        div {
                                            style: "font-size: 11px; font-weight: 600; color: #f1f5f9; display: flex; align-items: center; gap: 6px;",
                                            span { style: "width: 6px; height: 6px; border-radius: 50%; background: {status_color}; display: inline-block;" }
                                            "{sk.name}"
                                        }
                                        div {
                                            style: "font-size: 9px; color: #64748b; font-family: monospace; margin-top: 2px;",
                                            "{sk.trigger} | {sk.type_}"
                                        }
                                    }
                                }
                            })}
                        }

                        // Selected Task Console & Specs
                        {if let Some(sk) = selected.clone() {
                            let status_text = sk.status.as_str();
                            let is_running = status_text == "Running";
                            let status_color = match status_text {
                                "Running" => "#38bdf8",
                                "Success" => "#4ade80",
                                "Failure" => "#f87171",
                                _ => "#94a3b8"
                            };

                            rsx! {
                                div {
                                    style: "display: flex; flex-direction: column; gap: 10px; flex-grow: 1;",
                                    
                                    // Info Card
                                    div {
                                        style: "background: rgba(15, 19, 34, 0.5); padding: 12px; border-radius: 10px; border: 1px solid rgba(255, 255, 255, 0.04);",
                                        div {
                                            style: "display: flex; justify-content: space-between; align-items: flex-start;",
                                            div {
                                                h3 { style: "font-size: 12px; font-weight: 600; color: #ffffff; margin: 0;", "{sk.name}" }
                                                p { style: "font-size: 10px; color: #94a3b8; margin: 4px 0 0 0; text-align: left;", "{sk.description}" }
                                            }
                                            button {
                                                style: "background: {status_color}; color: #000; border: none; padding: 6px 14px; border-radius: 6px; font-size: 11px; font-weight: 700; cursor: pointer; transition: all 0.2s; display: flex; align-items: center; gap: 6px;",
                                                disabled: is_running,
                                                onclick: run_selected_skill,
                                                if is_running { "Running..." } else { "Run Task" }
                                            }
                                        }
                                        div {
                                            style: "display: flex; gap: 16px; margin-top: 8px; border-top: 1px solid rgba(255, 255, 255, 0.04); padding-top: 8px; font-size: 9px; font-family: monospace; color: #64748b;",
                                            span { "Trigger: {sk.trigger}" }
                                            span { "Type: {sk.type_}" }
                                            {sk.last_executed.as_ref().map(|t| rsx! {
                                                span { "Last Exec: {t}" }
                                            })}
                                        }
                                    }

                                    // Console Shell
                                    div {
                                        style: "flex-grow: 1; display: flex; flex-direction: column; background: #07090f; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 10px; overflow: hidden; min-height: 180px;",
                                        // Header
                                        div {
                                            style: "background: #0f1322; border-bottom: 1px solid rgba(255, 255, 255, 0.06); padding: 6px 12px; display: flex; justify-content: space-between; align-items: center;",
                                            span { style: "font-family: monospace; font-size: 10px; color: #64748b;", "oxide-terminal - {sk.id}" }
                                            div {
                                                style: "display: flex; gap: 4px;",
                                                span { style: "width: 8px; height: 8px; border-radius: 50%; background: #ef4444; display: inline-block;" }
                                                span { style: "width: 8px; height: 8px; border-radius: 50%; background: #eab308; display: inline-block;" }
                                                span { style: "width: 8px; height: 8px; border-radius: 50%; background: #22c55e; display: inline-block;" }
                                            }
                                        }
                                        // Body
                                        pre {
                                            style: "flex-grow: 1; margin: 0; padding: 12px; font-family: monospace; font-size: 10px; color: #48bb78; overflow-y: auto; white-space: pre-wrap; line-height: 1.4; text-align: left; background: #07090f;",
                                            "{sk.execution_log.clone().unwrap_or_default()}"
                                            if is_running {
                                                span { style: "color: #38bdf8; font-weight: 700; animate-pulse-subtle", " ▋" }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                div { style: "color: #64748b; font-size: 11px; text-align: center; padding: 20px;", "Select or Register a task to view execution telemetry." }
                            }
                        }}
                    }
                }
            }}
        }
    }
}
