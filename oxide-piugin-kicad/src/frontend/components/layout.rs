#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use crate::frontend::state::KiCadState;
#[cfg(target_arch = "wasm32")]
use crate::frontend::components::{SchematicViewer, PCBViewer, ComponentDesigner, ComponentLibraryPanel, ChatPanel};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[cfg(target_arch = "wasm32")]
#[component]
pub fn MainLayout() -> Element {
    let mut state: KiCadState = use_context::<KiCadState>();
    
    let active_panel = state.active_panel.read().clone();
    let selected_ref = state.selected_ref.read().clone();
    let board_sync = state.board_sync.read().clone();
    let error_msg = state.error_msg.read().clone();

    // Auto poll board sync status from Actix Web backend every 2 seconds
    use_effect(move || {
        spawn_local(async move {
            loop {
                gloo_timers::future::TimeoutFuture::new(2000).await;
                let client = reqwest::Client::new();
                if let Ok(resp) = client.get("/api/board").send().await {
                    if resp.status().is_success() {
                        if let Ok(sync_info) = resp.json::<crate::frontend::state::BoardSyncRequest>().await {
                            state.board_sync.set(Some(sync_info));
                        }
                    }
                }
            }
        });
    });

    rsx! {
        div {
            style: "display: flex; height: 100vh; width: 100vw; bg-color: #0a0a0b; color: #cbd5e1; font-family: system-ui, sans-serif; overflow: hidden; background-color: #0a0a0b; box-sizing: border-box;",
            
            // Sidebar Navigation
            aside {
                style: "width: 48px; background-color: #0f0f11; border-right: 1px solid #27272a; display: flex; flex-direction: column; align-items: center; py: 16px; gap: 20px; shrink: 0; padding-top: 16px; box-sizing: border-box;",
                
                // Logo
                div {
                    style: "width: 28px; height: 28px; border-radius: 6px; background: linear-gradient(135deg, #14b8a6, #0e7490); display: flex; align-items: center; justify-content: center; font-weight: bold; color: #fff; font-size: 11px;",
                    "OX"
                }
                
                div { style: "height: 1px; width: 24px; background-color: #27272a;" }

                // Library
                button {
                    onclick: move |_| {
                        if *state.active_panel.read() == "library" {
                            state.active_panel.set("".to_string());
                        } else {
                            state.active_panel.set("library".to_string());
                        }
                    },
                    style: format!("width: 32px; height: 32px; border-radius: 8px; border: none; display: flex; align-items: center; justify-content: center; cursor: pointer; transition: all 0.2s; background-color: {}; color: {};",
                        if active_panel == "library" { "rgba(20, 184, 166, 0.1)" } else { "transparent" },
                        if active_panel == "library" { "#14b8a6" } else { "#71717a" }
                    ),
                    title: "Component Library",
                    "📚"
                }

                // Component Designer
                button {
                    onclick: move |_| {
                        if *state.active_panel.read() == "designer" {
                            state.active_panel.set("".to_string());
                        } else {
                            state.active_panel.set("designer".to_string());
                        }
                    },
                    style: format!("width: 32px; height: 32px; border-radius: 8px; border: none; display: flex; align-items: center; justify-content: center; cursor: pointer; transition: all 0.2s; background-color: {}; color: {};",
                        if active_panel == "designer" { "rgba(20, 184, 166, 0.1)" } else { "transparent" },
                        if active_panel == "designer" { "#14b8a6" } else { "#71717a" }
                    ),
                    title: "Silicon Component Designer",
                    "⚙️"
                }

                // Chat
                button {
                    onclick: move |_| {
                        if *state.active_panel.read() == "chat" {
                            state.active_panel.set("".to_string());
                        } else {
                            state.active_panel.set("chat".to_string());
                        }
                    },
                    style: format!("width: 32px; height: 32px; border-radius: 8px; border: none; display: flex; align-items: center; justify-content: center; cursor: pointer; transition: all 0.2s; background-color: {}; color: {};",
                        if active_panel == "chat" { "rgba(20, 184, 166, 0.1)" } else { "transparent" },
                        if active_panel == "chat" { "#14b8a6" } else { "#71717a" }
                    ),
                    title: "AI Assistant",
                    "💬"
                }
            }

            // Main UI
            div {
                style: "flex: 1; display: flex; flex-direction: column; min-width: 0; box-sizing: border-box;",
                
                // Top Header Toolbar
                header {
                    style: "height: 44px; background-color: #0f0f11; border-bottom: 1px solid #27272a; display: flex; align-items: center; px: 16px; justify-content: space-between; padding-left: 16px; padding-right: 16px; box-sizing: border-box; shrink: 0;",
                    div {
                        style: "display: flex; align-items: center; gap: 12px;",
                        span { style: "font-size: 11px; font-weight: bold; color: #fff; tracking-wider: 0.05em; text-transform: uppercase;", "Build.OS Lattice 🎛️" }
                        div { style: "width: 1px; height: 12px; background-color: #27272a;" }
                        span { style: "font-size: 9px; font-family: monospace; color: #14b8a6;", "KiCad Core Bridge" }
                    }

                    // Layers Toggles
                    div {
                        style: "display: flex; gap: 6px;",
                        for &layer in ["F.Cu", "F.Silkscreen", "B.Cu"].iter() {
                            button {
                                onclick: move |_| {
                                    let mut layers = state.visible_layers.write();
                                    if layers.contains(layer) {
                                        layers.remove(layer);
                                    } else {
                                        layers.insert(layer.to_string());
                                    }
                                },
                                style: format!("padding: 3px 10px; border-radius: 6px; font-size: 9px; font-weight: bold; text-transform: uppercase; border: 1px solid {}; background-color: {}; color: {}; cursor: pointer; transition: all 0.2s;",
                                    if state.visible_layers.read().contains(layer) { "#14b8a6" } else { "#27272a" },
                                    if state.visible_layers.read().contains(layer) { "rgba(20, 184, 166, 0.1)" } else { "#09090b" },
                                    if state.visible_layers.read().contains(layer) { "#fff" } else { "#71717a" }
                                ),
                                "{layer}"
                            }
                        }
                    }
                }

                // Workspace Content Splitting
                div {
                    style: "flex: 1; display: flex; min-height: 0; background-color: #0a0a0b; p: 8px; padding: 8px; gap: 8px; overflow: hidden; box-sizing: border-box;",
                    
                    if active_panel == "library" {
                        div {
                            style: "width: 280px; shrink: 0; height: 100%;",
                            ComponentLibraryPanel {}
                        }
                    }

                    // Schematic Window
                    div {
                        style: "flex: 1; display: flex; flex-direction: column; background-color: #141417; border: 1px solid #27272a; border-radius: 12px; relative; overflow: hidden; height: 100%; box-sizing: border-box;",
                        div {
                            style: "position: absolute; top: 12px; left: 12px; z-index: 10; padding: 3px 8px; background-color: rgba(15, 23, 42, 0.7); border: 1px solid #1e293b; border-radius: 6px; font-size: 9px; font-family: monospace; font-weight: bold; color: #94a3b8; text-transform: uppercase;",
                            "Schematic Sheet"
                        }
                        SchematicViewer {}
                    }

                    // PCB Window
                    div {
                        style: "flex: 1; display: flex; flex-direction: column; background-color: #141417; border: 1px solid #27272a; border-radius: 12px; relative; overflow: hidden; height: 100%; box-sizing: border-box;",
                        div {
                            style: "position: absolute; top: 12px; left: 12px; z-index: 10; padding: 3px 8px; background-color: rgba(15, 23, 42, 0.7); border: 1px solid #1e293b; border-radius: 6px; font-size: 9px; font-family: monospace; font-weight: bold; color: #94a3b8; text-transform: uppercase;",
                            "PCB Layout Canvas"
                        }
                        PCBViewer {}
                    }

                    if active_panel == "designer" {
                        div {
                            style: "width: 290px; shrink: 0; height: 100%;",
                            ComponentDesigner {}
                        }
                    }

                    if active_panel == "chat" {
                        div {
                            style: "width: 320px; shrink: 0; height: 100%;",
                            ChatPanel {}
                        }
                    }
                }

                // Error Alerts if any
                if let Some(err) = error_msg {
                    div {
                        style: "background-color: #ef4444; color: #fff; padding: 6px 16px; font-size: 11px; font-weight: bold; display: flex; align-items: center; gap: 8px; shrink: 0;",
                        span { "⚠️" }
                        span { "{err}" }
                        button {
                            onclick: move |_| state.error_msg.set(None),
                            style: "margin-left: auto; background: transparent; border: none; color: #fff; cursor: pointer; font-size: 12px;",
                            "×"
                        }
                    }
                }

                // Footer Status Bar
                footer {
                    style: "height: 28px; background-color: #0f0f11; border-top: 1px solid #27272a; px: 16px; padding-left: 16px; padding-right: 16px; display: flex; align-items: center; justify-content: space-between; font-family: monospace; font-size: 9px; color: #4b5563; shrink: 0; text-transform: uppercase; box-sizing: border-box;",
                    div {
                        style: "display: flex; gap: 16px;",
                        div {
                            style: "display: flex; align-items: center; gap: 6px;",
                            span {
                                style: format!("width: 6px; height: 6px; border-radius: 50%; background-color: {}; box-shadow: 0 0 6px {};",
                                    if board_sync.is_some() { "#10b981" } else { "#f59e0b" },
                                    if board_sync.is_some() { "#10b981" } else { "#f59e0b" }
                                )
                            }
                            if let Some(ref sync) = board_sync {
                                span { style: "color: #cbd5e1;", "CONNECTED: {sync.filename}" }
                            } else {
                                span { "WAITING FOR KICAD SYNC..." }
                            }
                        }
                        
                        if let Some(ref sync) = board_sync {
                            span { style: "color: #4b5563;",
                                "Layers: "
                                span { style: "color: #10b981;", "{sync.layer_count}" }
                            }
                        }

                        if let Some(ref selected) = selected_ref {
                            span {
                                "Selected Footprint: "
                                span { style: "color: #22d3ee;", "{selected}" }
                            }
                        }
                    }

                    div {
                        style: "display: flex; gap: 12px;",
                        span { "UTC+03:30" }
                        span { style: "color: #cbd5e1;", "Lattice v7.0" }
                    }
                }
            }
        }
    }
}
