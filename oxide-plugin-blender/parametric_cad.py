import bpy
import os
from oxide_tech_client import OxideClient

class OxideEnclosureGenerator:
    def __init__(self, project_id: str):
        self.client = OxideClient()
        self.project_id = project_id
    
    def generate_from_pcb(self) -> str:
        """Generate enclosure based on PCB dimensions from graph"""
        ctx = self.client.get_project_context(self.project_id)
        pcb_dims = ctx.get_pcb_dimensions()
        max_height = ctx.get_max_component_height()
        connectors = ctx.get_edge_connectors()
        
        # Clear scene
        bpy.ops.object.select_all(action='SELECT')
        bpy.ops.object.delete()
        
        # Create base enclosure
        bpy.ops.mesh.primitive_cube_add(size=1, location=(0, 0, 0))
        enclosure = bpy.context.active_object
        enclosure.scale = (
            pcb_dims.width + 10,
            pcb_dims.height + 10,
            max_height + 10
        )
        bpy.ops.object.transform_apply(scale=True)
        
        # Carve connector cutouts
        for conn in connectors:
            bpy.ops.mesh.primitive_cube_add(size=1, location=conn.position)
            cutout = bpy.context.active_object
            cutout.scale = conn.dimensions
            
            # Select the enclosure as active object to apply modifier
            bpy.context.view_layer.objects.active = enclosure
            
            modifier = enclosure.modifiers.new(
                name=f"Cutout_{conn.name}",
                type='BOOLEAN'
            )
            modifier.operation = 'DIFFERENCE'
            modifier.object = cutout
            bpy.ops.object.modifier_apply(modifier=modifier.name)
            bpy.data.objects.remove(cutout)
        
        # Export STEP
        output_path = f"/tmp/oxide-tech/cad/{self.project_id}_enclosure.step"
        os.makedirs(os.path.dirname(output_path), exist_ok=True)
        bpy.ops.wm.save_as_mainfile(filepath=output_path)
        
        return output_path
