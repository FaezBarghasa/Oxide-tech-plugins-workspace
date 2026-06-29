#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use crate::frontend::state::CADState;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[cfg(target_arch = "wasm32")]
#[component]
pub fn ParametricInputPanel() -> Element {
    let mut state: CADState = use_context::<CADState>();
    let current_params = state.params.read().clone();
    
    let handle_width_change = move |e: Event<FormData>| {
        let val = e.value().parse::<f64>().unwrap_or(0.0);
        let mut new_params = state.params.read().clone();
        new_params.dimensions.width = val;
        state.update_params(new_params);
    };

    let handle_height_change = move |e: Event<FormData>| {
        let val = e.value().parse::<f64>().unwrap_or(0.0);
        let mut new_params = state.params.read().clone();
        new_params.dimensions.height = val;
        state.update_params(new_params);
    };

    let handle_depth_change = move |e: Event<FormData>| {
        let val = e.value().parse::<f64>().unwrap_or(0.0);
        let mut new_params = state.params.read().clone();
        new_params.dimensions.depth = val;
        state.update_params(new_params);
    };

    let handle_wall_change = move |e: Event<FormData>| {
        let val = e.value().parse::<f64>().unwrap_or(0.0);
        let mut new_params = state.params.read().clone();
        new_params.wall_thickness = val;
        state.update_params(new_params);
    };

    let mut handle_material_select = move |mat: &str| {
        let mut new_params = state.params.read().clone();
        new_params.material = mat.to_string();
        state.update_params(new_params);
    };

    let handle_dia_change = move |e: Event<FormData>| {
        let val = e.value().parse::<f64>().unwrap_or(0.0);
        let mut new_params = state.params.read().clone();
        new_params.vent_config.hole_diameter = val;
        state.update_params(new_params);
    };

    let handle_spacing_change = move |e: Event<FormData>| {
        let val = e.value().parse::<f64>().unwrap_or(0.0);
        let mut new_params = state.params.read().clone();
        new_params.vent_config.spacing = val;
        state.update_params(new_params);
    };

    let handle_qty_change = move |e: Event<FormData>| {
        let val = e.value().parse::<i32>().unwrap_or(0);
        let mut new_params = state.params.read().clone();
        new_params.vent_config.quantity = val;
        state.update_params(new_params);
    };

    let on_generate = move |_| {
        let params = state.params.read().clone();
        
        // Validation rules
        if params.dimensions.width < 50.0 || params.dimensions.width > 500.0 {
            *state.error_msg.write() = Some("Width must be between 50mm and 500mm".to_string());
            return;
        }
        if params.dimensions.height < 50.0 || params.dimensions.height > 500.0 {
            *state.error_msg.write() = Some("Height must be between 50mm and 500mm".to_string());
            return;
        }
        if params.dimensions.depth < 30.0 || params.dimensions.depth > 500.0 {
            *state.error_msg.write() = Some("Depth must be between 30mm and 500mm".to_string());
            return;
        }
        if params.wall_thickness < 1.5 || params.wall_thickness > 10.0 {
            *state.error_msg.write() = Some("Wall thickness must be between 1.5mm and 10mm".to_string());
            return;
        }
        if params.vent_config.hole_diameter < 2.0 || params.vent_config.hole_diameter > 50.0 {
            *state.error_msg.write() = Some("Vent hole diameter must be between 2mm and 50mm".to_string());
            return;
        }

        *state.error_msg.write() = None;
        *state.is_generating.write() = true;

        spawn_local(async move {
            let client = reqwest::Client::new();
            match client.post("/api/parameters")
                .json(&params)
                .send()
                .await 
            {
                Ok(resp) => {
                    if resp.status().is_success() {
                        // Successfully updated params on server
                        *state.is_generating.write() = false;
                    } else {
                        *state.error_msg.write() = Some("Failed to save parameters on server".to_string());
                        *state.is_generating.write() = false;
                    }
                }
                Err(err) => {
                    *state.error_msg.write() = Some(format!("Network error: {:?}", err));
                    *state.is_generating.write() = false;
                }
            }
        });
    };

    let error_display = if let Some(err) = state.error_msg.read().clone() {
        rsx! {
            div {
                style: "background-color: rgba(239, 68, 68, 0.1); border: 1px solid rgba(239, 68, 68, 0.5); color: #f87171; font-size: 12px; padding: 10px; border-radius: 8px; margin-bottom: 12px;",
                "{err}"
            }
        }
    } else {
        rsx! {}
    };

    let generate_text = if *state.is_generating.read() {
        "Generating Model..."
    } else {
        "Generate Enclosure"
    };

    rsx! {
        div {
            style: "padding: 16px; height: 100%; display: flex; flex-direction: column; box-sizing: border-box;",
            div {
                style: "margin-bottom: 16px;",
                h2 { style: "font-size: 14px; font-weight: bold; margin: 0; color: #fff; text-transform: uppercase; letter-spacing: 0.05em;", "CAD Configuration" }
                p { style: "font-size: 11px; color: #a1a1aa; margin: 2px 0 0 0;", "Define mechanical CAD parameters." }
            }

            {error_display}

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px; margin-bottom: 16px;",
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { style: "font-size: 10px; font-weight: bold; color: #71717a; text-transform: uppercase; letter-spacing: 0.05em;", "Width (mm)" }
                    input {
                        r#type: "number",
                        value: "{current_params.dimensions.width}",
                        oninput: handle_width_change,
                        style: "padding: 6px 8px; background-color: #09090b; border: 1px solid #27272a; border-radius: 8px; color: #fff; font-family: monospace; font-size: 12px; outline: none;"
                    }
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { style: "font-size: 10px; font-weight: bold; color: #71717a; text-transform: uppercase; letter-spacing: 0.05em;", "Height (mm)" }
                    input {
                        r#type: "number",
                        value: "{current_params.dimensions.height}",
                        oninput: handle_height_change,
                        style: "padding: 6px 8px; background-color: #09090b; border: 1px solid #27272a; border-radius: 8px; color: #fff; font-family: monospace; font-size: 12px; outline: none;"
                    }
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { style: "font-size: 10px; font-weight: bold; color: #71717a; text-transform: uppercase; letter-spacing: 0.05em;", "Depth (mm)" }
                    input {
                        r#type: "number",
                        value: "{current_params.dimensions.depth}",
                        oninput: handle_depth_change,
                        style: "padding: 6px 8px; background-color: #09090b; border: 1px solid #27272a; border-radius: 8px; color: #fff; font-family: monospace; font-size: 12px; outline: none;"
                    }
                }
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { style: "font-size: 10px; font-weight: bold; color: #71717a; text-transform: uppercase; letter-spacing: 0.05em;", "Wall (mm)" }
                    input {
                        r#type: "number",
                        value: "{current_params.wall_thickness}",
                        oninput: handle_wall_change,
                        style: "padding: 6px 8px; background-color: #09090b; border: 1px solid #27272a; border-radius: 8px; color: #fff; font-family: monospace; font-size: 12px; outline: none;"
                    }
                }
            }

            div { style: "height: 1px; background-color: #27272a; margin: 8px 0;" }

            // Material selector
            div {
                style: "display: flex; flex-direction: column; gap: 8px; margin-bottom: 16px;",
                label { style: "font-size: 12px; font-weight: bold; color: #71717a; text-transform: uppercase; letter-spacing: 0.05em;", "Material" }
                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px;",
                    button {
                        style: format!("display: flex; align-items: center; gap: 6px; padding: 6px 10px; border-radius: 8px; font-size: 12px; border: 1px solid {}; background-color: {}; color: {}; cursor: pointer;",
                            if current_params.material == "pla" { "#fff" } else { "#27272a" },
                            if current_params.material == "pla" { "#fff" } else { "#09090b" },
                            if current_params.material == "pla" { "#000" } else { "#a1a1aa" }
                        ),
                        onclick: move |_| handle_material_select("pla"),
                        div { style: "width: 16px; height: 16px; border-radius: 50%; background-color: #cccccc; border: 1px solid #000;" }
                        "PLA"
                    }
                    button {
                        style: format!("display: flex; align-items: center; gap: 6px; padding: 6px 10px; border-radius: 8px; font-size: 12px; border: 1px solid {}; background-color: {}; color: {}; cursor: pointer;",
                            if current_params.material == "abs" { "#fff" } else { "#27272a" },
                            if current_params.material == "abs" { "#fff" } else { "#09090b" },
                            if current_params.material == "abs" { "#000" } else { "#a1a1aa" }
                        ),
                        onclick: move |_| handle_material_select("abs"),
                        div { style: "width: 16px; height: 16px; border-radius: 50%; background-color: #aaaaaa; border: 1px solid #000;" }
                        "ABS"
                    }
                    button {
                        style: format!("display: flex; align-items: center; gap: 6px; padding: 6px 10px; border-radius: 8px; font-size: 12px; border: 1px solid {}; background-color: {}; color: {}; cursor: pointer;",
                            if current_params.material == "petg" { "#fff" } else { "#27272a" },
                            if current_params.material == "petg" { "#fff" } else { "#09090b" },
                            if current_params.material == "petg" { "#000" } else { "#a1a1aa" }
                        ),
                        onclick: move |_| handle_material_select("petg"),
                        div { style: "width: 16px; height: 16px; border-radius: 50%; background-color: #dddddd; border: 1px solid #000;" }
                        "PETG"
                    }
                    button {
                        style: format!("display: flex; align-items: center; gap: 6px; padding: 6px 10px; border-radius: 8px; font-size: 12px; border: 1px solid {}; background-color: {}; color: {}; cursor: pointer;",
                            if current_params.material == "aluminum" { "#fff" } else { "#27272a" },
                            if current_params.material == "aluminum" { "#fff" } else { "#09090b" },
                            if current_params.material == "aluminum" { "#000" } else { "#a1a1aa" }
                        ),
                        onclick: move |_| handle_material_select("aluminum"),
                        div { style: "width: 16px; height: 16px; border-radius: 50%; background-color: #999999; border: 1px solid #000;" }
                        "Aluminum"
                    }
                }
            }

            // Vent panel
            div {
                style: "display: flex; flex-direction: column; gap: 8px; padding: 12px; background-color: #09090b; border: 1px solid #27272a; border-radius: 12px; margin-bottom: 16px;",
                h3 { style: "font-size: 10px; font-weight: bold; color: #71717a; text-transform: uppercase; letter-spacing: 0.05em; margin: 0 0 4px 0;", "Ventilation Array" }
                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 8px;",
                    div {
                        style: "display: flex; flex-direction: column; gap: 4px;",
                        label { style: "font-size: 8px; font-weight: bold; color: #71717a; text-transform: uppercase;", "Diameter" }
                        input {
                            r#type: "number",
                            value: "{current_params.vent_config.hole_diameter}",
                            oninput: handle_dia_change,
                            style: "padding: 4px; background-color: #09090b; border: 1px solid #27272a; border-radius: 6px; color: #fff; font-family: monospace; font-size: 10px; outline: none; width: 100%; box-sizing: border-box;"
                        }
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 4px;",
                        label { style: "font-size: 8px; font-weight: bold; color: #71717a; text-transform: uppercase;", "Spacing" }
                        input {
                            r#type: "number",
                            value: "{current_params.vent_config.spacing}",
                            oninput: handle_spacing_change,
                            style: "padding: 4px; background-color: #09090b; border: 1px solid #27272a; border-radius: 6px; color: #fff; font-family: monospace; font-size: 10px; outline: none; width: 100%; box-sizing: border-box;"
                        }
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 4px;",
                        label { style: "font-size: 8px; font-weight: bold; color: #71717a; text-transform: uppercase;", "Quantity" }
                        input {
                            r#type: "number",
                            value: "{current_params.vent_config.quantity}",
                            oninput: handle_qty_change,
                            style: "padding: 4px; background-color: #09090b; border: 1px solid #27272a; border-radius: 6px; color: #fff; font-family: monospace; font-size: 10px; outline: none; width: 100%; box-sizing: border-box;"
                        }
                    }
                }
            }

            button {
                style: "margin-top: auto; padding: 10px; background-color: #fff; color: #000; font-size: 12px; font-weight: bold; border-radius: 8px; border: none; cursor: pointer; transition: background-color 0.2s; display: flex; align-items: center; justify-content: center; gap: 8px;",
                onclick: on_generate,
                "{generate_text}"
            }
        }
    }
}
