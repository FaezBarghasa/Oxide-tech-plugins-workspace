#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use crate::frontend::state::IntellijState;
#[cfg(target_arch = "wasm32")]
use crate::frontend::components::{
    SkillsPanel, McpPanel, JetbrainsPanel, ReviewPanel, ExplorerPanel, ChatPanel
};

#[cfg(target_arch = "wasm32")]
#[component]
pub fn MainLayout() -> Element {
    let mut state: IntellijState = use_context::<IntellijState>();
    let active = state.active_tab.read().clone();

    rsx! {
        style {
            r#"
            .main-grid {{
                display: grid;
                grid-template-columns: 1fr;
                gap: 20px;
                flex-grow: 1;
                align-items: stretch;
            }}
            @media (min-width: 1024px) {{
                .main-grid {{
                    grid-template-columns: repeat(12, minmax(0, 1fr));
                }}
            }}
            "#
        }
        div {
            style: "min-height: 100vh; background: radial-gradient(circle at top left, #0d1222, #06080e); color: #cbd5e1; font-family: system-ui, -apple-system, sans-serif; display: flex; flex-direction: column; align-items: center; padding: 20px;",
            div {
                style: "width: 100%; max-width: 1200px; display: flex; flex-direction: column; min-height: calc(100vh - 40px); gap: 20px;",
                Header {}
                div {
                    class: "main-grid",
                    
                    // Left Panel Container (takes 5 cols on lg screens)
                    div {
                        style: "grid-column: span 5 / span 5; display: flex; flex-direction: column; gap: 16px;",
                        
                        // Tab Navigation
                        div {
                            style: "display: flex; background: rgba(17, 21, 32, 0.7); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.08); padding: 4px; border-radius: 12px; gap: 4px; justify-content: space-between;",
                            TabButton {
                                label: "Skills",
                                is_active: active == "skills",
                                onclick: move |_| state.active_tab.set("skills".to_string()),
                            }
                            TabButton {
                                label: "MCP",
                                is_active: active == "mcp",
                                onclick: move |_| state.active_tab.set("mcp".to_string()),
                            }
                            TabButton {
                                label: "JetBrains",
                                is_active: active == "jetbrains",
                                onclick: move |_| state.active_tab.set("jetbrains".to_string()),
                            }
                            TabButton {
                                label: "PR Review",
                                is_active: active == "review",
                                onclick: move |_| state.active_tab.set("review".to_string()),
                            }
                            TabButton {
                                label: "Files",
                                is_active: active == "explorer",
                                onclick: move |_| state.active_tab.set("explorer".to_string()),
                            }
                        }

                        // Tab Content
                        div {
                            style: "flex-grow: 1; display: flex; flex-direction: column;",
                            {match active.as_str() {
                                "skills" => rsx! { SkillsPanel {} },
                                "mcp" => rsx! { McpPanel {} },
                                "jetbrains" => rsx! { JetbrainsPanel {} },
                                "review" => rsx! { ReviewPanel {} },
                                "explorer" => rsx! { ExplorerPanel {} },
                                _ => rsx! { div { "Tab not found" } }
                            }}
                        }
                    }

                    // Right Panel Container (takes 7 cols on lg screens)
                    div {
                        style: "grid-column: span 7 / span 7; display: flex; flex-direction: column;",
                        ChatPanel {}
                    }
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
fn Header() -> Element {
    let state: IntellijState = use_context::<IntellijState>();
    let connected = state.jb_state.read().connected;
    let port = state.jb_state.read().port;

    rsx! {
        header {
            style: "display: flex; flex-direction: row; justify-content: space-between; align-items: center; border-bottom: 1px solid rgba(255, 255, 255, 0.08); padding-bottom: 12px; gap: 16px;",
            div {
                div {
                    style: "display: flex; flex-wrap: wrap; items-center; gap: 8px; margin-bottom: 4px;",
                    span {
                        style: "font-family: monospace; font-size: 10px; padding: 2px 6px; border-radius: 4px; background: rgba(6, 182, 212, 0.1); color: #22d3ee; border: 1px solid rgba(6, 182, 212, 0.2);",
                        "Agent Active (Host: {port})"
                    }
                    span {
                        style: "font-family: monospace; font-size: 10px; padding: 2px 6px; border-radius: 4px; background: rgba(249, 115, 22, 0.1); color: #fb923c; border: 1px solid rgba(249, 115, 22, 0.2);",
                        "Cortex-M / no_std Checkers"
                    }
                    {if connected {
                        rsx! {
                            span {
                                style: "font-family: monospace; font-size: 10px; padding: 2px 6px; border-radius: 4px; background: rgba(34, 197, 94, 0.1); color: #4ade80; border: 1px solid rgba(34, 197, 94, 0.2); display: inline-flex; align-items: center; gap: 4px;",
                                span { style: "width: 4px; height: 4px; border-radius: 50%; background-color: #4ade80; display: inline-block;" }
                                "IDE Connected"
                            }
                        }
                    } else {
                        rsx! {
                            span {
                                style: "font-family: monospace; font-size: 10px; padding: 2px 6px; border-radius: 4px; background: rgba(239, 68, 68, 0.1); color: #f87171; border: 1px solid rgba(239, 68, 68, 0.2);",
                                "IDE Offline"
                            }
                        }
                    }}
                }
                h1 {
                    style: "font-size: 20px; font-weight: 700; letter-spacing: -0.025em; color: #ffffff; margin: 0; display: flex; align-items: center; gap: 8px;",
                    "Oxide-Tech IDE Co-Pilot"
                }
            }
            p {
                style: "color: #94a3b8; font-size: 12px; max-width: 320px; text-align: right; margin: 0; line-height: 1.4;",
                "Embedded firmware simulator, MCP tooling and no_std code verifier control room."
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
fn TabButton(label: &'static str, is_active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let btn_style = if is_active {
        "flex-grow: 1; padding: 6px 4px; border-radius: 8px; font-size: 10px; font-weight: 600; font-family: monospace; text-align: center; border: 1px solid rgba(6, 182, 212, 0.2); background: rgba(6, 182, 212, 0.15); color: #22d3ee; cursor: pointer; transition: all 0.2s;"
    } else {
        "flex-grow: 1; padding: 6px 4px; border-radius: 8px; font-size: 10px; font-weight: 500; font-family: monospace; text-align: center; border: 1px solid transparent; background: transparent; color: #94a3b8; cursor: pointer; transition: all 0.2s;"
    };

    rsx! {
        button {
            style: "{btn_style}",
            onclick: move |e| onclick.call(e),
            "{label}"
        }
    }
}
