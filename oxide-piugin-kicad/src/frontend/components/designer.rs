#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use crate::frontend::state::KiCadState;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;
#[cfg(target_arch = "wasm32")]
use serde::{Serialize, Deserialize};

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Pin {
    pub num: String,
    pub name: String,
    pub r#type: String,
    pub side: Option<String>, // "left" or "right"
}

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
    pub pitch: f64,
    pub body_width: f64,
    pub body_length: f64,
    pub body_height: f64,
    pub color: String,
}

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ComponentModel {
    pub name: String,
    pub value: String,
    pub r#type: String,
    pub reference_prefix: String,
    pub package: String,
    pub pins: Vec<Pin>,
    pub dimensions: Dimensions,
}

#[cfg(target_arch = "wasm32")]
fn ne555_preset() -> ComponentModel {
    ComponentModel {
        name: "NE555".to_string(),
        value: "Precision Timer".to_string(),
        r#type: "Timer".to_string(),
        reference_prefix: "U".to_string(),
        package: "DIP-8".to_string(),
        pins: vec![
            Pin { num: "1".to_string(), name: "GND".to_string(), r#type: "gnd".to_string(), side: Some("left".to_string()) },
            Pin { num: "2".to_string(), name: "TRIG".to_string(), r#type: "input".to_string(), side: Some("left".to_string()) },
            Pin { num: "3".to_string(), name: "OUT".to_string(), r#type: "output".to_string(), side: Some("right".to_string()) },
            Pin { num: "4".to_string(), name: "RESET".to_string(), r#type: "input".to_string(), side: Some("left".to_string()) },
            Pin { num: "5".to_string(), name: "CONT".to_string(), r#type: "passive".to_string(), side: Some("right".to_string()) },
            Pin { num: "6".to_string(), name: "THRES".to_string(), r#type: "input".to_string(), side: Some("left".to_string()) },
            Pin { num: "7".to_string(), name: "DISCH".to_string(), r#type: "passive".to_string(), side: Some("right".to_string()) },
            Pin { num: "8".to_string(), name: "VCC".to_string(), r#type: "power".to_string(), side: Some("right".to_string()) },
        ],
        dimensions: Dimensions {
            width: 10.16,
            height: 7.62,
            pitch: 2.54,
            body_width: 6.35,
            body_length: 9.27,
            body_height: 3.3,
            color: "#18181A".to_string(),
        },
    }
}

#[cfg(target_arch = "wasm32")]
fn cp2102_preset() -> ComponentModel {
    ComponentModel {
        name: "CP2102".to_string(),
        value: "USB-to-UART Bridge".to_string(),
        r#type: "Bridge".to_string(),
        reference_prefix: "U".to_string(),
        package: "QFN-28".to_string(),
        pins: vec![
            Pin { num: "1".to_string(), name: "DCD".to_string(), r#type: "input".to_string(), side: Some("left".to_string()) },
            Pin { num: "2".to_string(), name: "RI".to_string(), r#type: "input".to_string(), side: Some("left".to_string()) },
            Pin { num: "3".to_string(), name: "GND".to_string(), r#type: "gnd".to_string(), side: Some("left".to_string()) },
            Pin { num: "4".to_string(), name: "D+".to_string(), r#type: "bidirectional".to_string(), side: Some("left".to_string()) },
            Pin { num: "5".to_string(), name: "D-".to_string(), r#type: "bidirectional".to_string(), side: Some("left".to_string()) },
            Pin { num: "6".to_string(), name: "VDD".to_string(), r#type: "power".to_string(), side: Some("left".to_string()) },
            Pin { num: "7".to_string(), name: "REGIN".to_string(), r#type: "power".to_string(), side: Some("left".to_string()) },
            Pin { num: "8".to_string(), name: "VBUS".to_string(), r#type: "input".to_string(), side: Some("left".to_string()) },
            Pin { num: "9".to_string(), name: "RST".to_string(), r#type: "input".to_string(), side: Some("right".to_string()) },
            Pin { num: "12".to_string(), name: "RXD".to_string(), r#type: "input".to_string(), side: Some("right".to_string()) },
            Pin { num: "13".to_string(), name: "TXD".to_string(), r#type: "output".to_string(), side: Some("right".to_string()) },
            Pin { num: "25".to_string(), name: "SUSPEND".to_string(), r#type: "output".to_string(), side: Some("right".to_string()) },
            Pin { num: "26".to_string(), name: "SUSPEND/".to_string(), r#type: "output".to_string(), side: Some("right".to_string()) },
        ],
        dimensions: Dimensions {
            width: 5.0,
            height: 5.0,
            pitch: 0.5,
            body_width: 5.0,
            body_length: 5.0,
            body_height: 0.9,
            color: "#24252C".to_string(),
        },
    }
}

#[cfg(target_arch = "wasm32")]
fn project_3d(x: f64, y: f64, z: f64, rot_x: f64, rot_y: f64, center_x: f64, center_y: f64) -> (f64, f64, f64) {
    let rad_y = rot_y * std::f64::consts::PI / 180.0;
    let cos_y = rad_y.cos();
    let sin_y = rad_y.sin();
    
    let x1 = x * cos_y - z * sin_y;
    let z1 = x * sin_y + z * cos_y;
    
    let rad_x = rot_x * std::f64::consts::PI / 180.0;
    let cos_x = rad_x.cos();
    let sin_x = rad_x.sin();
    
    let y2 = y * cos_x - z1 * sin_x;
    let z2 = y * sin_x + z1 * cos_x;
    
    let factor = 220.0 / (220.0 + z2);
    let px = center_x + x1 * factor * 13.0;
    let py = center_y + y2 * factor * 13.0;
    
    (px, py, z2)
}

#[cfg(target_arch = "wasm32")]
#[component]
pub fn ComponentDesigner() -> Element {
    let mut state: KiCadState = use_context::<KiCadState>();
    
    let mut datasheet_input = use_signal(|| {
        "ESP32-S3 Pinout & Dimensions:\n- Pin 1: GND (GND)\n- Pin 2: 3V3 (Power)\n- Pin 3: EN (Input)\n- Pin 4: GPIO4 (I/O)\n- Pin 5: GPIO5 (I/O)\n- Pin 6: TXD0 (UART TX)\n- Pin 7: RXD0 (UART RX)\n- Pin 8: GPIO15 (I/O)\n- Outer package: SQA-24 SMD, Pitch 1.27mm\n- Body Width: 7.0mm, Body Length: 7.0mm, Height: 1.0mm".to_string()
    });

    let mut active_model = use_signal(ne555_preset);
    let mut active_tab = use_signal(|| "3d".to_string()); // "datasheet" | "schematic" | "footprint" | "3d"
    let mut is_parsing = use_signal(|| false);
    let mut parse_status = use_signal(|| "".to_string());

    let mut rot_x = use_signal(|| -20.0);
    let mut rot_y = use_signal(|| 35.0);
    let mut is_dragging = use_signal(|| false);
    let mut drag_start = use_signal(|| (0.0, 0.0));

    let mut load_preset = move |name: &str| {
        if name == "NE555" {
            active_model.set(ne555_preset());
        } else if name == "CP2102" {
            active_model.set(cp2102_preset());
        }
    };

    let handle_parse_datasheet = move |_| {
        let text = datasheet_input.read().clone();
        if text.trim().is_empty() { return; }
        
        is_parsing.set(true);
        parse_status.set("AI Reading electrical specs...".to_string());
        
        spawn_local(async move {
            let client = reqwest::Client::new();
            match client.post("/api/datasheet-parser")
                .json(&serde_json::json!({ "datasheetText": text }))
                .send()
                .await 
            {
                Ok(resp) => {
                    if resp.status().is_success() {
                        if let Ok(mut parsed) = resp.json::<ComponentModel>().await {
                            // Inject side properties if missing
                            for (i, pin) in parsed.pins.iter_mut().enumerate() {
                                if pin.side.is_none() {
                                    pin.side = Some(if i % 2 == 0 { "left".to_string() } else { "right".to_string() });
                                }
                            }
                            active_model.set(parsed);
                            parse_status.set("Successfully generated fully customizable CAD footprint, schematic symbol, and interactive 3D model!".to_string());
                            active_tab.set("3d".to_string());
                        } else {
                            parse_status.set("Error: Failed to parse datasheet JSON response.".to_string());
                        }
                    } else {
                        parse_status.set("Error: Failed to parse datasheet. Server returned error.".to_string());
                    }
                }
                Err(e) => {
                    parse_status.set(format!("Error: Network connection failed: {:?}", e));
                }
            }
            is_parsing.set(false);
        });
    };

    let handle_deploy = move |_| {
        let model = active_model.read().clone();
        let pins: Vec<(String, String)> = model.pins.iter().map(|p| (p.num.clone(), p.name.clone())).collect();
        state.instantiate_part(
            &model.name,
            &model.value,
            &model.r#type,
            &pins,
            "F.Cu"
        );
        parse_status.set(format!("Released {} successfully to active schematic and layout!", model.name));
    };

    // Orbit view handlers
    let handle_mouse_down = move |e: MouseEvent| {
        is_dragging.set(true);
        let coords = e.coordinates();
        drag_start.set((coords.page().x, coords.page().y));
    };

    let handle_mouse_move = move |e: MouseEvent| {
        if *is_dragging.read() {
            let coords = e.coordinates();
            let dx = coords.page().x - drag_start.read().0;
            let dy = coords.page().y - drag_start.read().1;
            
            let current_ry = *rot_y.read();
            rot_y.set(current_ry + dx * 1.0);
            
            let current_rx = *rot_x.read();
            rot_x.set((current_rx - dy * 1.0).max(-85.0).min(85.0));
            
            drag_start.set((coords.page().x, coords.page().y));
        }
    };

    let handle_mouse_up = move |_| {
        is_dragging.set(false);
    };

    // Calculate 3D projected elements for the model
    let model = active_model.read().clone();
    let dw = model.dimensions.body_width;
    let dl = model.dimensions.body_length;
    let dh = model.dimensions.body_height;
    let pitch_val = model.dimensions.pitch;

    // Body vertices
    let v3d = vec![
        (-dw/2.0, -dh/2.0, -dl/2.0), // 0
        (dw/2.0,  -dh/2.0, -dl/2.0), // 1
        (dw/2.0,  dh/2.0,  -dl/2.0), // 2
        (-dw/2.0, dh/2.0,  -dl/2.0), // 3
        (-dw/2.0, -dh/2.0, dl/2.0),  // 4
        (dw/2.0,  -dh/2.0, dl/2.0),  // 5
        (dw/2.0,  dh/2.0,  dl/2.0),  // 6
        (-dw/2.0, dh/2.0,  dl/2.0),  // 7
    ];

    let cx = 100.0;
    let cy = 75.0;
    let rx = *rot_x.read();
    let ry = *rot_y.read();

    let projected: Vec<(f64, f64, f64)> = v3d.iter().map(|v| project_3d(v.0, v.1, v.2, rx, ry, cx, cy)).collect();

    // Painters algorithm faces list
    // Face structures containing the indices of vertices and their average depth z
    let mut faces = vec![
        (vec![0, 1, 2, 3], "back", (projected[0].2 + projected[1].2 + projected[2].2 + projected[3].2) / 4.0),
        (vec![4, 5, 6, 7], "front", (projected[4].2 + projected[5].2 + projected[6].2 + projected[7].2) / 4.0),
        (vec![0, 4, 7, 3], "left", (projected[0].2 + projected[4].2 + projected[7].2 + projected[3].2) / 4.0),
        (vec![1, 5, 6, 2], "right", (projected[1].2 + projected[5].2 + projected[6].2 + projected[2].2) / 4.0),
        (vec![0, 1, 5, 4], "top", (projected[0].2 + projected[1].2 + projected[5].2 + projected[4].2) / 4.0),
        (vec![3, 2, 6, 7], "bottom", (projected[3].2 + projected[2].2 + projected[6].2 + projected[7].2) / 4.0),
    ];
    // Sort faces from back to front (descending depth Z)
    faces.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    rsx! {
        div {
            style: "display: flex; flex-direction: column; h-full; background-color: #111113; border: 1px solid #27272a; border-radius: 16px; overflow: hidden; box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5); box-sizing: border-box; height: 100%;",
            
            // Header
            div {
                style: "background-color: #18181b; border-bottom: 1px solid #27272a; padding: 10px; box-sizing: border-box; display: flex; flex-direction: column; gap: 8px;",
                div {
                    style: "display: flex; align-items: center; justify-content: space-between;",
                    div {
                        style: "display: flex; align-items: center; gap: 6px;",
                        span { style: "font-size: 14px;", "⚙️" }
                        h3 { style: "font-size: 11px; font-weight: bold; text-transform: uppercase; tracking-wider: 0.05em; color: #fff; margin: 0;", "Component Studio" }
                    }
                    div {
                        style: "display: flex; gap: 4px;",
                        button {
                            onclick: move |_| load_preset("NE555"),
                            style: format!("padding: 3px 8px; border-radius: 4px; font-size: 9px; font-weight: bold; border: 1px solid {}; background-color: {}; color: {}; cursor: pointer; transition: all 0.2s;",
                                if active_model.read().name == "NE555" { "#14b8a6" } else { "#27272a" },
                                if active_model.read().name == "NE555" { "rgba(20, 184, 166, 0.1)" } else { "#09090b" },
                                if active_model.read().name == "NE555" { "#14b8a6" } else { "#a1a1aa" }
                            ),
                            "NE555"
                        }
                        button {
                            onclick: move |_| load_preset("CP2102"),
                            style: format!("padding: 3px 8px; border-radius: 4px; font-size: 9px; font-weight: bold; border: 1px solid {}; background-color: {}; color: {}; cursor: pointer; transition: all 0.2s;",
                                if active_model.read().name == "CP2102" { "#14b8a6" } else { "#27272a" },
                                if active_model.read().name == "CP2102" { "rgba(20, 184, 166, 0.1)" } else { "#09090b" },
                                if active_model.read().name == "CP2102" { "#14b8a6" } else { "#a1a1aa" }
                            ),
                            "CP2102"
                        }
                    }
                }

                // Tabs bar
                div {
                    style: "display: flex; background-color: #09090b; padding: 2px; border-radius: 8px; border: 1px solid #27272a;",
                    for tab in ["datasheet", "schematic", "footprint", "3d"].iter() {
                        button {
                            onclick: move |_| active_tab.set(tab.to_string()),
                            style: format!("flex: 1; py: 6px; border-radius: 6px; font-size: 9px; font-weight: bold; border: none; text-transform: uppercase; cursor: pointer; transition: all 0.2s; background-color: {}; color: {};",
                                if active_tab.read().as_str() == *tab { "#14b8a6" } else { "transparent" },
                                if active_tab.read().as_str() == *tab { "#fff" } else { "#a1a1aa" }
                            ),
                            "{tab}"
                        }
                    }
                }
            }

            // Panel Body
            div {
                style: "flex: 1; overflow-y: auto; padding: 12px; box-sizing: border-box;",
                
                if active_tab.read().as_str() == "datasheet" {
                    div {
                        style: "display: flex; flex-direction: column; gap: 10px;",
                        div {
                            style: "background-color: #18181b; border: 1px solid #27272a; padding: 10px; border-radius: 8px;",
                            span { style: "font-size: 9px; color: #71717a; text-transform: uppercase; font-weight: bold; display: block; margin-bottom: 6px;", "Raw Silicon Specs Text" }
                            textarea {
                                value: "{datasheet_input}",
                                oninput: move |e| datasheet_input.set(e.value()),
                                style: "width: 100%; height: 120px; background-color: #09090b; color: #e2e8f0; font-family: monospace; font-size: 10px; padding: 8px; border: 1px solid #27272a; border-radius: 6px; outline: none; resize: none; line-height: 1.4; box-sizing: border-box;"
                            }
                        }
                        button {
                            onclick: handle_parse_datasheet,
                            disabled: *is_parsing.read(),
                            style: "width: 100%; padding: 10px; background-color: #14b8a6; color: #fff; border-radius: 8px; border: none; font-size: 10px; font-weight: bold; text-transform: uppercase; cursor: pointer; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.3); transition: all 0.2s;",
                            if *is_parsing.read() { "Parsing Datasheet Spec..." } else { "Extract Pinout with Gemini" }
                        }
                    }
                }

                if active_tab.read().as_str() == "schematic" {
                    div {
                        style: "display: flex; flex-direction: column; gap: 12px;",
                        div {
                            style: "background-color: #09090b; border: 1px solid #27272a; border-radius: 8px; padding: 16px; display: flex; align-items: center; justify-content: center; min-height: 140px; box-sizing: border-box;",
                            
                            // SVG schematic box preview
                            svg {
                                width: "120",
                                height: "100",
                                rect {
                                    x: "20",
                                    y: "10",
                                    width: "80",
                                    height: "80",
                                    fill: "#1f1f2e",
                                    stroke: "#14b8a6",
                                    stroke_width: "1.0",
                                    rx: "3"
                                }
                                text {
                                    x: "60",
                                    y: "35",
                                    fill: "#fff",
                                    font_size: "9.0",
                                    font_weight: "bold",
                                    text_anchor: "middle",
                                    "{model.name}"
                                }
                                text {
                                    x: "60",
                                    y: "50",
                                    fill: "#94a3b8",
                                    font_size: "6.0",
                                    text_anchor: "middle",
                                    "{model.value}"
                                }
                                
                                // Pins lines left
                                {model.pins.iter().filter(|p| p.side.as_deref() == Some("left")).enumerate().map(|(i, _)| {
                                    let y_pos = 20.0 + (i as f64) * 10.0;
                                    rsx! {
                                        line { x1: "10", y1: "{y_pos}", x2: "20", y2: "{y_pos}", stroke: "#94a3b8", stroke_width: "0.8" }
                                        circle { cx: "10", cy: "{y_pos}", r: "1.2", fill: "#94a3b8" }
                                    }
                                })}
                                
                                // Pins lines right
                                {model.pins.iter().filter(|p| p.side.as_deref() == Some("right")).enumerate().map(|(i, _)| {
                                    let y_pos = 20.0 + (i as f64) * 10.0;
                                    rsx! {
                                        line { x1: "100", y1: "{y_pos}", x2: "110", y2: "{y_pos}", stroke: "#94a3b8", stroke_width: "0.8" }
                                        circle { cx: "110", cy: "{y_pos}", r: "1.2", fill: "#94a3b8" }
                                    }
                                })}
                            }
                        }

                        // Pins attributes list
                        div {
                            style: "background-color: #18181b; border: 1px solid #27272a; border-radius: 8px; padding: 10px; max-height: 150px; overflow-y: auto;",
                            span { style: "font-size: 9px; color: #71717a; text-transform: uppercase; font-weight: bold; display: block; margin-bottom: 6px;", "Extracted Pin Assignment Grid" }
                            div {
                                style: "display: flex; flex-direction: column; gap: 4px;",
                                for pin in model.pins.iter() {
                                    div {
                                        style: "display: flex; align-items: center; justify-content: space-between; padding: 4px 8px; background-color: #09090b; border: 1px solid #27272a; border-radius: 4px; font-family: monospace; font-size: 10px;",
                                        span { style: "color: #14b8a6; font-weight: bold;", "#{pin.num} {pin.name}" }
                                        span { style: "color: #a1a1aa; text-transform: uppercase; font-size: 8px;", "{pin.r#type}" }
                                    }
                                }
                            }
                        }
                    }
                }

                if active_tab.read().as_str() == "footprint" {
                    div {
                        style: "display: flex; flex-direction: column; gap: 12px;",
                        div {
                            style: "background-color: #09090b; border: 1px solid #27272a; border-radius: 8px; padding: 12px; display: flex; align-items: center; justify-content: center; min-height: 140px; box-sizing: border-box; overflow: hidden; position: relative;",
                            
                            // 2D Copper Pads Layout
                            svg {
                                width: "120",
                                height: "120",
                                view_box: "-30 -30 60 60",
                                
                                // Package body outline silkscreen
                                rect {
                                    x: "-{dw/2.0}",
                                    y: "-{dl/2.0}",
                                    width: "{dw}",
                                    height: "{dl}",
                                    fill: "#1f1f2e",
                                    stroke: "rgba(255, 255, 255, 0.2)",
                                    stroke_width: "0.3"
                                }
                                
                                // Center notch
                                path {
                                    d: "M -2,-{dl/2.0} A 2 2 0 0 0 2,-{dl/2.0} Z",
                                    fill: "#09090b"
                                }

                                // Left Column Pads
                                {0..((model.pins.len() + 1) / 2)}.map(|i| {
                                    let pad_y = -dl/2.0 + (i as f64) * pitch_val;
                                    rsx! {
                                        rect {
                                            x: "-{dw/2.0 + 1.2}",
                                            y: "{pad_y - 0.8}",
                                            width: "2.4",
                                            height: "1.6",
                                            fill: "#f59e0b",
                                            rx: "0.2"
                                        }
                                        text {
                                            x: "-{dw/2.0 + 2.0}",
                                            y: "{pad_y}",
                                            fill: "#fff",
                                            font_size: "2.5",
                                            text_anchor: "middle",
                                            alignment_baseline: "middle",
                                            font_family: "monospace",
                                            "{i + 1}"
                                        }
                                    }
                                })

                                // Right Column Pads
                                {0..(model.pins.len() / 2)}.map(|i| {
                                    let pad_y = -dl/2.0 + (i as f64) * pitch_val;
                                    rsx! {
                                        rect {
                                            x: "{dw/2.0 - 1.2}",
                                            y: "{pad_y - 0.8}",
                                            width: "2.4",
                                            height: "1.6",
                                            fill: "#f59e0b",
                                            rx: "0.2"
                                        }
                                        text {
                                            x: "{dw/2.0 + 2.0}",
                                            y: "{pad_y}",
                                            fill: "#fff",
                                            font_size: "2.5",
                                            text_anchor: "middle",
                                            alignment_baseline: "middle",
                                            font_family: "monospace",
                                            "{model.pins.len() - i}"
                                        }
                                    }
                                })
                            }
                        }

                        // Sliders adjustment
                        div {
                            style: "background-color: #18181b; border: 1px solid #27272a; border-radius: 8px; padding: 10px; display: flex; flex-direction: column; gap: 8px; box-sizing: border-box;",
                            span { style: "font-size: 9px; color: #71717a; text-transform: uppercase; font-weight: bold; display: block;", "Adjust Packaging Envelope (mm)" }
                            
                            div {
                                style: "display: flex; flex-direction: column; gap: 6px; font-size: 10px;",
                                
                                div {
                                    style: "display: flex; align-items: center; justify-content: space-between; bg-white/5 p-1 rounded;",
                                    span { style: "color: #a1a1aa;", "Body Width (X)" }
                                    input {
                                        r#type: "range", min: "2", max: "15", step: "0.1",
                                        value: "{dw}",
                                        oninput: move |e| {
                                            let val = e.value().parse::<f64>().unwrap_or(dw);
                                            active_model.write().dimensions.body_width = val;
                                        },
                                        style: "width: 80px; accent-color: #14b8a6; height: 3px;"
                                    }
                                    span { style: "color: #14b8a6; font-family: monospace; font-weight: bold;", "{dw:.1}" }
                                }

                                div {
                                    style: "display: flex; align-items: center; justify-content: space-between; bg-white/5 p-1 rounded;",
                                    span { style: "color: #a1a1aa;", "Body Length (Y)" }
                                    input {
                                        r#type: "range", min: "2", max: "15", step: "0.1",
                                        value: "{dl}",
                                        oninput: move |e| {
                                            let val = e.value().parse::<f64>().unwrap_or(dl);
                                            active_model.write().dimensions.body_length = val;
                                        },
                                        style: "width: 80px; accent-color: #14b8a6; height: 3px;"
                                    }
                                    span { style: "color: #14b8a6; font-family: monospace; font-weight: bold;", "{dl:.1}" }
                                }

                                div {
                                    style: "display: flex; align-items: center; justify-content: space-between; bg-white/5 p-1 rounded;",
                                    span { style: "color: #a1a1aa;", "Pin Pitch" }
                                    input {
                                        r#type: "range", min: "0.4", max: "3", step: "0.05",
                                        value: "{pitch_val}",
                                        oninput: move |e| {
                                            let val = e.value().parse::<f64>().unwrap_or(pitch_val);
                                            active_model.write().dimensions.pitch = val;
                                        },
                                        style: "width: 80px; accent-color: #14b8a6; height: 3px;"
                                    }
                                    span { style: "color: #14b8a6; font-family: monospace; font-weight: bold;", "{pitch_val:.2}" }
                                }
                            }
                        }
                    }
                }

                if active_tab.read().as_str() == "3d" {
                    div {
                        style: "display: flex; flex-direction: column; gap: 10px;",
                        div {
                            style: "background-color: #09090b; border: 1px solid #27272a; border-radius: 8px; padding: 6px; display: flex; align-items: center; justify-content: center; min-height: 140px; box-sizing: border-box; position: relative;",
                            span { style: "position: absolute; top: 6px; right: 6px; font-size: 8px; color: #71717a; font-family: monospace;", "Drag to orbit" }
                            
                            // Responsive SVG 3D Flat-Shaded Package Visualizer
                            svg {
                                width: "200",
                                height: "140",
                                onmousedown: handle_mouse_down,
                                onmousemove: handle_mouse_move,
                                onmouseup: handle_mouse_up,
                                onmouseleave: handle_mouse_up,
                                style: "cursor: grab; width: 100%; height: 100%;",
                                
                                // Leads/Pins projected lines
                                {(0..model.pins.len()).map(|i| {
                                    let is_left = i < ((model.pins.len() + 1) / 2);
                                    let offset_idx = i % ((model.pins.len() + 1) / 2);
                                    let pin_z = -dl/2.0 + (offset_idx as f64) * pitch_val;
                                    
                                    if pin_z.abs() <= dl/2.0 + 0.1 {
                                        let lead_x_start = if is_left { -dw/2.0 } else { dw/2.0 };
                                        let lead_x_end = if is_left { -dw/2.0 - 1.2 } else { dw/2.0 + 1.2 };
                                        let lead_y_bot = dh/2.0 + 0.8;
                                        
                                        let p1 = project_3d(lead_x_start, dh/2.0 - 0.2, pin_z, rx, ry, cx, cy);
                                        let p2 = project_3d(lead_x_end, dh/2.0, pin_z, rx, ry, cx, cy);
                                        let p3 = project_3d(lead_x_end, lead_y_bot, pin_z, rx, ry, cx, cy);
                                        
                                        rsx! {
                                            path {
                                                d: "M {p1.0},{p1.1} L {p2.0},{p2.1} L {p3.0},{p3.1}",
                                                fill: "none",
                                                stroke: "#94a3b8",
                                                stroke_width: "2.0",
                                                stroke_linecap: "round"
                                            }
                                        }
                                    } else {
                                        rsx! {}
                                    }
                                })}

                                // Sort and Draw Faces (Painters Algorithm)
                                {faces.iter().map(|face| {
                                    let mut fill_color = model.dimensions.color.clone();
                                    if face.1 == "top" { fill_color = "#2a2b33".to_string(); }
                                    else if face.1 == "front" || face.1 == "right" { fill_color = "#16161a".to_string(); }
                                    
                                    let pts_str = face.0.iter().map(|&idx| {
                                        let proj = projected[idx];
                                        format!("{},{}", proj.0, proj.1)
                                    }).collect::<Vec<String>>().join(" ");
                                    
                                    rsx! {
                                        polygon {
                                            points: "{pts_str}",
                                            fill: "{fill_color}",
                                            stroke: "rgba(255,255,255,0.12)",
                                            stroke_width: "0.4"
                                        }
                                    }
                                })}
                            }
                        }

                        // 3D stats
                        div {
                            style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 4px; text-align: center; font-family: monospace; font-size: 9px;",
                            div {
                                style: "background-color: #18181b; border: 1px solid #27272a; padding: 4px; border-radius: 4px;",
                                span { style: "font-size: 7.5px; color: #71717a; display: block;", "WIDTH X" }
                                span { style: "color: #14b8a6; font-weight: bold; display: block; margin-top: 2px;", "{dw:.1} mm" }
                            }
                            div {
                                style: "background-color: #18181b; border: 1px solid #27272a; padding: 4px; border-radius: 4px;",
                                span { style: "font-size: 7.5px; color: #71717a; display: block;", "LENGTH Y" }
                                span { style: "color: #14b8a6; font-weight: bold; display: block; margin-top: 2px;", "{dl:.1} mm" }
                            }
                            div {
                                style: "background-color: #18181b; border: 1px solid #27272a; padding: 4px; border-radius: 4px;",
                                span { style: "font-size: 7.5px; color: #71717a; display: block;", "HEIGHT Z" }
                                span { style: "color: #14b8a6; font-weight: bold; display: block; margin-top: 2px;", "{dh:.1} mm" }
                            }
                        }
                    }
                }
            }

            // Status alerts
            if !parse_status.read().is_empty() {
                div {
                    style: format!("padding: 10px; margin: 12px; font-size: 11px; border-radius: 8px; border: 1px solid {}; background-color: {}; color: {}; line-height: 1.4;",
                        if parse_status.read().contains("Error") { "rgba(239, 68, 68, 0.2)" } else { "rgba(20, 184, 166, 0.2)" },
                        if parse_status.read().contains("Error") { "rgba(239, 68, 68, 0.05)" } else { "rgba(20, 184, 166, 0.05)" },
                        if parse_status.read().contains("Error") { "#ef4444" } else { "#14b8a6" }
                    ),
                    "{parse_status}"
                }
            }

            // Footer actions
            div {
                style: "background-color: #18181b; border-top: 1px solid #27272a; padding: 10px; display: flex; align-items: center; justify-content: space-between; gap: 8px; box-sizing: border-box;",
                button {
                    onclick: handle_deploy,
                    style: "flex: 1; padding: 10px; background-color: #14b8a6; color: #fff; border-radius: 8px; border: none; font-size: 10px; font-weight: bold; text-transform: uppercase; cursor: pointer; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.3); transition: all 0.2s;",
                    "Instantiate in Workspace"
                }
            }
        }
    }
}
