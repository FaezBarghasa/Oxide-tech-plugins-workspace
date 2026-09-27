#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use crate::frontend::state::CADState;
#[cfg(target_arch = "wasm32")]
use crate::frontend::components::{ParametricInputPanel, Scene, ChatPanel};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
#[component]
pub fn BlenderExtensionModal(on_close: EventHandler<()>) -> Element {
    let state: CADState = use_context::<CADState>();
    let params = state.params.read().clone();
    let mut copied = use_signal(|| false);

    let python_script = format!(
        r#"import bpy
import math

class OBJECT_OT_GenerateParametricEnclosure(bpy.types.Operator):
    """Generate a Parametric Enclosure with Custom Ventilation"""
    bl_idname = "object.generate_parametric_enclosure"
    bl_label = "Generate Parametric Enclosure"
    bl_options = {{'REGISTER', 'UNDO'}}

    # Preconfigured parametric dimensions
    width: bpy.props.FloatProperty(name="Width (mm)", default={:.1}, min=50, max=500)
    height: bpy.props.FloatProperty(name="Height (mm)", default={:.1}, min=50, max=500)
    depth: bpy.props.FloatProperty(name="Depth (mm)", default={:.1}, min=30, max=500)
    wall_thickness: bpy.props.FloatProperty(name="Wall Thickness (mm)", default={:.2}, min=1.5, max=10)
    
    vent_diameter: bpy.props.FloatProperty(name="Vent Diameter (mm)", default={:.2}, min=2, max=50)
    vent_spacing: bpy.props.FloatProperty(name="Vent Spacing (mm)", default={:.2}, min=2, max=100)
    vent_quantity: bpy.props.IntProperty(name="Vent Quantity", default={}, min=0, max=100)

    def execute(self, context):
        if "Cube" in bpy.data.objects:
            bpy.data.objects.remove(bpy.data.objects["Cube"], do_unlink=True)

        scale_factor = 0.001
        w = self.width * scale_factor
        h = self.height * scale_factor
        d = self.depth * scale_factor
        t = self.wall_thickness * scale_factor

        bpy.ops.mesh.primitive_cube_add(size=1.0, location=(0, 0, h/2))
        outer_box = context.active_object
        outer_box.name = "Enclosure_Outer"
        outer_box.scale = (w, d, h)
        bpy.ops.object.transform_apply(scale=True)

        bpy.ops.mesh.primitive_cube_add(size=1.0, location=(0, 0, h/2))
        inner_box = context.active_object
        inner_box.name = "Enclosure_Inner"
        inner_box.scale = (w - t*2, d - t*2, h)
        bpy.ops.object.transform_apply(scale=True)

        bool_mod = outer_box.modifiers.new(name="ShellHollow", type='BOOLEAN')
        bool_mod.operation = 'DIFFERENCE'
        bool_mod.object = inner_box
        context.view_layer.objects.active = outer_box
        bpy.ops.object.modifier_apply(modifier="ShellHollow")
        bpy.data.objects.remove(inner_box, do_unlink=True)

        if self.vent_quantity > 0:
            vd = self.vent_diameter * scale_factor
            vs = self.vent_spacing * scale_factor
            
            bpy.ops.mesh.primitive_cylinder_add(
                radius=vd / 2, 
                depth=t * 4, 
                location=(0, 0, h - (t/2))
            )
            vent_cutter = context.active_object
            vent_cutter.name = "Vent_Cutter"
            
            for idx in range(self.vent_quantity):
                offset = (idx - (self.vent_quantity - 1) / 2) * vs
                if idx > 0:
                    bpy.ops.object.duplicate()
                curr_vent = context.active_object
                curr_vent.location.x = offset
                
                sub_mod = outer_box.modifiers.new(name=f"VentCut_{{idx}}", type='BOOLEAN')
                sub_mod.operation = 'DIFFERENCE'
                sub_mod.object = curr_vent
                bpy.ops.object.modifier_apply(modifier=f"VentCut_{{idx}}")
                bpy.data.objects.remove(curr_vent, do_unlink=True)

        self.report({{'INFO'}}, f"Generated model with dimensions: {{self.width}}x{{self.height}}x{{self.depth}}mm")
        return {{'FINISHED'}}

def register():
    bpy.utils.register_class(OBJECT_OT_GenerateParametricEnclosure)

def unregister():
    bpy.utils.unregister_class(OBJECT_OT_GenerateParametricEnclosure)

if __name__ == "__main__":
    register()
    bpy.ops.object.generate_parametric_enclosure()
"#,
        params.dimensions.width,
        params.dimensions.height,
        params.dimensions.depth,
        params.wall_thickness,
        params.vent_config.hole_diameter,
        params.vent_config.spacing,
        params.vent_config.quantity
    );

    let handle_copy = {
        let python_script = python_script.clone();
        move |_| {
            let window = web_sys::window().unwrap();
            let navigator = window.navigator();
            let clipboard = navigator.clipboard();
            let _ = clipboard.write_text(&python_script);
            copied.set(true);
        }
    };

    let handle_download = {
        let python_script = python_script.clone();
        move |_| {
            let document = web_sys::window().unwrap().document().unwrap();
            let a = document.create_element("a").unwrap();
            let encoded = urlencoding::encode(&python_script);
            let href = format!("data:text/plain;charset=utf-8,{}", encoded);
            let _ = a.set_attribute("href", &href);
            let _ = a.set_attribute("download", "blender_cad_parametric_extension.py");
            if let Some(body) = document.body() {
                let _ = body.append_child(&a);
                let html_elem = a.dyn_into::<web_sys::HtmlElement>().unwrap();
                html_elem.click();
                let _ = body.remove_child(&html_elem);
            }
        }
    };

    rsx! {
        div {
            style: "position: fixed; inset: 0; z-index: 50; display: flex; align-items: center; justify-content: center; background-color: rgba(9, 9, 11, 0.8); backdrop-filter: blur(8px); padding: 16px; box-sizing: border-box;",
            div {
                style: "position: relative; background-color: #18181b; border: 1px solid #27272a; border-radius: 24px; width: 100%; max-width: 600px; display: flex; flex-direction: column; max-height: 80vh; overflow-y: auto; color: #fafafa; padding: 24px; box-sizing: border-box;",
                
                // Close button
                button {
                    onclick: move |_| on_close.call(()),
                    style: "position: absolute; top: 16px; right: 16px; background-color: #09090b; border: 1px solid #27272a; color: #a1a1aa; border-radius: 50%; width: 32px; height: 32px; cursor: pointer; display: flex; align-items: center; justify-content: center;",
                    "×"
                }

                h2 { style: "font-size: 18px; font-weight: bold; margin: 0 0 8px 0; color: #fff;", "Blender Extension Script" }
                p { style: "font-size: 12px; color: #a1a1aa; margin: 0 0 16px 0;", "Automate and render parametric mechanical designs in Blender." }

                div {
                    style: "background-color: #09090b; border: 1px solid #27272a; padding: 12px; border-radius: 12px; margin-bottom: 16px; font-size: 11px; color: #a1a1aa; line-height: 1.5;",
                    strong { style: "color: #fff; display: block; margin-bottom: 4px;", "Installation Instructions:" }
                    ol {
                        style: "margin: 0; padding-left: 16px;",
                        li { "Copy or click Download Script (.py) below." }
                        li { "Open your Blender CAD interface workspace." }
                        li { "Switch to the Scripting tab and click New to create a document." }
                        li { "Paste this script and press Run Script (Alt+P)." }
                    }
                }

                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;",
                    span { style: "font-size: 10px; font-family: monospace; color: #a1a1aa;", "CAD_GENERATOR.PY" }
                    div {
                        style: "display: flex; gap: 8px;",
                        button {
                            onclick: handle_copy,
                            style: "padding: 6px 12px; background-color: #27272a; color: #fff; font-size: 11px; font-weight: bold; border-radius: 6px; border: none; cursor: pointer;",
                            if *copied.read() { "Copied!" } else { "Copy Script" }
                        }
                        button {
                            onclick: handle_download,
                            style: "padding: 6px 12px; background-color: #fff; color: #000; font-size: 11px; font-weight: bold; border-radius: 6px; border: none; cursor: pointer;",
                            "Download (.py)"
                        }
                    }
                }

                pre {
                    style: "background-color: #09090b; border: 1px solid #27272a; padding: 12px; border-radius: 12px; font-family: monospace; font-size: 10px; color: #a1a1aa; overflow-x: auto; max-height: 200px; margin: 0; white-space: pre;",
                    "{python_script}"
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
pub fn MainLayout() -> Element {
    let mut state: CADState = use_context::<CADState>();
    let is_modal_open = *state.is_modal_open.read();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100vh; width: 100vw; background-color: #09090b; color: #fafafa; overflow: hidden; box-sizing: border-box;",
            
            // Header
            header {
                style: "display: flex; align-items: center; justify-content: space-between; padding: 12px 20px; background-color: #18181b; border: 1px solid #27272a; border-radius: 16px; margin: 12px; box-sizing: border-box;",
                
                // Brand
                div {
                    style: "display: flex; align-items: center; gap: 12px;",
                    div {
                        style: "background-color: #27272a; padding: 6px; border-radius: 8px; display: flex; align-items: center; justify-content: center;",
                        span { style: "font-size: 16px;", "📦" }
                    }
                    div {
                        h1 { style: "font-size: 14px; font-weight: bold; margin: 0; color: #fff;", "CAD Control Plane" }
                        p { style: "font-size: 9px; color: #a1a1aa; font-family: monospace; margin: 2px 0 0 0;", "SYSTEM_VER ✨ v2.1.0" }
                    }
                }

                // Status Indicator
                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    span { style: "width: 8px; height: 8px; border-radius: 50%; background-color: #22c55e; box-shadow: 0 0 8px #22c55e;" }
                    span { style: "font-size: 10px; font-weight: bold; font-family: monospace; color: #22c55e;", "Blender Connected" }
                }

                // Actions
                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    button {
                        onclick: move |_| state.is_modal_open.set(true),
                        style: "padding: 6px 12px; background-color: #fff; color: #000; font-size: 11px; font-weight: bold; border-radius: 6px; border: none; cursor: pointer; display: flex; align-items: center; gap: 4px;",
                        "🔌 Blender Extension"
                    }
                    button {
                        onclick: move |_| state.undo(),
                        style: "padding: 6px 12px; background-color: #09090b; border: 1px solid #27272a; color: #fff; font-size: 11px; font-weight: bold; border-radius: 6px; cursor: pointer;",
                        "Undo"
                    }
                    button {
                        onclick: move |_| state.redo(),
                        style: "padding: 6px 12px; background-color: #09090b; border: 1px solid #27272a; color: #fff; font-size: 11px; font-weight: bold; border-radius: 6px; cursor: pointer;",
                        "Redo"
                    }
                }
            }

            // Main Bento Grid Content
            main {
                style: "display: flex; flex: 1; overflow: hidden; gap: 12px; padding: 0 12px 12px 12px; box-sizing: border-box;",
                
                // Left Configuration
                div {
                    style: "background-color: #18181b; border: 1px solid #27272a; border-radius: 16px; overflow: hidden; width: 280px; shrink: 0; height: 100%;",
                    ParametricInputPanel {}
                }

                // Center 3D viewport
                div {
                    style: "flex-grow: 1; position: relative; background-color: #18181b; border: 1px solid #27272a; border-radius: 16px; overflow: hidden; height: 100%; display: flex; flex-direction: column;",
                    div {
                        style: "position: absolute; top: 16px; left: 16px; z-index: 10; display: flex; gap: 6px;",
                        div { style: "background-color: rgba(9, 9, 11, 0.8); border: 1px solid #27272a; padding: 4px 8px; border-radius: 6px; font-family: monospace; font-size: 9px; color: #a1a1aa; font-weight: bold; text-transform: uppercase;", "Perspective" }
                        div { style: "background-color: rgba(9, 9, 11, 0.8); border: 1px solid #27272a; padding: 4px 8px; border-radius: 6px; font-family: monospace; font-size: 9px; color: #a1a1aa; font-weight: bold; text-transform: uppercase;", "Shaded" }
                    }
                    Scene {}
                }

                // Right Chat Assistant
                ChatPanel {}
            }

            if is_modal_open {
                BlenderExtensionModal {
                    on_close: move |_| state.is_modal_open.set(false)
                }
            }
        }
    }
}
