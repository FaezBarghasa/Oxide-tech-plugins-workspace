bl_info = {
    "name": "Oxide-Tech Casing Collision Checker",
    "description": "Local clearance and collision verification loop directly in the Viewport querying the native UniFFI-rs Python interface with BYOK validation.",
    "author": "Oxide Tech Engineering",
    "version": (1, 1, 0),
    "blender": (4, 0, 0),
    "location": "View3D > Sidebar > Oxide Panel",
    "category": "3D View"
}

import bpy
import sys
import os
import traceback
import json
import urllib.request
import urllib.error

# JIT Import resolution path
uniffi_path = "/usr/lib/oxide_tech/bindings/python"
if uniffi_path not in sys.path:
    sys.path.append(uniffi_path)

try:
    import oxide_core
    OXIDE_CORE_LOADED = True
except ImportError:
    OXIDE_CORE_LOADED = False
    print(f"ERROR: Could not load oxide_core from {uniffi_path}")

# Global controller reference
_oxide_controller = None

class OxideProperties(bpy.types.PropertyGroup):
    # Masked BYOK Property Registration
    gemini_api_key: bpy.props.StringProperty(
        name="Gemini API Key",
        subtype='PASSWORD',
        description="Your personal Gemini API Key for Oxide-Tech Agent actions",
        default=""
    )
    pcb_width: bpy.props.FloatProperty(
        name="PCB Width",
        description="Target PCB Width in mm",
        default=100.0,
        min=1.0
    )
    pcb_length: bpy.props.FloatProperty(
        name="PCB Length",
        description="Target PCB Length in mm",
        default=100.0,
        min=1.0
    )
    enclosure_thickness: bpy.props.FloatProperty(
        name="Enclosure Thickness",
        description="Wall thickness in mm",
        default=2.0,
        min=0.1
    )
    target_clearance: bpy.props.FloatProperty(
        name="Target Clearance",
        description="Clearance threshold in mm",
        default=1.5,
        min=0.0
    )


class VIEW3D_PT_oxide_panel(bpy.types.Panel):
    bl_space_type = 'VIEW_3D'
    bl_region_type = 'UI'
    bl_category = 'Oxide'
    bl_label = "Oxide Collision Checker"

    def draw(self, context):
        layout = self.layout
        scene = context.scene
        props = scene.oxide_props

        if not OXIDE_CORE_LOADED:
            layout.label(text="Oxide Core Library not found!", icon='ERROR')
            return

        # Settings group
        layout.label(text="BYOK Credentials:")
        layout.prop(props, "gemini_api_key")
        layout.operator("oxide.validate_byok_key", text="Validate Key", icon='KEY_DECORATED')

        layout.separator()
        layout.label(text="Physical Parameters:")
        layout.prop(props, "pcb_width")
        layout.prop(props, "pcb_length")
        layout.prop(props, "enclosure_thickness")
        layout.prop(props, "target_clearance")
        
        layout.operator("oxide.reinitialize_controller", text="Apply Parameters", icon='FILE_REFRESH')


class OXIDE_OT_validate_byok_key(bpy.types.Operator):
    bl_idname = "oxide.validate_byok_key"
    bl_label = "Validate API Key"
    bl_description = "Validates the Gemini API key against Google AI Studio"

    def execute(self, context):
        props = context.scene.oxide_props
        api_key = props.gemini_api_key.strip()
        
        if not api_key:
            self.report({'WARNING'}, "API Key is empty.")
            return {'CANCELLED'}
            
        url = f"https://generativelanguage.googleapis.com/v1beta/models?key={api_key}"
        try:
            req = urllib.request.Request(url, headers={'User-Agent': 'Oxide-Tech-Blender-Validator'})
            with urllib.request.urlopen(req, timeout=5.0) as response:
                if response.getcode() == 200:
                    self.report({'INFO'}, "API Key validated successfully.")
                    return {'FINISHED'}
        except urllib.error.HTTPError as e:
            try:
                err_data = json.loads(e.read().decode('utf-8'))
                err_msg = err_data.get("error", {}).get("message", "HTTP Error")
                self.report({'ERROR'}, f"Validation failed: {err_msg}")
            except Exception:
                self.report({'ERROR'}, f"HTTP Error {e.code}")
        except Exception as e:
            self.report({'ERROR'}, f"Network validation failed: {str(e)}")
            
        return {'CANCELLED'}


class OXIDE_OT_reinitialize_controller(bpy.types.Operator):
    bl_idname = "oxide.reinitialize_controller"
    bl_label = "Reinitialize Oxide Controller"
    bl_description = "Applies current properties to the UniFFI Rust GJK solver"

    def execute(self, context):
        global _oxide_controller
        props = context.scene.oxide_props
        
        if not OXIDE_CORE_LOADED:
            self.report({'ERROR'}, "oxide_core module not available.")
            return {'CANCELLED'}

        try:
            physical_dim = oxide_core.PhysicalDimension(
                width=props.pcb_width,
                length=props.pcb_length,
                wall_thickness=props.enclosure_thickness,
                clearance=props.target_clearance
            )
            _oxide_controller = oxide_core.SystemController(physical_dim)
            self.report({'INFO'}, "Successfully initialized Oxide SystemController.")
        except Exception as e:
            self.report({'ERROR'}, f"Failed to initialize Oxide SystemController: {str(e)}")
            traceback.print_exc()
            return {'CANCELLED'}
            
        return {'FINISHED'}


