#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use crate::frontend::state::{IntellijState, JetBrainsExtension, JetbrainsState};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[cfg(target_arch = "wasm32")]
#[component]
pub fn JetbrainsPanel() -> Element {
    let mut state: IntellijState = use_context::<IntellijState>();

    // Form inputs
    let mut is_adding = use_signal(|| false);
    let mut new_name = use_signal(|| "".to_string());
    let mut new_type = use_signal(|| "Annotator".to_string());
    let mut new_class = use_signal(|| "".to_string());
    let mut new_desc = use_signal(|| "".to_string());

    // Simulation inputs
    let mut sim_event = use_signal(|| "completion".to_string());
    let mut sim_val = use_signal(|| "TIM2".to_string());
    let mut sim_log_out = use_signal(|| "".to_string());
    let mut sim_result_out = use_signal(|| None::<serde_json::Value>);

    let jb_st = state.jb_state.read().clone();
    let jb_exts = state.jb_extensions.read().clone();

    let toggle_connection = move |_| {
        spawn_local(async move {
            let client = reqwest::Client::new();
            if let Ok(resp) = client.post("/api/jetbrains/connection").send().await {
                if resp.status().is_success() {
                    if let Ok(json_val) = resp.json::<serde_json::Value>().await {
                        if let Ok(st) = serde_json::from_value::<JetbrainsState>(json_val["state"].clone()) {
                            state.jb_state.set(st);
                        }
                    }
                }
            }
        });
    };

    let toggle_extension = move |id: String| {
        spawn_local(async move {
            let client = reqwest::Client::new();
            let payload = serde_json::json!({ "id": id });
            if let Ok(resp) = client.post("/api/jetbrains/toggle").json(&payload).send().await {
                if resp.status().is_success() {
                    if let Ok(json_val) = resp.json::<serde_json::Value>().await {
                        if let Ok(exts) = serde_json::from_value::<Vec<JetBrainsExtension>>(json_val["extensions"].clone()) {
                            state.jb_extensions.set(exts);
                        }
                    }
                }
            }
        });
    };

    let submit_extension = move |_| {
        let name = new_name.read().clone();
        let ext_type = new_type.read().clone();
        let class = new_class.read().clone();
        let desc = new_desc.read().clone();

        if name.trim().is_empty() || class.trim().is_empty() {
            state.error_msg.set(Some("Name and Class path are required.".to_string()));
            return;
        }

        spawn_local(async move {
            let client = reqwest::Client::new();
            let payload = serde_json::json!({
                "name": name,
                "type": ext_type,
                "implementationClass": class,
                "description": if desc.trim().is_empty() { None } else { Some(desc) },
            });

            if let Ok(resp) = client.post("/api/jetbrains/add").json(&payload).send().await {
                if resp.status().is_success() {
                    if let Ok(json_val) = resp.json::<serde_json::Value>().await {
                        if let Ok(exts) = serde_json::from_value::<Vec<JetBrainsExtension>>(json_val["extensions"].clone()) {
                            state.jb_extensions.set(exts);
                        }
                        is_adding.set(false);
                        state.error_msg.set(None);
                        // Reset Form
                        new_name.set("".to_string());
                        new_class.set("".to_string());
                        new_desc.set("".to_string());
                    }
                }
            }
        });
    };

    let compile_plugin = move |_| {
        let current_jb_st = state.jb_state.read().clone();
        if current_jb_st.gradle_task_executing {
            return;
        }
        
        let mut st = current_jb_st;
        st.gradle_task_executing = true;
        st.gradle_log = "[GRADLE] Executing task: :buildPlugin\n[GRADLE] Loading project configurations...\n".to_string();
        state.jb_state.set(st);

        spawn_local(async move {
            let client = reqwest::Client::new();
            if let Ok(resp) = client.post("/api/jetbrains/compile").send().await {
                if resp.status().is_success() {
                    // Poll periodically to fetch updated log output
                    for _ in 0..6 {
                        gloo_timers::future::TimeoutFuture::new(400).await;
                        if let Ok(resp2) = client.get("/api/jetbrains").send().await {
                            if let Ok(json_val) = resp2.json::<serde_json::Value>().await {
                                if let Ok(st2) = serde_json::from_value::<JetbrainsState>(json_val["state"].clone()) {
                                    state.jb_state.set(st2.clone());
                                    if !st2.gradle_task_executing {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
    };

    let simulate_event = move |_| {
        let ev = sim_event.read().clone();
        let val = sim_val.read().clone();

        spawn_local(async move {
            let client = reqwest::Client::new();
            let payload = serde_json::json!({
                "eventType": ev,
                "value": if val.trim().is_empty() { None } else { Some(val) },
            });

            if let Ok(resp) = client.post("/api/jetbrains/simulate").json(&payload).send().await {
                if resp.status().is_success() {
                    if let Ok(json_val) = resp.json::<serde_json::Value>().await {
                        if let Some(log_str) = json_val["log"].as_str() {
                            sim_log_out.set(log_str.to_string());
                        }
                        sim_result_out.set(Some(json_val["result"].clone()));
                    }
                }
            }
        });
    };

    rsx! {
        div {
            style: "background: rgba(17, 21, 32, 0.7); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 16px; overflow: hidden; display: flex; flex-direction: column; flex-grow: 1; padding: 16px; gap: 16px; max-height: calc(100vh - 120px); overflow-y: auto;",
            
            // Jetbrains connection status card
            div {
                style: "background: rgba(15, 19, 34, 0.5); border: 1px solid rgba(255, 255, 255, 0.04); border-radius: 10px; padding: 12px; display: flex; flex-direction: column; gap: 8px;",
                div {
                    style: "display: flex; justify-content: space-between; align-items: center;",
                    div {
                        h3 { style: "font-size: 13px; font-weight: 600; color: #ffffff; margin: 0;", "JetBrains IntelliJ Bridge" }
                        p { style: "font-size: 10px; color: #64748b; margin: 2px 0 0 0;", "Version: {jb_st.ide_version} | Port: {jb_st.port}" }
                    }
                    button {
                        style: if jb_st.connected {
                            "background: rgba(239, 68, 68, 0.1); border: 1px solid rgba(239, 68, 68, 0.3); color: #ef4444; padding: 6px 12px; border-radius: 8px; font-size: 11px; font-weight: 600; cursor: pointer;"
                        } else {
                            "background: rgba(34, 197, 94, 0.1); border: 1px solid rgba(34, 197, 94, 0.3); color: #22c55e; padding: 6px 12px; border-radius: 8px; font-size: 11px; font-weight: 600; cursor: pointer;"
                        },
                        onclick: toggle_connection,
                        if jb_st.connected { "Disconnect" } else { "Connect" }
                    }
                }
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 16px; border-top: 1px solid rgba(255, 255, 255, 0.04); padding-top: 8px; font-size: 9px; font-family: monospace; color: #64748b;",
                    span { "Active Language: {jb_st.active_language}" }
                    span { "Build: {jb_st.plugin_build}" }
                    if !jb_st.last_handshake.is_empty() {
                        span { "Handshake: {jb_st.last_handshake}" }
                    }
                }
            }

            // Extensions list section
            div {
                style: "display: flex; flex-direction: column; gap: 8px;",
                div {
                    style: "display: flex; justify-content: space-between; align-items: center;",
                    h3 { style: "font-size: 11px; font-weight: 600; text-transform: uppercase; color: #94a3b8; margin: 0;", "Extension Points" }
                    button {
                        style: "background: transparent; border: none; color: #38bdf8; font-size: 11px; font-weight: 600; cursor: pointer;",
                        onclick: move |_| {
                            let curr = *is_adding.read();
                            is_adding.set(!curr);
                        },
                        if *is_adding.read() { "[View Extensions]" } else { "[+ Register Extension]" }
                    }
                }

                {if *is_adding.read() {
                    rsx! {
                        div {
                            style: "display: flex; flex-direction: column; gap: 10px; background: rgba(15, 19, 34, 0.4); border: 1px solid rgba(255, 255, 255, 0.06); border-radius: 8px; padding: 12px; animation: fadeIn 0.2s;",
                            
                            div {
                                style: "display: flex; flex-direction: column; gap: 4px;",
                                label { style: "font-size: 9px; color: #64748b; font-family: monospace;", "Extension Name" }
                                input {
                                    style: "background: #0f1322; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 6px; font-size: 11px; color: #ffffff;",
                                    placeholder: "e.g., Circular Telemetry Inspection",
                                    value: "{new_name}",
                                    oninput: move |e| new_name.set(e.value().clone()),
                                }
                            }

                            div {
                                style: "display: flex; gap: 10px;",
                                div {
                                    style: "flex: 1; display: flex; flex-direction: column; gap: 4px;",
                                    label { style: "font-size: 9px; color: #64748b; font-family: monospace;", "Type" }
                                    select {
                                        style: "background: #0f1322; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 6px; font-size: 11px; color: #ffffff;",
                                        value: "{new_type}",
                                        onchange: move |e| new_type.set(e.value().clone()),
                                        option { value: "Annotator", "Annotator" }
                                        option { value: "Completion", "Completion" }
                                        option { value: "QuickDoc", "QuickDoc" }
                                        option { value: "ProjectService", "ProjectService" }
                                    }
                                }
                                div {
                                    style: "flex: 2; display: flex; flex-direction: column; gap: 4px;",
                                    label { style: "font-size: 9px; color: #64748b; font-family: monospace;", "JNA Implementation Class" }
                                    input {
                                        style: "background: #0f1322; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 6px; font-size: 11px; color: #ffffff; font-family: monospace;",
                                        placeholder: "com.oxidetech.embedded.analysis.TelemetryInspection",
                                        value: "{new_class}",
                                        oninput: move |e| new_class.set(e.value().clone()),
                                    }
                                }
                            }

                            div {
                                style: "display: flex; flex-direction: column; gap: 4px;",
                                label { style: "font-size: 9px; color: #64748b; font-family: monospace;", "Description" }
                                input {
                                    style: "background: #0f1322; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 6px; font-size: 11px; color: #ffffff;",
                                    placeholder: "Inspected telemetry buffers bounds warnings...",
                                    value: "{new_desc}",
                                    oninput: move |e| new_desc.set(e.value().clone()),
                                }
                            }

                            {state.error_msg.read().as_ref().map(|err| rsx! {
                                div { style: "font-size: 10px; color: #f87171;", "{err}" }
                            })}

                            button {
                                style: "background: #14b8a6; color: #ffffff; border: none; padding: 8px; border-radius: 6px; font-size: 11px; font-weight: 600; cursor: pointer;",
                                onclick: submit_extension,
                                "Register Extension"
                            }
                        }
                    }
                } else {
                    rsx! {
                        div {
                            style: "display: flex; flex-direction: column; gap: 6px; max-height: 150px; overflow-y: auto;",
                            {jb_exts.iter().map(|ext| {
                                rsx! {
                                    div {
                                        key: "{ext.id}",
                                        style: "background: rgba(15, 19, 34, 0.3); border: 1px solid rgba(255, 255, 255, 0.03); border-radius: 8px; padding: 8px 12px; display: flex; justify-content: space-between; align-items: center;",
                                        div {
                                            style: "text-align: left;",
                                            h4 { style: "font-size: 11px; font-weight: 600; color: #e2e8f0; margin: 0;", "{ext.name}" }
                                            p { style: "font-size: 9px; color: #64748b; margin: 2px 0 0 0; font-family: monospace; text-align: left;", "Class: {ext.implementation_class}" }
                                        }
                                        div {
                                            style: "display: flex; align-items: center; gap: 8px;",
                                            span {
                                                style: "font-family: monospace; font-size: 8px; padding: 1px 4px; border-radius: 4px; background: rgba(255,255,255,0.04); color: #cbd5e1;",
                                                "{ext.type_}"
                                            }
                                            input {
                                                r#type: "checkbox",
                                                checked: ext.enabled,
                                                style: "cursor: pointer; width: 12px; height: 12px;",
                                                onclick: {
                                                    let id = ext.id.clone();
                                                    move |_| toggle_extension(id.clone())
                                                }
                                            }
                                        }
                                    }
                                }
                            })}
                        }
                    }
                }}
            }

            // Compilation Build Section
            div {
                style: "display: flex; flex-direction: column; gap: 8px;",
                div {
                    style: "display: flex; justify-content: space-between; align-items: center;",
                    h3 { style: "font-size: 11px; font-weight: 600; text-transform: uppercase; color: #94a3b8; margin: 0;", "Gradle Compile Panel" }
                    button {
                        style: "background: #0891b2; color: #ffffff; border: none; padding: 4px 10px; border-radius: 6px; font-size: 10px; font-weight: 600; cursor: pointer; transition: all 0.2s;",
                        disabled: jb_st.gradle_task_executing,
                        onclick: compile_plugin,
                        if jb_st.gradle_task_executing { "Compiling..." } else { "Run buildPlugin" }
                    }
                }
                pre {
                    style: "background: #07090f; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 8px; padding: 10px; font-family: monospace; font-size: 9px; color: #e2e8f0; max-height: 120px; overflow-y: auto; text-align: left; margin: 0; white-space: pre-wrap; line-height: 1.4;",
                    "{jb_st.gradle_log}"
                    if jb_st.gradle_task_executing {
                        span { style: "color: #38bdf8; font-weight: 700; animate-pulse-subtle", " ▋" }
                    }
                }
            }

            // Simulator event panel
            div {
                style: "display: flex; flex-direction: column; gap: 8px;",
                h3 { style: "font-size: 11px; font-weight: 600; text-transform: uppercase; color: #94a3b8; margin: 0; text-align: left;", "IntelliJ Event Simulator" }
                
                div {
                    style: "display: flex; gap: 8px; align-items: center;",
                    select {
                        style: "background: #0f1322; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 6px 8px; font-size: 11px; color: #ffffff; cursor: pointer; flex-grow: 1;",
                        value: "{sim_event}",
                        onchange: move |e| {
                            let ev = e.value().clone();
                            sim_event.set(ev.clone());
                            if ev == "completion" {
                                sim_val.set("TIM2".to_string());
                            } else if ev == "hoverdoc" {
                                sim_val.set("RCC_AHB1ENR".to_string());
                            } else {
                                sim_val.set("".to_string());
                            }
                        },
                        option { value: "completion", "Autocomplete Query" }
                        option { value: "nostd", "no_std Compliance Check" }
                        option { value: "hoverdoc", "Hover Documentation Quick-Doc" }
                    }
                    if *sim_event.read() != "nostd" {
                        input {
                            style: "background: #0f1322; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 6px; font-size: 11px; color: #ffffff; font-family: monospace; width: 100px;",
                            placeholder: "Term",
                            value: "{sim_val}",
                            oninput: move |e| sim_val.set(e.value().clone()),
                        }
                    }
                    button {
                        style: "background: #14b8a6; color: #ffffff; border: none; padding: 6px 12px; border-radius: 6px; font-size: 11px; font-weight: 600; cursor: pointer; transition: all 0.2s;",
                        onclick: simulate_event,
                        "Simulate"
                    }
                }

                // Split screen logs + results
                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px;",
                    
                    // Simulation console log
                    div {
                        style: "display: flex; flex-direction: column; gap: 4px;",
                        span { style: "font-family: monospace; font-size: 9px; color: #64748b; text-align: left;", "Simulation Log" }
                        pre {
                            style: "background: #07090f; border: 1px solid rgba(255, 255, 255, 0.06); border-radius: 6px; padding: 8px; font-family: monospace; font-size: 9px; color: #34d399; overflow-y: auto; height: 100px; text-align: left; margin: 0; white-space: pre-wrap; line-height: 1.3;",
                            "{sim_log_out}"
                        }
                    }
                    
                    // Simulation result payload
                    div {
                        style: "display: flex; flex-direction: column; gap: 4px;",
                        span { style: "font-family: monospace; font-size: 9px; color: #64748b; text-align: left;", "Bridge Result Payload" }
                        pre {
                            style: "background: #07090f; border: 1px solid rgba(255, 255, 255, 0.06); border-radius: 6px; padding: 8px; font-family: monospace; font-size: 9px; color: #38bdf8; overflow-y: auto; height: 100px; text-align: left; margin: 0; white-space: pre-wrap; line-height: 1.3;",
                            {if let Some(res) = sim_result_out.read().as_ref() {
                                rsx! { "{res:#}" }
                            } else {
                                rsx! { "" }
                            }}
                        }
                    }
                }
            }
        }
    }
}
