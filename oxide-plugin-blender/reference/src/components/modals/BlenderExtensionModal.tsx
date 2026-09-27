import React from 'react';
import { X, Copy, Check, Download, Puzzle, FileText } from 'lucide-react';
import { useEnclosureStore } from '../../state/enclosureStore';

interface BlenderExtensionModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export function BlenderExtensionModal({ isOpen, onClose }: BlenderExtensionModalProps) {
  const { currentEnclosure } = useEnclosureStore();
  const [copied, setCopied] = React.useState(false);

  if (!isOpen) return null;

  // Dynamically generate the precise Blender Python script based on parameters
  const generateBlenderPython = () => {
    const { dimensions, wallThickness, material, ventConfig } = currentEnclosure;
    return `import bpy
import math

class OBJECT_OT_GenerateParametricEnclosure(bpy.types.Operator):
    """Generate a Parametric Enclosure with Custom Ventilation"""
    bl_idname = "object.generate_parametric_enclosure"
    bl_label = "Generate Parametric Enclosure"
    bl_options = {'REGISTER', 'UNDO'}

    # Interactive Parametric dimensions (preconfigured from control plane)
    width: bpy.props.FloatProperty(name="Width (mm)", default=${dimensions.width}, min=50, max=500)
    height: bpy.props.FloatProperty(name="Height (mm)", default=${dimensions.height}, min=50, max=500)
    depth: bpy.props.FloatProperty(name="Depth (mm)", default=${dimensions.depth}, min=30, max=500)
    wall_thickness: bpy.props.FloatProperty(name="Wall Thickness (mm)", default=${wallThickness}, min=1.5, max=10)
    
    vent_diameter: bpy.props.FloatProperty(name="Vent Diameter (mm)", default=${ventConfig.holeDiameter}, min=2, max=50)
    vent_spacing: bpy.props.FloatProperty(name="Vent Spacing (mm)", default=${ventConfig.spacing}, min=2, max=100)
    vent_quantity: bpy.props.IntProperty(name="Vent Quantity", default=${ventConfig.quantity}, min=0, max=100)

    def execute(self, context):
        # 1. Clear default system cube
        if "Cube" in bpy.data.objects:
            bpy.data.objects.remove(bpy.data.objects["Cube"], do_unlink=True)

        scale_factor = 0.001 # mm to meters default scale
        w = self.width * scale_factor
        h = self.height * scale_factor
        d = self.depth * scale_factor
        t = self.wall_thickness * scale_factor

        # 2. Add Outer Shell Primitive
        bpy.ops.mesh.primitive_cube_add(size=1.0, location=(0, 0, h/2))
        outer_box = context.active_object
        outer_box.name = "Enclosure_Outer"
        outer_box.scale = (w, d, h)
        bpy.ops.object.transform_apply(scale=True)

        # 3. Add Inner Hollow Primitive
        bpy.ops.mesh.primitive_cube_add(size=1.0, location=(0, 0, h/2))
        inner_box = context.active_object
        inner_box.name = "Enclosure_Inner"
        inner_box.scale = (w - t*2, d - t*2, h) # Hollowed body
        bpy.ops.object.transform_apply(scale=True)

        # Apply Boolean Difference Modifier
        bool_mod = outer_box.modifiers.new(name="ShellHollow", type='BOOLEAN')
        bool_mod.operation = 'DIFFERENCE'
        bool_mod.object = inner_box
        context.view_layer.objects.active = outer_box
        bpy.ops.object.modifier_apply(modifier="ShellHollow")
        bpy.data.objects.remove(inner_box, do_unlink=True)

        # 4. Add Array Ventilation Holes if specified
        if self.vent_quantity > 0:
            vd = self.vent_diameter * scale_factor
            vs = self.vent_spacing * scale_factor
            
            # Create primitive Cylinder for vent cutting
            bpy.ops.mesh.primitive_cylinder_add(
                radius=vd / 2, 
                depth=t * 4, 
                location=(0, 0, h - (t/2))
            )
            vent_cutter = context.active_object
            vent_cutter.name = "Vent_Cutter"
            
            # Setup vent array pattern on the casing surface
            for idx in range(self.vent_quantity):
                offset = (idx - (self.vent_quantity - 1) / 2) * vs
                if idx > 0:
                    bpy.ops.object.duplicate()
                curr_vent = context.active_object
                curr_vent.location.x = offset
                
                # Apply Boolean cutout operation
                sub_mod = outer_box.modifiers.new(name=f"VentCut_{idx}", type='BOOLEAN')
                sub_mod.operation = 'DIFFERENCE'
                sub_mod.object = curr_vent
                bpy.ops.object.modifier_apply(modifier=f"VentCut_{idx}")
                bpy.data.objects.remove(curr_vent, do_unlink=True)

        self.report({'INFO'}, f"Generated model with dimensions: {self.width}x{self.height}x{self.depth}mm")
        return {'FINISHED'}

def register():
    bpy.utils.register_class(OBJECT_OT_GenerateParametricEnclosure)

def unregister():
    bpy.utils.unregister_class(OBJECT_OT_GenerateParametricEnclosure)

if __name__ == "__main__":
    register()
    # Auto-run operator instantly for demonstration
    bpy.ops.object.generate_parametric_enclosure()
`;
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(generateBlenderPython());
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleDownload = () => {
    const element = document.createElement("a");
    const file = new Blob([generateBlenderPython()], { type: 'text/plain' });
    element.href = URL.createObjectURL(file);
    element.download = "blender_cad_parametric_extension.py";
    document.body.appendChild(element);
    element.click();
    document.body.removeChild(element);
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-[#09090b]/80 backdrop-blur-md p-4">
      <div className="relative bg-[#18181b] border border-[#27272a] rounded-[2rem] w-full max-w-3xl flex flex-col max-h-[85vh] shadow-2xl overflow-hidden text-[#fafafa]">
        
        {/* Modal Close */}
        <button 
          onClick={onClose}
          className="absolute top-6 right-6 p-2 rounded-full bg-[#09090b] border border-[#27272a] hover:bg-[#27272a] transition"
        >
          <X size={18} className="text-[#a1a1aa] hover:text-white" />
        </button>

        {/* Modal Header */}
        <div className="p-8 border-b border-[#27272a] flex items-center gap-4">
          <div className="bg-[#27272a] p-3 rounded-2xl border border-[#3f3f46]">
            <Puzzle size={24} className="text-white" />
          </div>
          <div>
            <h2 className="text-xl font-bold text-white tracking-tight">Blender Extension Script</h2>
            <p className="text-sm text-[#a1a1aa] mt-0.5">Automate and render parametric mechanical designs in Blender.</p>
          </div>
        </div>

        {/* Modal Content */}
        <div className="p-8 flex-grow overflow-y-auto space-y-6">
          <div className="bg-[#09090b] border border-[#27272a] p-5 rounded-2xl flex flex-col gap-2">
            <span className="text-xs font-bold uppercase tracking-widest text-[#71717a]">Installation Instructions</span>
            <ul className="list-decimal list-inside space-y-2 text-sm text-[#a1a1aa] leading-relaxed">
              <li>Copy or click **Download Script (.py)** below.</li>
              <li>Open your **Blender CAD** interface workspace.</li>
              <li>Switch to the **Scripting tab** and click **New** to create a document.</li>
              <li>Paste this script and click **Run Script** (or key <kbd className="px-1.5 py-0.5 bg-[#27272a] rounded text-xs font-mono text-white">Alt+P</kbd>).</li>
              <li>A custom parametric enclosure object generates instantly according to your customized specifications!</li>
            </ul>
          </div>

          <div className="flex justify-between items-center bg-[#09090b] border-t border-x border-[#27272a] px-5 py-3 rounded-t-2xl shrink-0">
            <span className="text-xs font-mono text-[#a1a1aa] flex items-center gap-1.5">
              <FileText size={14} /> CAD_GENERATOR.PY
            </span>
            <div className="flex gap-2">
              <button 
                onClick={handleCopy}
                className="flex items-center gap-1.5 px-3.5 py-1.5 bg-[#27272a] hover:bg-[#3f3f46] text-white rounded-lg text-xs font-bold transition"
              >
                {copied ? (
                  <>
                    <Check size={14} className="text-green-500" /> Copied!
                  </>
                ) : (
                  <>
                    <Copy size={14} /> Copy Script
                  </>
                )}
              </button>
              <button 
                onClick={handleDownload}
                className="flex items-center gap-1.5 px-3.5 py-1.5 bg-white text-black hover:bg-gray-200 rounded-lg text-xs font-bold transition"
              >
                <Download size={14} /> Download (.py)
              </button>
            </div>
          </div>

          <pre className="bg-[#09090b] border border-t-0 border-[#27272a] p-5 rounded-b-2xl overflow-x-auto text-xs font-mono leading-relaxed text-[#a1a1aa] max-h-60 select-all">
            {generateBlenderPython()}
          </pre>
        </div>
      </div>
    </div>
  );
}
