import React from 'react';

interface DimensionInputProps {
  label: string;
  value: number;
  min?: number;
  max?: number;
  onChange: (value: number) => void;
}

export function DimensionInput({ label, value, min, max, onChange }: DimensionInputProps) {
  return (
    <div className="flex flex-col gap-1">
      <label className="text-[10px] font-bold uppercase tracking-widest text-[#71717a]">{label}</label>
      <input
        type="number"
        value={value}
        min={min}
        max={max}
        onChange={(e) => onChange(parseFloat(e.target.value) || 0)}
        className="px-2 py-1.5 bg-[#09090b] border border-[#27272a] rounded-lg text-white focus:outline-none focus:border-[#71717a] font-mono text-xs transition-colors"
      />
    </div>
  );
}
