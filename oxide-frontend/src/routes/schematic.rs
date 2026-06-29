use dioxus::prelude::*;
use crate::state::*;

#[component]
pub fn SchematicViewer() -> Element {
    let mut schematic_state = use_signal(|| SchematicData {
        components: vec![
            SchematicComponent {
                reference: "U1".to_string(),
                value: "STM32F405RGT6".to_string(),
                r#type: "MCU".to_string(),
            },
            SchematicComponent {
                reference: "C1".to_string(),
                value: "100nF".to_string(),
                r#type: "Capacitor".to_string(),
            },
            SchematicComponent {
                reference: "R1".to_string(),
                value: "10k".to_string(),
                r#type: "Resistor".to_string(),
            },
        ],
        nets: vec![
            Net {
                name: "VCC".to_string(),
                connections: vec![
                    Connection { source: "U1".to_string(), target: "C1".to_string() },
                    Connection { source: "C1".to_string(), target: "R1".to_string() },
                ],
            },
            Net {
                name: "GND".to_string(),
                connections: vec![
                    Connection { source: "U1".to_string(), target: "C1".to_string() },
                ],
            },
        ],
    });

    let mut zoom = use_signal(|| 1.2f64);
    let mut pan = use_signal(|| (100.0, 100.0));
    let mut is_dragging = use_signal(|| false);
    let mut drag_start = use_signal(|| (0.0, 0.0));
    let mut selected_comp = use_signal(|| None::<SchematicComponent>);

    use_effect(move || {
        spawn(async move {
            let client = reqwest::Client::new();
            if let Ok(resp) = client.get("http://localhost:8085/api/schematic").send().await {
                if resp.status().is_success() {
                    if let Ok(data) = resp.json::<SchematicData>().await {
                        schematic_state.set(data);
                    }
                }
            }
        });
    });

    let handle_wheel = move |e: WheelEvent| {
        let delta = match e.delta() {
            dioxus::html::geometry::WheelDelta::Pixels(p) => p.y,
            dioxus::html::geometry::WheelDelta::Lines(l) => l.y,
            dioxus::html::geometry::WheelDelta::Pages(p) => p.y,
        };
        let factor = if delta < 0.0 { 1.15f64 } else { 0.85f64 };
        let new_zoom: f64 = *zoom.read() * factor;
        zoom.set(new_zoom.max(0.5).min(5.0));
    };

    let handle_mouse_down = move |e: MouseEvent| {
        is_dragging.set(true);
        let coord = e.coordinates();
        drag_start.set((coord.page().x, coord.page().y));
    };

    let handle_mouse_move = move |e: MouseEvent| {
        if *is_dragging.read() {
            let coord = e.coordinates();
            let dx = coord.page().x - drag_start.read().0;
            let dy = coord.page().y - drag_start.read().1;
            drag_start.set((coord.page().x, coord.page().y));
            let current_pan = *pan.read();
            pan.set((current_pan.0 + dx, current_pan.1 + dy));
        }
    };

    let handle_mouse_up = move |_| {
        is_dragging.set(false);
    };

    let schematic = schematic_state.read().clone();

    rsx! {
        div {
            class: "flex h-full flex-1 bg-slate-950 overflow-hidden relative select-none",
            onwheel: handle_wheel,
            onmousedown: handle_mouse_down,
            onmousemove: handle_mouse_move,
            onmouseup: handle_mouse_up,
            onmouseleave: handle_mouse_up,

            // Interactive SVG Canvas
            svg {
                class: "absolute inset-0 w-full h-full cursor-grab active:cursor-grabbing",
                
                // Grid background pattern
                defs {
                    pattern {
                        id: "sch-grid",
                        width: "30",
                        height: "30",
                        pattern_units: "userSpaceOnUse",
                        circle { cx: "0", cy: "0", r: "1", fill: "#374151" }
                    }
                }

                rect { width: "100%", height: "100%", fill: "url(#sch-grid)" }

                // Global transform group
                g {
                    transform: "translate({pan.read().0} {pan.read().1}) scale({zoom.read()})",

                    // Connections / Nets (wires)
                    for net in schematic.nets.iter() {
                        for conn in net.connections.iter() {
                            {
                                let src_idx = schematic.components.iter().position(|c| c.reference == conn.source);
                                let tgt_idx = schematic.components.iter().position(|c| c.reference == conn.target);
                                
                                if let (Some(s), Some(t)) = (src_idx, tgt_idx) {
                                    let sx = 100.0 + (s as f64) * 150.0;
                                    let sy = 120.0 + (s % 2) as f64 * 100.0;
                                    let tx = 100.0 + (t as f64) * 150.0;
                                    let ty = 120.0 + (t % 2) as f64 * 100.0;
                                    
                                    rsx! {
                                        g {
                                            path {
                                                d: "M {sx} {sy} L {(sx+tx)/2.0} {sy} L {(sx+tx)/2.0} {ty} L {tx} {ty}",
                                                fill: "none",
                                                stroke: "#14b8a6",
                                                stroke_width: "1.5",
                                                stroke_linecap: "round"
                                            }
                                            circle {
                                                cx: "{(sx+tx)/2.0}",
                                                cy: "{(sy+ty)/2.0}",
                                                r: "3",
                                                fill: "#22d3ee"
                                            }
                                        }
                                    }
                                } else {
                                    rsx! {}
                                }
                            }
                        }
                    }

                    // Components
                    for (idx, comp) in schematic.components.iter().enumerate() {
                        {
                            let cx = 100.0 + (idx as f64) * 150.0;
                            let cy = 120.0 + (idx % 2) as f64 * 100.0;
                            let is_selected = selected_comp.read().as_ref().map(|c| &c.reference) == Some(&comp.reference);
                            let border_color = if is_selected { "#22d3ee" } else { "#475569" };

                            rsx! {
                                g {
                                    transform: "translate({cx} {cy})",
                                    class: "group cursor-pointer",
                                    onclick: {
                                        let comp_clone = comp.clone();
                                        move |e| {
                                            e.stop_propagation();
                                            selected_comp.set(Some(comp_clone.clone()));
                                        }
                                    },

                                    // Component box
                                    rect {
                                        x: "-30",
                                        y: "-40",
                                        width: "60",
                                        height: "80",
                                        fill: "#1e1e2f",
                                        stroke: border_color,
                                        stroke_width: if is_selected { "2.0" } else { "1.0" },
                                        rx: "4",
                                    }

                                    // Designator
                                    text {
                                        x: "0",
                                        y: "-20",
                                        fill: "#22d3ee",
                                        font_size: "8.0",
                                        font_weight: "bold",
                                        text_anchor: "middle",
                                        "{comp.reference}"
                                    }
                                    // Value
                                    text {
                                        x: "0",
                                        y: "5",
                                        fill: "#e2e8f0",
                                        font_size: "6.0",
                                        text_anchor: "middle",
                                        "{comp.value}"
                                    }
                                    // Type
                                    text {
                                        x: "0",
                                        y: "25",
                                        fill: "#64748b",
                                        font_size: "5.0",
                                        text_anchor: "middle",
                                        "{comp.r#type}"
                                    }

                                    // Pins lines
                                    line { x1: "-40", y1: "-15", x2: "-30", y2: "-15", stroke: "#94a3b8", stroke_width: "1.0" }
                                    circle { cx: "-40", cy: "-15", r: "1.5", fill: "#94a3b8" }

                                    line { x1: "-40", y1: "15", x2: "-30", y2: "15", stroke: "#94a3b8", stroke_width: "1.0" }
                                    circle { cx: "-40", cy: "15", r: "1.5", fill: "#94a3b8" }

                                    line { x1: "30", y1: "-15", x2: "40", y2: "-15", stroke: "#94a3b8", stroke_width: "1.0" }
                                    circle { cx: "40", cy: "-15", r: "1.5", fill: "#94a3b8" }

                                    line { x1: "30", y1: "15", x2: "40", y2: "15", stroke: "#94a3b8", stroke_width: "1.0" }
                                    circle { cx: "40", cy: "15", r: "1.5", fill: "#94a3b8" }
                                }
                            }
                        }
                    }
                }
            }

            // Sidebar details panel
            div {
                class: "absolute top-4 left-4 bg-slate-900/90 border border-slate-700/80 backdrop-blur-md p-4 rounded-xl shadow-2xl w-64 pointer-events-auto flex flex-col gap-4 text-xs",
                div {
                    class: "border-b border-slate-800 pb-2",
                    h3 { class: "font-semibold text-slate-200", "Schematic Properties" }
                    span { class: "text-slate-500 text-[10px]", "STM32 Breakout Board" }
                }

                if let Some(comp) = selected_comp.read().as_ref() {
                    div {
                        class: "flex flex-col gap-1.5",
                        h4 { class: "font-medium text-slate-400 text-[10px] uppercase", "Selected Component" }
                        div { class: "flex justify-between", span { class: "text-slate-500", "Ref:" } span { class: "text-slate-200 font-bold", "{comp.reference}" } }
                        div { class: "flex justify-between", span { class: "text-slate-500", "Value:" } span { class: "text-slate-200", "{comp.value}" } }
                        div { class: "flex justify-between", span { class: "text-slate-500", "Type:" } span { class: "text-slate-200", "{comp.r#type}" } }
                    }
                } else {
                    span { class: "text-slate-500 italic", "Click a component to view specifications" }
                }
            }
        }
    }
}
