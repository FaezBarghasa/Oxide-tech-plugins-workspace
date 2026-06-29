#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use crate::frontend::state::KiCadState;

#[cfg(target_arch = "wasm32")]
#[component]
pub fn PCBViewer() -> Element {
    let mut state: KiCadState = use_context::<KiCadState>();
    let pcb = state.pcb.read().clone();
    let selected_ref = state.selected_ref.read().clone();
    
    let mut zoom = use_signal(|| 1.0);
    let mut pan = use_signal(|| (0.0, 0.0));
    let mut is_dragging = use_signal(|| false);
    let mut drag_start = use_signal(|| (0.0, 0.0));

    let handle_wheel = move |e: WheelEvent| {
        let delta = match e.delta() {
            dioxus::html::geometry::WheelDelta::Pixels(p) => p.y,
            dioxus::html::geometry::WheelDelta::Lines(l) => l.y,
            dioxus::html::geometry::WheelDelta::Pages(p) => p.y,
        };
        let factor = if delta < 0.0 { 1.1 } else { 0.9 };
        let new_zoom: f64 = *zoom.read() * factor;
        zoom.set(new_zoom.max(0.2).min(10.0));
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
            pan.set((current_pan.0 + dx / *zoom.read(), current_pan.1 + dy / *zoom.read()));
        }
    };

    let handle_mouse_up = move |_| {
        is_dragging.set(false);
    };

    let layers = state.visible_layers.read().clone();
    let show_f_cu = layers.contains("F.Cu");
    let show_b_cu = layers.contains("B.Cu");
    let show_f_silk = layers.contains("F.Silkscreen");
    let show_b_silk = layers.contains("B.Silkscreen");

    rsx! {
        div {
            style: "width: 100%; height: 100%; position: relative; overflow: hidden; background-color: #0b0f19; cursor: grab; user-select: none;",
            onwheel: handle_wheel,
            onmousedown: handle_mouse_down,
            onmousemove: handle_mouse_move,
            onmouseup: handle_mouse_up,
            onmouseleave: handle_mouse_up,
            
            // Grid background
            svg {
                style: "position: absolute; inset: 0; width: 100%; height: 100%; pointer-events: none;",
                defs {
                    pattern {
                        id: "pcb-grid",
                        width: "20",
                        height: "20",
                        pattern_units: "userSpaceOnUse",
                        path { d: "M 20 0 L 0 0 0 20", fill: "none", stroke: "#1f2937", stroke_width: "0.5" }
                    }
                }
                rect { width: "100%", height: "100%", fill: "url(#pcb-grid)" }
            }

            // Interactive SVG Board Content
            svg {
                style: "position: absolute; inset: 0; width: 100%; height: 100%;",
                
                // SVG Filter for Neon Glow
                defs {
                    filter {
                        id: "glow-f",
                        feGaussianBlur { std_deviation: "0.8", result: "coloredBlur" }
                        feMerge {
                            feMergeNode { _in: "coloredBlur" }
                            feMergeNode { _in: "SourceGraphic" }
                        }
                    }
                }

                // Global transform group for Pan & Zoom
                g {
                    transform: "translate({pan.read().0 + 150.0} {pan.read().1 + 150.0}) scale({zoom.read()})",
                    
                    // 1. Board boundary outline
                    rect {
                        x: pcb.board_bounds.min_x,
                        y: pcb.board_bounds.min_y,
                        width: pcb.board_bounds.max_x - pcb.board_bounds.min_x,
                        height: pcb.board_bounds.max_y - pcb.board_bounds.min_y,
                        fill: "#0c1524",
                        stroke: "#0e7490",
                        stroke_width: "1.5",
                        stroke_dasharray: "4 2",
                        opacity: "0.9"
                    }

                    // 2. Draw Copper Traces
                    if show_b_cu {
                        for trace in pcb.traces.iter().filter(|t| t.layer == "B.Cu") {
                            line {
                                x1: trace.start_x,
                                y1: trace.start_y,
                                x2: trace.end_x,
                                y2: trace.end_y,
                                stroke: "#3b82f6", // Bottom copper Blue
                                stroke_width: "{trace.width}",
                                stroke_linecap: "round",
                                opacity: "0.5"
                            }
                        }
                    }

                    if show_f_cu {
                        for trace in pcb.traces.iter().filter(|t| t.layer == "F.Cu") {
                            line {
                                x1: trace.start_x,
                                y1: trace.start_y,
                                x2: trace.end_x,
                                y2: trace.end_y,
                                stroke: "#f97316", // Top copper Orange
                                stroke_width: "{trace.width}",
                                stroke_linecap: "round",
                                filter: "url(#glow-f)",
                                opacity: "0.8"
                            }
                        }
                    }

                    // 3. Draw Footprints
                    for footprint in pcb.footprints.iter() {
                        {
                            let is_f_layer = footprint.layer == "F.Cu";
                            let footprint_ref = footprint.reference.clone();
                            let is_selected = selected_ref.as_ref() == Some(&footprint_ref);
                            let border_color = if is_selected { "#06b6d4" } else { "#4b5563" };
                            let label_color = if is_selected { "#22d3ee" } else { "#94a3b8" };
                            
                            // Only draw if its layer is active
                            if (is_f_layer && (show_f_cu || show_f_silk)) || (!is_f_layer && (show_b_cu || show_b_silk)) {
                                rsx! {
                                    g {
                                        transform: "translate({footprint.x} {footprint.y}) rotate({footprint.orientation})",
                                        class: "footprint-group",
                                        onclick: move |_| {
                                            if is_selected {
                                                state.selected_ref.set(None);
                                            } else {
                                                state.selected_ref.set(Some(footprint_ref.clone()));
                                            }
                                        },
                                        
                                        // Package outline box
                                        rect {
                                            x: "-10",
                                            y: "-10",
                                            width: "20",
                                            height: "20",
                                            fill: "#111827",
                                            fill_opacity: "0.6",
                                            stroke: border_color,
                                            stroke_width: if is_selected { "1.0" } else { "0.4" },
                                            style: "cursor: pointer; transition: all 0.2s;"
                                        }

                                        // Pin Pads
                                        for pad in footprint.pads.iter() {
                                            {
                                                let pad_lx = pad.x - footprint.x;
                                                let pad_ly = pad.y - footprint.y;
                                                rsx! {
                                                    g {
                                                        rect {
                                                            x: "{pad_lx - 1.5}",
                                                            y: "{pad_ly - 1.5}",
                                                            width: "3",
                                                            height: "3",
                                                            fill: if is_f_layer { "#f59e0b" } else { "#60a5fa" }, // copper color pins
                                                            stroke: "#111827",
                                                            stroke_width: "0.2"
                                                        }
                                                        text {
                                                            x: "{pad_lx}",
                                                            y: "{pad_ly}",
                                                            fill: "#ffffff",
                                                            font_size: "1.2",
                                                            font_family: "monospace",
                                                            text_anchor: "middle",
                                                            alignment_baseline: "middle",
                                                            "{pad.name}"
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        // Footprint Ref Text
                                        text {
                                            x: "0",
                                            y: "13",
                                            fill: label_color,
                                            font_size: "3.0",
                                            font_family: "sans-serif",
                                            font_weight: "bold",
                                            text_anchor: "middle",
                                            "{footprint.reference}"
                                        }
                                    }
                                }
                            } else {
                                rsx! {}
                            }
                        }
                    }
                }
            }

            // Layer Legend overlay
            div {
                style: "position: absolute; top: 16px; right: 16px; background-color: rgba(15, 23, 42, 0.85); backdrop-filter: blur(8px); padding: 12px; border-radius: 12px; border: 1px solid #1e293b; font-family: monospace; font-size: 10px; width: 120px; box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.5); pointer-events: none;",
                div { style: "color: #64748b; font-weight: bold; margin-bottom: 6px; text-transform: uppercase;", "PCB Layers" }
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    div {
                        style: "display: flex; align-items: center; gap: 6px;",
                        div { style: format!("width: 6px; height: 6px; border-radius: 50%; background-color: {};", if show_f_cu { "#f97316" } else { "#334155" }) }
                        span { style: format!("color: {};", if show_f_cu { "#cbd5e1" } else { "#475569" }), "F.Cu (Top)" }
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 6px;",
                        div { style: format!("width: 6px; height: 6px; border-radius: 50%; background-color: {};", if show_f_silk { "#e2e8f0" } else { "#334155" }) }
                        span { style: format!("color: {};", if show_f_silk { "#cbd5e1" } else { "#475569" }), "F.Silk" }
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 6px;",
                        div { style: format!("width: 6px; height: 6px; border-radius: 50%; background-color: {};", if show_b_cu { "#3b82f6" } else { "#334155" }) }
                        span { style: format!("color: {};", if show_b_cu { "#cbd5e1" } else { "#475569" }), "B.Cu (Bot)" }
                    }
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
pub fn SchematicViewer() -> Element {
    let state: KiCadState = use_context::<KiCadState>();
    let schematic = state.schematic.read().clone();
    let selected_ref = state.selected_ref.read().clone();

    let mut zoom = use_signal(|| 1.0);
    let mut pan = use_signal(|| (0.0, 0.0));
    let mut is_dragging = use_signal(|| false);
    let mut drag_start = use_signal(|| (0.0, 0.0));

    let handle_wheel = move |e: WheelEvent| {
        let delta = match e.delta() {
            dioxus::html::geometry::WheelDelta::Pixels(p) => p.y,
            dioxus::html::geometry::WheelDelta::Lines(l) => l.y,
            dioxus::html::geometry::WheelDelta::Pages(p) => p.y,
        };
        let factor = if delta < 0.0 { 1.1 } else { 0.9 };
        let new_zoom: f64 = *zoom.read() * factor;
        zoom.set(new_zoom.max(0.3).min(5.0));
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
            pan.set((current_pan.0 + dx / *zoom.read(), current_pan.1 + dy / *zoom.read()));
        }
    };

    let handle_mouse_up = move |_| {
        is_dragging.set(false);
    };

    rsx! {
        div {
            style: "width: 100%; height: 100%; position: relative; overflow: hidden; background-color: #0c0e17; cursor: grab; user-select: none;",
            onwheel: handle_wheel,
            onmousedown: handle_mouse_down,
            onmousemove: handle_mouse_move,
            onmouseup: handle_mouse_up,
            onmouseleave: handle_mouse_up,
            
            // Grid background
            svg {
                style: "position: absolute; inset: 0; width: 100%; height: 100%; pointer-events: none;",
                defs {
                    pattern {
                        id: "sch-grid",
                        width: "30",
                        height: "30",
                        pattern_units: "userSpaceOnUse",
                        path { d: "M 30 0 L 0 0 0 30", fill: "none", stroke: "#111827", stroke_width: "0.5" }
                        circle { cx: "0", cy: "0", r: "1", fill: "#374151" }
                    }
                }
                rect { width: "100%", height: "100%", fill: "url(#sch-grid)" }
            }

            // Interactive SVG Schematic elements
            svg {
                style: "position: absolute; inset: 0; width: 100%; height: 100%;",
                
                g {
                    transform: "translate({pan.read().0 + 100.0} {pan.read().1 + 100.0}) scale({zoom.read()})",
                    
                    // Draw connections/wires
                    for net in schematic.nets.iter() {
                        for conn in net.connections.iter() {
                            {
                                // Locate source component index and target component index to draw line
                                let src_opt = schematic.components.iter().position(|c| c.reference == conn.source);
                                let tgt_opt = schematic.components.iter().position(|c| c.reference == conn.target);
                                
                                if let (Some(s_idx), Some(t_idx)) = (src_opt, tgt_opt) {
                                    let sx = 50.0 + (s_idx as f64) * 120.0;
                                    let sy = 80.0 + (s_idx % 2) as f64 * 80.0;
                                    let tx = 50.0 + (t_idx as f64) * 120.0;
                                    let ty = 80.0 + (t_idx % 2) as f64 * 80.0;
                                    rsx! {
                                        g {
                                            path {
                                                d: "M {sx} {sy} L {(sx+tx)/2.0} {sy} L {(sx+tx)/2.0} {ty} L {tx} {ty}",
                                                fill: "none",
                                                stroke: "#14b8a6",
                                                stroke_width: "1.2",
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

                    // Draw Components
                    for (idx, comp) in schematic.components.iter().enumerate() {
                        {
                            let cx = 50.0 + (idx as f64) * 120.0;
                            let cy = 80.0 + (idx % 2) as f64 * 80.0;
                            let is_selected = selected_ref.as_ref() == Some(&comp.reference);
                            let border_color = if is_selected { "#22d3ee" } else { "#4b5563" };
                            
                            rsx! {
                                g {
                                    transform: "translate({cx} {cy})",
                                    
                                    // Main component body rectangle
                                    rect {
                                        x: "-25",
                                        y: "-35",
                                        width: "50",
                                        height: "70",
                                        fill: "#1e1e2f",
                                        stroke: border_color,
                                        stroke_width: if is_selected { "2.0" } else { "1.0" },
                                        rx: "4",
                                        style: "cursor: pointer; transition: all 0.2s;"
                                    }

                                    // Component designator and value labels
                                    text {
                                        x: "0",
                                        y: "-15",
                                        fill: "#22d3ee",
                                        font_size: "7.0",
                                        font_weight: "bold",
                                        text_anchor: "middle",
                                        "{comp.reference}"
                                    }
                                    text {
                                        x: "0",
                                        y: "5",
                                        fill: "#cbd5e1",
                                        font_size: "5.0",
                                        text_anchor: "middle",
                                        "{comp.value}"
                                    }
                                    text {
                                        x: "0",
                                        y: "20",
                                        fill: "#64748b",
                                        font_size: "4.0",
                                        text_anchor: "middle",
                                        "{comp.r#type}"
                                    }

                                    // Left/Right connection pins lines
                                    line { x1: "-35", y1: "-15", x2: "-25", y2: "-15", stroke: "#94a3b8", stroke_width: "1.0" }
                                    circle { cx: "-35", cy: "-15", r: "1.5", fill: "#94a3b8" }

                                    line { x1: "-35", y1: "15", x2: "-25", y2: "15", stroke: "#94a3b8", stroke_width: "1.0" }
                                    circle { cx: "-35", cy: "15", r: "1.5", fill: "#94a3b8" }

                                    line { x1: "25", y1: "-15", x2: "35", y2: "-15", stroke: "#94a3b8", stroke_width: "1.0" }
                                    circle { cx: "35", cy: "-15", r: "1.5", fill: "#94a3b8" }

                                    line { x1: "25", y1: "15", x2: "35", y2: "15", stroke: "#94a3b8", stroke_width: "1.0" }
                                    circle { cx: "35", cy: "15", r: "1.5", fill: "#94a3b8" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
