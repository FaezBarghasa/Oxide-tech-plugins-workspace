import React from 'react';
import { VentConfig } from '../../types/blender';
import { DimensionInput } from './DimensionInput';

interface VentConfigPanelProps {
  config: VentConfig;
  onChange: (config: VentConfig) => void;
}

export function VentConfigPanel({ config, onChange }: VentConfigPanelProps) {
  return (
    <div className="flex flex-col gap-3 p-3 bg-[#09090b] rounded-xl border border-[#27272a]">
      <h3 className="text-[10px] font-bold uppercase tracking-widest text-[#71717a]">Ventilation Array</h3>
      <div className="grid grid-cols-3 gap-2">
        <DimensionInput
          label="Diameter"
          value={config.holeDiameter}
          onChange={(val) => onChange({ ...config, holeDiameter: val })}
        />
        <DimensionInput
          label="Spacing"
          value={config.spacing}
          onChange={(val) => onChange({ ...config, spacing: val })}
        />
        <DimensionInput
          label="Quantity"
          value={config.quantity}
          onChange={(val) => onChange({ ...config, quantity: val })}
        />
      </div>
    </div>
  );
}
