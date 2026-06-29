#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use crate::frontend::state::{IntellijState, McpExtension};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[cfg(target_arch = "wasm32")]
#[component]
pub fn McpPanel() -> Element {
    let mut state: IntellijState = use_context::<IntellijState>();

    // Form states
    let mut is_adding = use_signal(|| false);
    let mut new_name = use_signal(|| "".to_string());
    let mut new_desc = use_signal(|| "".to_string());
    let mut new_url = use_signal(|| "".to_string());
    let mut new_caps = use_signal(|| "tools/read_hardware, tools/datasheet_lookup".to_string());

    let mcp_list = state.mcp_servers.read().clone();

    let toggle_server = move |id: String| {
        spawn_local(async move {
            let client = reqwest::Client::new();
            let payload = serde_json::json!({ "id": id });
            if let Ok(resp) = client.post("/api/mcp/toggle").json(&payload).send().await {
                if resp.status().is_success() {
                    if let Ok(list) = resp.json::<Vec<McpExtension>>().await {
                        state.mcp_servers.set(list);
                    }
                }
            }
        });
    };

    let trigger_discovery = move |_| {
        spawn_local(async move {
            let client = reqwest::Client::new();
            if let Ok(resp) = client.post("/api/mcp/discover").send().await {
                if resp.status().is_success() {
                    if let Ok(list) = resp.json::<Vec<McpExtension>>().await {
                        state.mcp_servers.set(list);
                    }
                }
            }
        });
    };

    let submit_server = move |_| {
        let name = new_name.read().clone();
        let desc = new_desc.read().clone();
        let url = new_url.read().clone();
        let caps_raw = new_caps.read().clone();

        if name.trim().is_empty() || url.trim().is_empty() {
            state.error_msg.set(Some("Name and URL are required.".to_string()));
            return;
        }

        let caps: Vec<String> = caps_raw.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        spawn_local(async move {
            let client = reqwest::Client::new();
            let payload = serde_json::json!({
                "name": name,
                "description": if desc.trim().is_empty() { None } else { Some(desc) },
                "url": url,
                "capabilities": Some(caps),
            });

            if let Ok(resp) = client.post("/api/mcp/add").json(&payload).send().await {
                if resp.status().is_success() {
                    if let Ok(list) = resp.json::<Vec<McpExtension>>().await {
                        state.mcp_servers.set(list);
                        is_adding.set(false);
                        state.error_msg.set(None);
                        // Reset form
                        new_name.set("".to_string());
                        new_desc.set("".to_string());
                        new_url.set("".to_string());
                    }
                }
            }
        });
    };

    rsx! {
        div {
            style: "background: rgba(17, 21, 32, 0.7); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 16px; overflow: hidden; display: flex; flex-direction: column; flex-grow: 1; padding: 16px; gap: 16px;",
            
            // Header bar
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",
                h2 {
                    style: "font-size: 14px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: #f1f5f9; margin: 0;",
                    "Model Context Protocol Plane"
                }
                div {
                    style: "display: flex; gap: 8px;",
                    button {
                        style: "background: rgba(255, 255, 255, 0.06); border: 1px solid rgba(255, 255, 255, 0.08); color: #cbd5e1; padding: 6px 12px; border-radius: 8px; font-size: 11px; font-weight: 600; cursor: pointer; transition: all 0.2s;",
                        onclick: trigger_discovery,
                        "Auto-Discover"
                    }
                    button {
                        style: "background: #0891b2; color: #ffffff; border: none; padding: 6px 12px; border-radius: 8px; font-size: 11px; font-weight: 600; cursor: pointer; transition: all 0.2s;",
                        onclick: move |_| {
                            let cur = *is_adding.read();
                            is_adding.set(!cur);
                        },
                        if *is_adding.read() { "View Servers" } else { "+ Add MCP Server" }
                    }
                }
            }

            {if *is_adding.read() {
                rsx! {
                    div {
                        style: "display: flex; flex-direction: column; gap: 12px; animation: fadeIn 0.2s;",
                        h3 { style: "font-size: 12px; font-weight: 600; color: #94a3b8; margin: 0;", "Register Custom MCP Adapter" }
                        
                        div {
                            style: "display: flex; flex-direction: column; gap: 4px;",
                            label { style: "font-size: 10px; color: #64748b; font-family: monospace;", "Server Name *" }
                            input {
                                style: "background: #0f1322; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 8px; font-size: 11px; color: #ffffff;",
                                placeholder: "e.g., Logic Probe Analyzer Service",
                                value: "{new_name}",
                                oninput: move |e| new_name.set(e.value().clone()),
                            }
                        }

                        div {
                            style: "display: flex; flex-direction: column; gap: 4px;",
                            label { style: "font-size: 10px; color: #64748b; font-family: monospace;", "Description" }
                            input {
                                style: "background: #0f1322; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 8px; font-size: 11px; color: #ffffff;",
                                placeholder: "Describe what context maps this server provides...",
                                value: "{new_desc}",
                                oninput: move |e| new_desc.set(e.value().clone()),
                            }
                        }

                        div {
                            style: "display: flex; flex-direction: column; gap: 4px;",
                            label { style: "font-size: 10px; color: #64748b; font-family: monospace;", "Server HTTP/SSE URL *" }
                            input {
                                style: "background: #0f1322; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 8px; font-size: 11px; color: #ffffff; font-family: monospace;",
                                placeholder: "e.g., http://localhost:8505/mcp",
                                value: "{new_url}",
                                oninput: move |e| new_url.set(e.value().clone()),
                            }
                        }

                        div {
                            style: "display: flex; flex-direction: column; gap: 4px;",
                            label { style: "font-size: 10px; color: #64748b; font-family: monospace;", "Capabilities (comma separated)" }
                            input {
                                style: "background: #0f1322; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 8px; font-size: 11px; color: #ffffff;",
                                placeholder: "tools/query_registers, resources/registers_schema",
                                value: "{new_caps}",
                                oninput: move |e| new_caps.set(e.value().clone()),
                            }
                        }

                        {state.error_msg.read().as_ref().map(|err| rsx! {
                            div { style: "font-size: 11px; color: #f87171;", "{err}" }
                        })}

                        button {
                            style: "background: #14b8a6; color: #ffffff; border: none; padding: 10px; border-radius: 8px; font-size: 11px; font-weight: 600; cursor: pointer; transition: all 0.2s; margin-top: 4px;",
                            onclick: submit_server,
                            "Register Adapter"
                        }
                    }
                }
            } else {
                rsx! {
                    div {
                        style: "display: flex; flex-direction: column; gap: 12px; flex-grow: 1; overflow-y: auto;",
                        {mcp_list.iter().map(|mcp| {
                            let status_color = match mcp.status.as_str() {
                                "Connected" => "#4ade80",
                                "Discovered" => "#fb923c",
                                _ => "#f87171"
                            };

                            rsx! {
                                div {
                                    key: "{mcp.id}",
                                    style: "background: rgba(15, 19, 34, 0.5); border: 1px solid rgba(255, 255, 255, 0.04); border-radius: 10px; padding: 12px; display: flex; flex-direction: column; gap: 8px;",
                                    
                                    // Row 1: Title, Status, Toggle
                                    div {
                                        style: "display: flex; justify-content: space-between; align-items: center;",
                                        div {
                                            style: "display: flex; align-items: center; gap: 8px;",
                                            span { style: "width: 8px; height: 8px; border-radius: 50%; background: {status_color}; display: inline-block;" }
                                            h3 { style: "font-size: 12px; font-weight: 600; color: #ffffff; margin: 0;", "{mcp.name}" }
                                        }
                                        button {
                                            style: "background: rgba(255, 255, 255, 0.04); border: 1px solid rgba(255, 255, 255, 0.08); color: #cbd5e1; padding: 4px 8px; border-radius: 6px; font-size: 10px; font-weight: 500; cursor: pointer; transition: all 0.2s;",
                                            onclick: {
                                                let id = mcp.id.clone();
                                                move |_| toggle_server(id.clone())
                                            },
                                            if mcp.status == "Connected" { "Disconnect" } else { "Connect" }
                                        }
                                    }

                                    // Row 2: Description
                                    p { style: "font-size: 10px; color: #94a3b8; margin: 0; line-height: 1.4; text-align: left;", "{mcp.description}" }

                                    // Row 3: URL
                                    div {
                                        style: "font-family: monospace; font-size: 9px; color: #64748b; text-align: left;",
                                        "Endpoint: {mcp.url}"
                                    }

                                    // Row 4: Capabilities badges
                                    div {
                                        style: "display: flex; flex-wrap: wrap; gap: 4px; border-top: 1px solid rgba(255, 255, 255, 0.04); padding-top: 8px;",
                                        {mcp.capabilities.iter().map(|cap| rsx! {
                                            span {
                                                key: "{cap}",
                                                style: "font-family: monospace; font-size: 8px; padding: 2px 6px; border-radius: 4px; background: rgba(255, 255, 255, 0.04); color: #94a3b8; border: 1px solid rgba(255, 255, 255, 0.08);",
                                                "{cap}"
                                            }
                                        })}
                                    }
                                }
                            }
                        })}
                    }
                }
            }}
        }
    }
}
