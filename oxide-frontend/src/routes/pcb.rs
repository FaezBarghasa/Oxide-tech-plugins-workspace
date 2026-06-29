use dioxus::prelude::*;
use crate::state::*;
use std::collections::HashSet;

#[component]
pub fn PCBViewer() -> Element {
    let mut pcb_state = use_signal(|| PCBData {
        board_name: "STM32_Breakout.kicad_pcb".to_string(),
        footprints: vec![
            Footprint {
                reference: "U1".to_string(),
                value: "STM32F405RGT6".to_string(),
                x: 60.0,
                y: 60.0,
                orientation: 0.0,
                layer: "F.Cu".to_string(),
                pads: vec![
                    Pad { name: "1".to_string(), net: "GND".to_string(), x: 56.0, y: 56.0 },
                    Pad { name: "2".to_string(), net: "VCC".to_string(), x: 64.0, y: 56.0 },
                    Pad { name: "3".to_string(), net: "PA0".to_string(), x: 56.0, y: 64.0 },
                    Pad { name: "4".to_string(), net: "PA1".to_string(), x: 64.0, y: 64.0 },
                ],
            },
            Footprint {
                reference: "C1".to_string(),
                value: "100nF".to_string(),
                x: 35.0,
                y: 45.0,
                orientation: 90.0,
                layer: "F.Cu".to_string(),
                pads: vec![
                    Pad { name: "1".to_string(), net: "VCC".to_string(), x: 35.0, y: 42.0 },
                    Pad { name: "2".to_string(), net: "GND".to_string(), x: 35.0, y: 48.0 },
                ],
            },
        ],
        traces: vec![
            Trace {
                start_x: 35.0,
                start_y: 42.0,
                end_x: 64.0,
                end_y: 56.0,
                net: "VCC".to_string(),
                width: 0.5,
                layer: "F.Cu".to_string(),
            },
            Trace {
                start_x: 35.0,
                start_y: 48.0,
                end_x: 56.0,
                end_y: 56.0,
                net: "GND".to_string(),
                width: 0.5,
                layer: "F.Cu".to_string(),
            },
        ],
        board_bounds: PCBBounds {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 120.0,
            max_y: 120.0,
        },
    });

    let mut zoom = use_signal(|| 4.0f64);
    let mut pan = use_signal(|| (50.0, 50.0));
    let mut is_dragging = use_signal(|| false);
    let mut drag_start = use_signal(|| (0.0, 0.0));
    let mut selected_footprint = use_signal(|| None::<Footprint>);
    let mut visible_layers = use_signal(|| {
        let mut hs = HashSet::new();
        hs.insert("F.Cu".to_string());
        hs.insert("B.Cu".to_string());
        hs.insert("F.Silkscreen".to_string());
        hs
    });

    use_effect(move || {
        spawn(async move {
            let client = reqwest::Client::new();
            if let Ok(resp) = client.get("http://localhost:8085/api/pcb").send().await {
                if resp.status().is_success() {
                    if let Ok(data) = resp.json::<PCBData>().await {
                        pcb_state.set(data);
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
        zoom.set(new_zoom.max(1.0).min(20.0));
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

    let pcb = pcb_state.read().clone();
    let show_f_cu = visible_layers.read().contains("F.Cu");
    let show_b_cu = visible_layers.read().contains("B.Cu");

    rsx! {
        div {
            class: "flex h-full flex-1 bg-slate-900 overflow-hidden relative select-none",
            onwheel: handle_wheel,
            onmousedown: handle_mouse_down,
            onmousemove: handle_mouse_move,
            onmouseup: handle_mouse_up,
            onmouseleave: handle_mouse_up,

            // Interactive SVG Canvas
            svg {
                class: "absolute inset-0 w-full h-full cursor-grab active:cursor-grabbing",
                
                // Definitions for grid patterns and filters
                defs {
                    pattern {
                        id: "pcb-grid",
                        width: "20",
                        height: "20",
                        pattern_units: "userSpaceOnUse",
                        path { d: "M 20 0 L 0 0 0 20", fill: "none", stroke: "#1f2937", stroke_width: "0.5" }
                    }
                    filter {
                        id: "neon-glow",
                        feGaussianBlur { std_deviation: "1.0", result: "coloredBlur" }
                        feMerge {
                            feMergeNode { _in: "coloredBlur" }
                            feMergeNode { _in: "SourceGraphic" }
                        }
                    }
                }

                // Grid background
                rect { width: "100%", height: "100%", fill: "url(#pcb-grid)" }

                // Board Layer Group (Pan & Zoom)
                g {
                    transform: "translate({pan.read().0} {pan.read().1}) scale({zoom.read()})",
                    
                    // Board boundary
                    rect {
                        x: pcb.board_bounds.min_x,
                        y: pcb.board_bounds.min_y,
                        width: pcb.board_bounds.max_x - pcb.board_bounds.min_x,
                        height: pcb.board_bounds.max_y - pcb.board_bounds.min_y,
                        fill: "#0e1726",
                        stroke: "#0e7490",
                        stroke_width: "1.5",
                        stroke_dasharray: "4 2",
                        opacity: "0.9"
                    }

                    // Bottom copper layer (Blue)
                    if show_b_cu {
                        for trace in pcb.traces.iter().filter(|t| t.layer == "B.Cu") {
                            line {
                                x1: trace.start_x,
                                y1: trace.start_y,
                                x2: trace.end_x,
                                y2: trace.end_y,
                                stroke: "#3b82f6",
                                stroke_width: "{trace.width}",
                                stroke_linecap: "round",
                                opacity: "0.6"
                            }
                        }
                    }

                    // Top copper layer (Orange/Red)
                    if show_f_cu {
                        for trace in pcb.traces.iter().filter(|t| t.layer == "F.Cu") {
                            line {
                                x1: trace.start_x,
                                y1: trace.start_y,
                                x2: trace.end_x,
                                y2: trace.end_y,
                                stroke: "#ef4444",
                                stroke_width: "{trace.width}",
                                stroke_linecap: "round",
                                filter: "url(#neon-glow)",
                                opacity: "0.85"
                            }
                        }
                    }

                    // Footprints
                    for fp in pcb.footprints.iter() {
                        g {
                            transform: "translate({fp.x} {fp.y}) rotate({fp.orientation})",
                            class: "group cursor-pointer",
                            onclick: {
                                let fp_clone = fp.clone();
                                move |e| {
                                    e.stop_propagation();
                                    selected_footprint.set(Some(fp_clone.clone()));
                                }
                            },

                            // Outline
                            rect {
                                x: "-12",
                                y: "-12",
                                width: "24",
                                height: "24",
                                fill: "#1e293b",
                                fill_opacity: "0.4",
                                stroke: if selected_footprint.read().as_ref().map(|f| &f.reference) == Some(&fp.reference) { "#06b6d4" } else { "#475569" },
                                stroke_width: "0.6",
                            }

                            // Pads
                            for pad in fp.pads.iter() {
                                {
                                    let lx = pad.x - fp.x;
                                    let ly = pad.y - fp.y;
                                    rsx! {
                                        rect {
                                            x: "{lx - 1.5}",
                                            y: "{ly - 1.5}",
                                            width: "3",
                                            height: "3",
                                            fill: "#f59e0b",
                                            stroke: "#0f172a",
                                            stroke_width: "0.4"
                                        }
                                        text {
                                            x: "{lx}",
                                            y: "{ly + 0.5}",
                                            fill: "#ffffff",
                                            font_size: "1.2",
                                            text_anchor: "middle",
                                            "{pad.name}"
                                        }
                                    }
                                }
                            }

                            // Reference designator
                            text {
                                x: "0",
                                y: "15",
                                fill: "#38bdf8",
                                font_size: "3.5",
                                text_anchor: "middle",
                                font_weight: "bold",
                                "{fp.reference}"
                            }
                        }
                    }
                }
            }

            // Overlay controls (Sidebar with layers and selection)
            div {
                class: "absolute top-4 left-4 bg-slate-900/90 border border-slate-700/80 backdrop-blur-md p-4 rounded-xl shadow-2xl w-64 pointer-events-auto flex flex-col gap-4 text-xs",
                div {
                    class: "border-b border-slate-800 pb-2",
                    h3 { class: "font-semibold text-slate-200", "PCB Properties" }
                    span { class: "text-slate-500 text-[10px]", "{pcb.board_name}" }
                }
                
                // Layers list
                div {
                    class: "flex flex-col gap-2",
                    h4 { class: "font-medium text-slate-400 text-[10px] uppercase", "Visible Layers" }
                    for layer in &["F.Cu", "B.Cu", "F.Silkscreen"] {
                        label {
                            class: "flex items-center gap-2 cursor-pointer text-slate-300",
                            input {
                                type: "checkbox",
                                checked: visible_layers.read().contains(*layer),
                                onchange: move |_| {
                                    let mut ls = visible_layers.read().clone();
                                    if ls.contains(*layer) {
                                        ls.remove(*layer);
                                    } else {
                                        ls.insert(layer.to_string());
                                    }
                                    visible_layers.set(ls);
                                }
                            }
                            "{layer}"
                        }
                    }
                }

                // Selected Footprint details
                if let Some(fp) = selected_footprint.read().as_ref() {
                    div {
                        class: "border-t border-slate-800 pt-3 flex flex-col gap-1.5",
                        h4 { class: "font-medium text-slate-400 text-[10px] uppercase", "Selected Component" }
                        div { class: "flex justify-between", span { class: "text-slate-500", "Ref:" } span { class: "text-slate-200 font-bold", "{fp.reference}" } }
                        div { class: "flex justify-between", span { class: "text-slate-500", "Value:" } span { class: "text-slate-200", "{fp.value}" } }
                        div { class: "flex justify-between", span { class: "text-slate-500", "Position:" } span { class: "text-slate-200 font-mono", "({fp.x:.1}, {fp.y:.1})" } }
                        div { class: "flex justify-between", span { class: "text-slate-500", "Layer:" } span { class: "text-slate-200", "{fp.layer}" } }
                    }
                }
            }
        }
    }
}
