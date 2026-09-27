import React, { useState } from 'react';
import { useSchematicStore } from '../../state/schematicStore';
import { usePcbStore } from '../../state/pcbStore';
import { 
  Plus, 
  Search, 
  Cpu, 
  Settings, 
  Layers, 
  Activity, 
  FolderGit2, 
  Compass,
  ArrowRight,
  Database
} from 'lucide-react';

interface PartTemplate {
  name: string;
  value: string;
  type: string;
  pins: { name: string; defaultNet: string }[];
  footprintLayer: string;
}

const LIBRARY_TEMPLATES: PartTemplate[] = [
  {
    name: 'U2 (AMS1117-3.3)',
    value: 'LDO Regulator',
    type: 'Regulator',
    pins: [
      { name: 'GND', defaultNet: 'GND' },
      { name: 'VOUT', defaultNet: 'VCC_3V3' },
      { name: 'VIN', defaultNet: 'VCC' }
    ],
    footprintLayer: 'F.Cu'
  },
  {
    name: 'J1 (USB-C)',
    value: 'USB Connector',
    type: 'Connector',
    pins: [
      { name: 'GND', defaultNet: 'GND' },
      { name: 'VBUS', defaultNet: 'VCC' },
      { name: 'D+', defaultNet: 'USB_D_P' },
      { name: 'D-', defaultNet: 'USB_D_N' }
    ],
    footprintLayer: 'F.Cu'
  },
  {
    name: 'LED1',
    value: 'Green Indicator',
    type: 'LED',
    pins: [
      { name: 'A', defaultNet: 'SIG' },
      { name: 'K', defaultNet: 'GND' }
    ],
    footprintLayer: 'F.Cu'
  },
  {
    name: 'R2',
    value: '220R',
    type: 'Resistor',
    pins: [
      { name: '1', defaultNet: 'SIG' },
      { name: '2', defaultNet: 'GND' }
    ],
    footprintLayer: 'F.Cu'
  }
];