def update_pcb_material(obj, has_collision):
    """Dynamically manipulates material NodeTree base color & emission values on collision."""
    if not obj or not obj.data.materials:
        return
        
    mat = obj.data.materials[0]
    if not mat.use_nodes:
        mat.use_nodes = True
        
    nodes = mat.node_tree.nodes
    principled_bsdf = None
    
    # Locate BSDF Principled nodes
    for node in nodes:
        if node.type == 'BSDF_PRINCIPLED':
            principled_bsdf = node
            break
            
    if not principled_bsdf:
        return
        
    # Red for collision, green for clean
    if has_collision:
        principled_bsdf.inputs['Base Color'].default_value = (0.8, 0.05, 0.05, 1.0)
        principled_bsdf.inputs['Emission Color'].default_value = (1.0, 0.0, 0.0, 1.0)
        # Handle emission strength differences between blender versions
        if 'Emission Strength' in principled_bsdf.inputs:
            principled_bsdf.inputs['Emission Strength'].default_value = 2.0
    else:
        principled_bsdf.inputs['Base Color'].default_value = (0.05, 0.8, 0.1, 1.0)
        principled_bsdf.inputs['Emission Color'].default_value = (0.0, 1.0, 0.0, 1.0)
        if 'Emission Strength' in principled_bsdf.inputs:
            principled_bsdf.inputs['Emission Strength'].default_value = 0.0


@bpy.app.handlers.persistent
def oxide_depsgraph_update_post_handler(scene, depsgraph):
    """Graph update handler to perform GJK sweeps on every frame updates."""
    global _oxide_controller
    
    if not _oxide_controller:
        return
        
    try:
        pcb_obj = bpy.data.objects.get("PCB_Target")
        casing_obj = bpy.data.objects.get("Enclosure_Casing")
        
        if not pcb_obj or not casing_obj:
            return
            
        pcb_eval = pcb_obj.evaluated_get(depsgraph)
        
        # Dimensions in Blender coordinates (meters to mm)
        pcb_x_mm = pcb_eval.location.x * 1000.0
        pcb_y_mm = pcb_eval.location.y * 1000.0
        pcb_height_mm = pcb_eval.dimensions.z * 1000.0
        
        has_collision = _oxide_controller.run_gjk_clearance_solver(
            designator="PCB_Target",
            x=pcb_x_mm,
            y=pcb_y_mm,
            height=pcb_height_mm
        )
        
        update_pcb_material(pcb_obj, has_collision)
        
    except Exception as e:
        print(f"Oxide Depsgraph Handler Error: {e}")
        traceback.print_exc()


# Headless interface exposed for external scripts/eda-agent coordination
def headless_clearance_sweep(api_key=None, target_pcb_name="PCB_Target"):
    """Exposes importable helper module function for eda-agent headless runs."""
    global _oxide_controller
    
    # Resolve API Key
    resolved_key = api_key
    if not resolved_key:
        scene = bpy.context.scene
        if hasattr(scene, "oxide_props"):
            resolved_key = scene.oxide_props.gemini_api_key
            
    if not resolved_key:
        print("WARNING: No Gemini API Key supplied for headless sweep.")
        
    pcb_obj = bpy.data.objects.get(target_pcb_name)
    if not pcb_obj:
        return {"success": False, "message": f"Object {target_pcb_name} not found."}
        
    if not _oxide_controller:
        # Fallback initializer
        if not OXIDE_CORE_LOADED:
            return {"success": False, "message": "oxide_core not loaded."}
            
        dim = oxide_core.PhysicalDimension(
            width=100.0, length=100.0, wall_thickness=2.0, clearance=1.5
        )
        _oxide_controller = oxide_core.SystemController(dim)
        
    pcb_x_mm = pcb_obj.location.x * 1000.0
    pcb_y_mm = pcb_obj.location.y * 1000.0
    pcb_height_mm = pcb_obj.dimensions.z * 1000.0
    
    has_collision = _oxide_controller.run_gjk_clearance_solver(
        designator=target_pcb_name,
        x=pcb_x_mm,
        y=pcb_y_mm,
        height=pcb_height_mm
    )
    
    return {
        "success": not has_collision,
        "collision": has_collision,
        "coordinates": {"x": pcb_x_mm, "y": pcb_y_mm, "height": pcb_height_mm}
    }


classes = (
    OxideProperties,
    VIEW3D_PT_oxide_panel,
    OXIDE_OT_validate_byok_key,
    OXIDE_OT_reinitialize_controller,
)

def register():
    for cls in classes:
        bpy.utils.register_class(cls)
        
    bpy.types.Scene.oxide_props = bpy.props.PointerProperty(type=OxideProperties)
    
    if oxide_depsgraph_update_post_handler not in bpy.app.handlers.depsgraph_update_post:
        bpy.app.handlers.depsgraph_update_post.append(oxide_depsgraph_update_post_handler)

def unregister():
    global _oxide_controller
    if oxide_depsgraph_update_post_handler in bpy.app.handlers.depsgraph_update_post:
        bpy.app.handlers.depsgraph_update_post.remove(oxide_depsgraph_update_post_handler)
        
    for cls in reversed(classes):
        bpy.utils.unregister_class(cls)
        
    if hasattr(bpy.types.Scene, "oxide_props"):
        del bpy.types.Scene.oxide_props
        
    _oxide_controller = None

if __name__ == "__main__":
    register()
