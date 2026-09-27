import React from 'react';
import { MATERIALS, MaterialId } from '../../types/materials';

interface MaterialSelectorProps {
  selectedMaterial: string;
  onSelect: (material: MaterialId) => void;
}

export function MaterialSelector({ selectedMaterial, onSelect }: MaterialSelectorProps) {
  return (
    <div className="flex flex-col gap-2">
      <label className="text-xs font-bold uppercase tracking-widest text-[#71717a] mb-1">Material</label>
      <div className="grid grid-cols-2 gap-2">
        {MATERIALS.map((mat) => (
          <button
            key={mat.id}
            onClick={() => onSelect(mat.id)}
            className={`flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-xs transition-colors ${
              selectedMaterial === mat.id
                ? 'bg-white text-black font-semibold border border-white'
                : 'bg-[#09090b] text-[#a1a1aa] border border-[#27272a] hover:border-[#71717a]'
            }`}
          >
            <div className="w-4 h-4 rounded-full border border-slate-900" style={{ backgroundColor: mat.color }} />
            {mat.name}
          </button>
        ))}
      </div>
    </div>
  );
}