export function ComponentLibraryPanel() {
  const [search, setSearch] = useState('');
  const { schematicData, setSchematicData } = useSchematicStore();
  const { pcbData, setPcbData } = usePcbStore();

  const handleInstantiate = (template: PartTemplate) => {
    if (!schematicData || !pcbData) return;

    // 1. Add to schematic
    const reference = template.name.split(' ')[0];
    
    // Check if duplicate
    if (schematicData.components.some(c => c.reference === reference)) {
      alert(`${reference} is already instantiated in your workspace.`);
      return;
    }

    const newComponent = {
      reference,
      value: template.value,
      type: template.type
    };

    // Synthesize connections
    const newNets = [...schematicData.nets];
    template.pins.forEach(pin => {
      let net = newNets.find(n => n.name === pin.defaultNet);
      if (!net) {
        net = { name: pin.defaultNet, connections: [] };
        newNets.push(net);
      }
      net.connections.push({
        source: reference,
        target: 'U1' // Connect to standard MCU for visualization
      });
    });

    setSchematicData({
      components: [...schematicData.components, newComponent],
      nets: newNets
    });

    // 2. Add footprint to PCB
    const index = pcbData.footprints.length;
    const px = 40 + (index * 12) % 60;
    const py = 40 + (index * 12) % 60;

    const newFootprint = {
      reference,
      value: template.value,
      x: px,
      y: py,
      orientation: 0,
      layer: template.footprintLayer,
      pads: template.pins.map((pin, i) => ({
        name: pin.name,
        net: pin.defaultNet,
        x: px - 4 + i * 4,
        y: py - 4
      }))
    };

    // Create custom visual traces for connected nets automatically
    const newTraces = [...pcbData.traces];
    template.pins.forEach((pin, i) => {
      newTraces.push({
        start_x: px - 4 + i * 4,
        start_y: py - 4,
        end_x: 60, // Standard MCU x
        end_y: 60, // Standard MCU y
        net: pin.defaultNet,
        width: pin.defaultNet === 'GND' || pin.defaultNet === 'VCC' ? 0.6 : 0.25,
        layer: 'F.Cu'
      });
    });

    setPcbData({
      ...pcbData,
      footprints: [...pcbData.footprints, newFootprint],
      traces: newTraces
    });
  };

  const filtered = LIBRARY_TEMPLATES.filter(item => 
    item.name.toLowerCase().includes(search.toLowerCase()) || 
    item.value.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <div className="flex flex-col h-full bg-[#141417]/80 backdrop-blur rounded-xl border border-white/5 overflow-hidden shadow-2xl">
      {/* Search Header */}
      <div className="p-3 bg-[#0F0F11] border-b border-white/5 space-y-2">
        <div className="flex items-center gap-1.5 ">
          <Compass className="w-3.5 h-3.5 text-teal-400" />
          <span className="text-[10px] uppercase tracking-widest text-slate-200 font-bold">Flux Libs & Nets</span>
        </div>
        <div className="relative flex items-center bg-white/5 border border-white/10 rounded-lg px-2.5 py-1.5">
          <Search className="w-3.5 h-3.5 text-slate-500 mr-2 shrink-0" />
          <input
            type="text"
            placeholder="Search parts, templates..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="w-full bg-transparent border-0 outline-none text-slate-200 text-xs placeholder-slate-500"
          />
        </div>
      </div>

      {/* Parts List */}
      <div className="flex-1 overflow-y-auto p-3 space-y-2.5 min-h-0 text-xs text-slate-300">
        <div className="text-[9px] uppercase tracking-widest text-slate-500 font-extrabold mb-1">Instantiation Templates</div>
        
        {filtered.map((item, idx) => (
          <div 
            key={idx} 
            className="p-3 bg-white/5 border border-white/5 rounded-lg hover:border-teal-500/30 transition-all flex flex-col gap-2 group hover:bg-white/10"
          >
            <div className="flex items-start justify-between">
              <div>
                <div className="font-bold font-mono text-[11px] text-slate-200 group-hover:text-white transition-colors">{item.name}</div>
                <div className="text-[10px] text-slate-400 mt-0.5">{item.value}</div>
              </div>
              <button 
                onClick={() => handleInstantiate(item)}
                className="w-5 h-5 rounded bg-teal-600/10 hover:bg-teal-600 text-teal-400 hover:text-white flex items-center justify-center transition-all cursor-pointer"
                title="Add to Workspace"
              >
                <Plus className="w-3.5 h-3.5" />
              </button>
            </div>

            {/* Pins overview */}
            <div className="flex flex-wrap gap-1">
              {item.pins.map((pin, pIdx) => (
                <span key={pIdx} className="bg-white/5 border border-white/5 px-1.5 py-0.5 rounded text-[8px] font-mono text-slate-400">
                  {pin.name} → <span className="text-teal-400">{pin.defaultNet}</span>
                </span>
              ))}
            </div>
          </div>
        ))}

        {filtered.length === 0 && (
          <div className="text-center text-slate-500 py-6">No matching templates found.</div>
        )}

        {/* Workspace hierarchy preview */}
        <div className="mt-4 pt-4 border-t border-white/5 space-y-2">
          <div className="text-[9px] uppercase tracking-widest text-slate-500 font-extrabold flex items-center gap-1">
            <Database className="w-3 h-3 text-teal-500" />
            Live Circuit Tree
          </div>
          <div className="space-y-1 text-[10px] font-mono text-slate-400">
            <div className="flex justify-between p-1 hover:bg-white/5 rounded">
              <span>🔲 U1 (Main MCU)</span>
              <span className="text-teal-500">Master</span>
            </div>
            {schematicData?.components.filter(c => c.reference !== 'U1').map((c, i) => (
              <div key={i} className="flex justify-between p-1 hover:bg-white/5 rounded">
                <span>🔲 {c.reference} ({c.value})</span>
                <span className="text-slate-500">{c.type}</span>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
