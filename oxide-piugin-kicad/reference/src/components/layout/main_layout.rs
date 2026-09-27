use dioxus::prelude::*;
use lucide_rs::{CircuitBoard, Waypoints, Play, Box, Sparkles, Cpu};
use std::collections::HashMap;

use crate::components::{
    schematic::schematic_viewer::SchematicViewer,
    pcb::pcb_viewer::PCBViewer,
    panels::{
        chat_assistant::ChatAssistant,
        component_designer::ComponentDesigner,
        component_library_panel::ComponentLibraryPanel,
        extension_control_center::ExtensionControlCenter,
    },
};

#[allow(non_snake_case)]
pub fn MainLayout(cx: Scope) -> Element {
    let show_chat = use_state(cx, || false);
    let show_extensions = use_state(cx, || false);
    let show_library = use_state(cx, || false);
    let show_designer = use_state(cx, || true);
    let visible_layers = use_ref(cx, || {
        let mut layers = HashMap::new();
        layers.insert("F.Cu".to_string(), true);
        layers.insert("B.Cu".to_string(), true);
        layers.insert("F.Silkscreen".to_string(), true);
        layers
    });

    let toggle_layer = move |layer: &str| {
        let mut layers = visible_layers.write();
        if let Some(is_visible) = layers.get_mut(layer) {
            *is_visible = !*is_visible;
        }
    };

    cx.render(rsx! {
        div {
            class: "flex h-screen bg-[#0A0A0B] text-slate-300 font-sans overflow-hidden",
            // Sidebar
            aside {
                class: "w-12 bg-[#0F0F11] border-r border-white/5 flex flex-col items-center py-4 space-y-5 shrink-0 z-10 shadow-lg animate-fade-in",
                button {
                    class: "text-teal-500 hover:text-teal-400 transition-colors cursor-pointer",
                    title: "Project",
                    CircuitBoard { size: 18 }
                },
                button {
                    class: "text-slate-500 hover:text-white transition-colors cursor-pointer",
                    title: "Schematic",
                    Waypoints { size: 18 }
                },
                button {
                    onclick: move |_| show_extensions.set(!show_extensions.get()),
                    class: format!(
                        "transition-all cursor-pointer {}",
                        if *show_extensions.get() {
                            "text-teal-400 scale-110 drop-shadow-[0_0_8px_rgba(20,184,166,0.4)]"
                        } else {
                            "text-slate-500 hover:text-white"
                        }
                    ),
                    title: "Extension Desk / Live DRC",
                    Play { size: 18 }
                },
                button {
                    onclick: move |_| show_chat.set(!show_chat.get()),
                    class: format!(
                        "transition-all cursor-pointer {}",
                        if *show_chat.get() {
                            "text-teal-400 scale-110 drop-shadow-[0_0_8px_rgba(20,184,166,0.4)]"
                        } else {
                            "text-slate-500 hover:text-white"
                        }
                    ),
                    title: "AI Command Chat",
                    Sparkles { size: 18 }
                },
                button {
                    onclick: move |_| show_designer.set(!show_designer.get()),
                    class: format!(
                        "transition-all cursor-pointer {}",
                        if *show_designer.get() {
                            "text-teal-400 scale-110 drop-shadow-[0_0_8px_rgba(20,184,166,0.4)]"
                        } else {
                            "text-slate-500 hover:text-white"
                        }
                    ),
                    title: "Datasheet Component Designer",
                    Cpu { size: 18 }
                },
                button {
                    onclick: move |_| show_library.set(!show_library.get()),
                    class: format!(
                        "transition-all mt-auto mb-2 cursor-pointer {}",
                        if *show_library.get() {
                            "text-teal-400 scale-110 drop-shadow-[0_0_8px_rgba(20,184,166,0.4)]"
                        } else {
                            "text-slate-500 hover:text-white"
                        }
                    ),
                    title: "Library & Tree",
                    Box { size: 18 }
                },
            },
            // Main Content Area
            main {
                class: "flex-1 flex flex-col min-w-0",
                // Top Toolbar
                header {
                    class: "h-11 bg-[#0F0F11] border-b border-[#1f1f23]/40 flex items-center px-4 justify-between shrink-0",
                    div {
                        class: "flex items-center space-x-4 text-xs font-medium",
                        div {
                            class: "flex items-center gap-1.5 animate-fade-in",
                            div {
                                class: "w-3.5 h-3.5 bg-teal-500 rounded-sm rotate-45 shadow-[0_0_8px_rgba(20,184,166,0.3)]"
                            },
                            span {
                                class: "font-bold text-white tracking-tight uppercase text-xs",
                                "Build.OS"
                            }
                        },
                        div {
                            class: "h-3 w-[1px] bg-white/10"
                        },
                        span {
                            class: "text-teal-400 font-mono text-[10px] uppercase tracking-widest",
                            "Engine Online"
                        }
                    },
                    div {
                        class: "flex gap-1.5",
                        {visible_layers.read().keys().map(|layer| rsx! {
                            button {
                                key: "{layer}",
                                onclick: move |_| toggle_layer(layer),
                                class: format!(
                                    "px-2.5 py-0.5 rounded text-[8px] font-bold uppercase tracking-widest transition-all cursor-pointer {}",
                                    if visible_layers.read()[layer] {
                                        "bg-teal-600 text-white shadow shadow-teal-900/20"
                                    } else {
                                        "bg-white/5 text-slate-400 border border-white/5 hover:bg-white/10"
                                    }
                                ),
                                "{layer}"
                            }
                        })}
                    }
                },
                // Workspace Split
                div {
                    class: "flex-1 flex min-h-0 bg-[#0A0A0B] p-2 gap-2 overflow-hidden",
                    {if *show_library.get() {
                        rsx! {
                            div {
                                class: "w-64 lg:w-72 flex flex-col shrink-0 text-slate-300",
                                ComponentLibraryPanel {}
                            }
                        }
                    }},
                    div {
                        class: "flex-1 flex flex-col bg-[#141417] border border-white/5 rounded-lg relative overflow-hidden",
                        div {
                            class: "absolute top-2.5 left-2.5 z-10 px-2 py-0.5 bg-white/5 rounded text-[8px] uppercase tracking-widest text-slate-400 border border-white/5 backdrop-blur pointer-events-none font-bold",
                            "Schematic Entry"
                        },
                        SchematicViewer {}
                    },
                    div {
                        class: "flex-1 flex flex-col bg-[#141417] border border-white/5 rounded-lg relative overflow-hidden",
                        div {
                            class: "absolute top-2.5 left-2.5 z-10 px-2 py-0.5 bg-white/5 rounded text-[8px] uppercase tracking-widest text-slate-400 border border-white/5 backdrop-blur pointer-events-none font-bold",
                            "PCB Canvas"
                        },
                        PCBViewer {}
                    },
                    {if *show_designer.get() {
                        rsx! {
                            div {
                                class: "w-72 lg:w-80 flex flex-col shrink-0",
                                ComponentDesigner {}
                            }
                        }
                    }},
                    {if *show_chat.get() {
                        rsx! {
                            div {
                                class: "w-72 lg:w-80 flex flex-col shrink-0",
                                ChatAssistant {}
                            }
                        }
                    }},
                    {if *show_extensions.get() {
                        rsx! {
                            div {
                                class: "w-72 lg:w-80 flex flex-col shrink-0",
                                ExtensionControlCenter {}
                            }
                        }
                    }},
                },
                // Status Bar
                footer {
                    class: "h-7 bg-[#0F0F11] border-t border-white/5 px-4 flex items-center justify-between text-[9px] font-mono text-slate-500 shrink-0 uppercase",
                    div {
                        class: "flex gap-4",
                        span {
                            class: "flex items-center gap-1",
                            span {
                                class: "w-1 h-1 rounded-full bg-teal-500 shadow-[0_0_8px_rgba(20,184,166,0.5)]"
                            },
                            "SYSTEM READY"
                        },
                        span {
                            class: "text-slate-500 ml-2",
                            "Selected Node: ",
                            span {
                                class: "text-teal-400",
                                "None"
                            }
                        }
                    },
                    div {
                        class: "flex gap-4",
                        span {
                            "UTC-08:00"
                        },
                        span {
                            class: "text-white",
                            "Build v4.2.0.881"
                        }
                    }
                }
            }
        }
    })
}
