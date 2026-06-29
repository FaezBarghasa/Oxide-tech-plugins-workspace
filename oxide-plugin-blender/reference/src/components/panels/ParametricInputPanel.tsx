import React, { useState } from 'react';
import { useEnclosureStore } from '../../state/enclosureStore';
import { sendBlenderRequest } from '../../hooks/useBlenderSocket';
import { validateEnclosureParams } from '../../utils/validators';
import { EnclosureParams, BlenderResult } from '../../types/blender';
import { DIMENSION_CONSTRAINTS } from '../../utils/validators';

import { DimensionInput } from './DimensionInput';
import { MaterialSelector } from './MaterialSelector';
import { VentConfigPanel } from './VentConfigPanel';

export function ParametricInputPanel() {
  const { currentEnclosure: params, updateParams, setMesh } = useEnclosureStore();
  const [isGenerating, setIsGenerating] = useState(false);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);
  
  const handleDimensionChange = (key: keyof EnclosureParams['dimensions'], value: number) => {
    const newParams = {
      ...params,
      dimensions: { ...params.dimensions, [key]: value }
    };
    
    // Allow typing, validate on submit or show inline warning
    updateParams(newParams);
  };
  
  const handleGenerate = async () => {
    setErrorMsg(null);
    const validation = validateEnclosureParams(params);
    
    if (!validation.valid) {
      setErrorMsg(validation.errors[0]);
      return;
    }
    
    setIsGenerating(true);
    try {
      const meshData = await sendBlenderRequest<BlenderResult>('generate_enclosure', params);
      // For mock execution, we are passing an empty ArrayBuffer, and the scene will use CSG or default primitives to preview.
      // In real scenario, loadMeshFromBinary(meshData.mesh_buffer)
      setMesh(null as any, meshData.mesh_buffer);
    } catch (error: any) {
      setErrorMsg(error.message || 'Generation failed');
    } finally {
      setIsGenerating(false);
    }
  };
  
  return (
    <div className="flex flex-col gap-4 p-4 h-full overflow-y-auto">
      <div>
        <h2 className="text-sm font-bold text-white uppercase tracking-wider">CAD Configuration</h2>
        <p className="text-[11px] text-[#a1a1aa] mt-0.5">Define mechanical CAD parameters.</p>
      </div>
      
      {errorMsg && (
        <div className="bg-red-500/10 border border-red-500/50 text-red-400 text-xs p-2.5 rounded-lg">
          {errorMsg}
        </div>
      )}
      
      <div className="grid grid-cols-2 gap-3">
        <DimensionInput
          label="Width (mm)"
          value={params.dimensions.width}
          min={DIMENSION_CONSTRAINTS.MIN_WIDTH}
          max={DIMENSION_CONSTRAINTS.MAX_WIDTH}
          onChange={(value) => handleDimensionChange('width', value)}
        />
        <DimensionInput
          label="Height (mm)"
          value={params.dimensions.height}
          onChange={(value) => handleDimensionChange('height', value)}
        />
        <DimensionInput
          label="Depth (mm)"
          value={params.dimensions.depth}
          onChange={(value) => handleDimensionChange('depth', value)}
        />
        <DimensionInput
          label="Wall (mm)"
          value={params.wallThickness}
          onChange={(value) => updateParams({ ...params, wallThickness: value })}
        />
      </div>
      
      <div className="bg-[#27272a] h-px w-full my-1"></div>

      <MaterialSelector
        selectedMaterial={params.material}
        onSelect={(mat) => updateParams({ ...params, material: mat })}
      />
      
      <VentConfigPanel
        config={params.ventConfig}
        onChange={(config) => updateParams({ ...params, ventConfig: config })}
      />
      
      <button
        disabled={isGenerating}
        onClick={handleGenerate}
        className="mt-auto px-4 py-2 bg-white text-black text-xs font-bold rounded-lg hover:bg-gray-200 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-1.5 shadow-sm"
      >
        {isGenerating ? (
          <>
            <svg className="animate-spin -ml-1 mr-2 h-4 w-4 text-black" fill="none" viewBox="0 0 24 24">
              <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
              <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            Generating Model...
          </>
        ) : 'Generate Enclosure'}
      </button>
    </div>
  );
}
