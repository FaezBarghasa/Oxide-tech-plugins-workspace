#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use crate::frontend::state::KiCadState;

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Debug, PartialEq)]
pub struct PartTemplate {
    pub name: String,
    pub value: String,
    pub r#type: String,
    pub pins: Vec<(String, String)>, // (PinName, DefaultNet)
    pub footprint_layer: String,
}

#[cfg(target_arch = "wasm32")]
fn library_templates() -> Vec<PartTemplate> {
    vec![
        PartTemplate {
            name: "U2 (AMS1117-3.3)".to_string(),
            value: "LDO Regulator".to_string(),
            r#type: "Regulator".to_string(),
            pins: vec![
                ("GND".to_string(), "GND".to_string()),
                ("VOUT".to_string(), "VCC_3V3".to_string()),
                ("VIN".to_string(), "VCC".to_string()),
            ],
            footprint_layer: "F.Cu".to_string(),
        },
        PartTemplate {
            name: "J1 (USB-C)".to_string(),
            value: "USB Connector".to_string(),
            r#type: "Connector".to_string(),
            pins: vec![
                ("GND".to_string(), "GND".to_string()),
                ("VBUS".to_string(), "VCC".to_string()),
                ("D+".to_string(), "USB_D_P".to_string()),
                ("D-".to_string(), "USB_D_N".to_string()),
            ],
            footprint_layer: "F.Cu".to_string(),
        },
        PartTemplate {
            name: "LED1".to_string(),
            value: "Green Indicator".to_string(),
            r#type: "LED".to_string(),
            pins: vec![
                ("A".to_string(), "SIG".to_string()),
                ("K".to_string(), "GND".to_string()),
            ],
            footprint_layer: "F.Cu".to_string(),
        },
        PartTemplate {
            name: "R2".to_string(),
            value: "220R".to_string(),
            r#type: "Resistor".to_string(),
            pins: vec![
                ("1".to_string(), "SIG".to_string()),
                ("2".to_string(), "GND".to_string()),
            ],
            footprint_layer: "F.Cu".to_string(),
        },
    ]
}

#[cfg(target_arch = "wasm32")]
#[component]
pub fn ComponentLibraryPanel() -> Element {
    let mut state: KiCadState = use_context::<KiCadState>();
    let schematic = state.schematic.read().clone();
    let mut search = use_signal(|| "".to_string());
    
    let templates = library_templates();
    
    let filtered_templates: Vec<PartTemplate> = templates.into_iter().filter(|t| {
        let q = search.read().to_lowercase();
        t.name.to_lowercase().contains(&q) || t.value.to_lowercase().contains(&q)
    }).collect();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; h-full; background-color: #141417; border: 1px solid #27272a; border-radius: 12px; overflow: hidden; box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.4); height: 100%; box-sizing: border-box;",
            
            // Search header
            div {
                style: "padding: 12px; background-color: #0f0f11; border-bottom: 1px solid #27272a; display: flex; flex-direction: column; gap: 8px; box-sizing: border-box;",
                div {
                    style: "display: flex; align-items: center; gap: 6px;",
                    span { style: "font-size: 11px; color: #14b8a6; font-weight: bold; text-transform: uppercase; tracking-wider: 0.05em;", "🧭 Flux Libs & Nets" }
                }
                div {
                    style: "display: flex; align-items: center; background-color: rgba(255,255,255,0.05); border: 1px solid #27272a; border-radius: 8px; padding: 6px 10px;",
                    span { style: "font-size: 11px; margin-right: 6px; color: #71717a;", "🔍" }
                    input {
                        r#type: "text",
                        placeholder: "Search parts, templates...",
                        value: "{search}",
                        oninput: move |e| search.set(e.value()),
                        style: "width: 100%; background: transparent; border: none; outline: none; color: #e2e8f0; font-size: 11px;"
                    }
                }
            }

            // Parts List
            div {
                style: "flex: 1; overflow-y: auto; padding: 12px; display: flex; flex-direction: column; gap: 10px; min-height: 0; box-sizing: border-box;",
                div { style: "font-size: 8px; font-weight: bold; text-transform: uppercase; tracking-widest: 0.05em; color: #71717a;", "Instantiation Templates" }
                
                for item in filtered_templates.iter() {
                    {
                        let name = item.name.clone();
                        let value = item.value.clone();
                        let type_str = item.r#type.clone();
                        let pins = item.pins.clone();
                        let layer = item.footprint_layer.clone();
                        
                        rsx! {
                            div {
                                style: "padding: 10px; background-color: rgba(255,255,255,0.02); border: 1px solid #27272a; border-radius: 8px; display: flex; flex-direction: column; gap: 6px; transition: all 0.2s; hover:border-color: rgba(20, 184, 166, 0.3); hover:background-color: rgba(255,255,255,0.05);",
                                div {
                                    style: "display: flex; align-items: flex-start; justify-content: space-between;",
                                    div {
                                        div { style: "font-family: monospace; font-size: 10px; font-weight: bold; color: #f1f5f9;", "{item.name}" }
                                        div { style: "font-size: 9px; color: #94a3b8; margin-top: 2px;", "{item.value}" }
                                    }
                                    button {
                                        onclick: move |_| {
                                            state.instantiate_part(&name, &value, &type_str, &pins, &layer);
                                        },
                                        style: "width: 20px; height: 20px; border-radius: 4px; border: none; background-color: rgba(20, 184, 166, 0.1); color: #14b8a6; display: flex; align-items: center; justify-content: center; cursor: pointer; transition: all 0.2s; hover:background-color: #14b8a6; hover:color: #fff;",
                                        "+"
                                    }
                                }
                                
                                // Pins
                                div {
                                    style: "display: flex; flex-wrap: wrap; gap: 4px;",
                                    for pin in item.pins.iter() {
                                        span {
                                            style: "font-family: monospace; font-size: 8px; color: #94a3b8; padding: 2px 4px; border-radius: 4px; background-color: rgba(255,255,255,0.04); border: 1px solid #27272a;",
                                            "{pin.0} → ",
                                            span { style: "color: #14b8a6;", "{pin.1}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if filtered_templates.is_empty() {
                    div { style: "text-align: center; color: #71717a; padding: 20px 0; font-size: 11px;", "No matching templates found." }
                }

                // Live circuit tree
                div {
                    style: "margin-top: 16px; border-top: 1px solid #27272a; padding-top: 12px; display: flex; flex-direction: column; gap: 8px;",
                    div {
                        style: "font-size: 8px; font-weight: bold; text-transform: uppercase; tracking-widest: 0.05em; color: #71717a; display: flex; align-items: center; gap: 4px;",
                        span { "💾" } "Live Circuit Tree"
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 2px; font-family: monospace; font-size: 9px; color: #94a3b8;",
                        div {
                            style: "display: flex; justify-content: space-between; padding: 3px 6px; hover:background-color: rgba(255,255,255,0.04); border-radius: 4px;",
                            span { "🔲 U1 (Main MCU)" }
                            span { style: "color: #14b8a6;", "Master" }
                        }
                        for comp in schematic.components.iter().filter(|c| c.reference != "U1") {
                            div {
                                style: "display: flex; justify-content: space-between; padding: 3px 6px; hover:background-color: rgba(255,255,255,0.04); border-radius: 4px;",
                                span { "🔲 {comp.reference} ({comp.value})" }
                                span { style: "color: #71717a;", "{comp.r#type}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
