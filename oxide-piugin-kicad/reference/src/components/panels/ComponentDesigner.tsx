import React, { useState, useEffect, useRef } from 'react';
import { 
  Rotate3d, 
  Cpu, 
  Sliders, 
  Zap, 
  Sparkles, 
  FileText, 
  CheckCircle2, 
  PlusCircle, 
  Download, 
  RefreshCw,
  Layers,
  ArrowRight,
  Eye,
  Settings
} from 'lucide-react';
import { useSchematicStore } from '../../state/schematicStore';
import { usePcbStore } from '../../state/pcbStore';

interface Pin {
  num: string;
  name: string;
  type: string;
  side?: 'left' | 'right' | 'top' | 'bottom'; // for symbol styling
}

interface ComponentModel {
  name: string;
  value: string;
  type: string;
  referencePrefix: string;
  package: string;
  pins: Pin[];
  dimensions: {
    width: number;
    height: number;
    pitch: number;
    bodyWidth: number;
    bodyLength: number;
    bodyHeight: number;
    color: string;
  };
}

const PRESETS: Record<string, ComponentModel> = {
  "NE555": {
    name: "NE555",
    value: "Precision Timer",
    type: "Timer",
    referencePrefix: "U",
    package: "DIP-8",
    pins: [
      { num: "1", name: "GND", type: "gnd", side: 'left' },
      { num: "2", name: "TRIG", type: "input", side: 'left' },
      { num: "3", name: "OUT", type: "output", side: 'right' },
      { num: "4", name: "RESET", type: "input", side: 'left' },
      { num: "5", name: "CONT", type: "passive", side: 'right' },
      { num: "6", name: "THRES", type: "input", side: 'left' },
      { num: "7", name: "DISCH", type: "passive", side: 'right' },
      { num: "8", name: "VCC", type: "power", side: 'right' }
    ],
    dimensions: {
      width: 10.16,
      height: 7.62,
      pitch: 2.54,
      bodyWidth: 6.35,
      bodyLength: 9.27,
      bodyHeight: 3.3,
      color: "#18181A"
    }
  },
  "CP2102": {
    name: "CP2102",
    value: "USB-to-UART Bridge",
    type: "Bridge",
    referencePrefix: "U",
    package: "QFN-28",
    pins: [
      { num: "1", name: "DCD", type: "input", side: 'left' },
      { num: "2", name: "RI", type: "input", side: 'left' },
      { num: "3", name: "GND", type: "gnd", side: 'left' },
      { num: "4", name: "D+", type: "bidirectional", side: 'left' },
      { num: "5", name: "D-", type: "bidirectional", side: 'left' },
      { num: "6", name: "VDD", type: "power", side: 'left' },
      { num: "7", name: "REGIN", type: "power", side: 'left' },
      { num: "8", name: "VBUS", type: "input", side: 'left' },
      { num: "9", name: "RST", type: "input", side: 'right' },
      { num: "12", name: "RXD", type: "input", side: 'right' },
      { num: "13", name: "TXD", type: "output", side: 'right' },
      { num: "25", name: "SUSPEND", type: "output", side: 'right' },
      { num: "26", name: "SUSPEND/", type: "output", side: 'right' }
    ],
    dimensions: {
      width: 5.0,
      height: 5.0,
      pitch: 0.5,
      bodyWidth: 5.0,
      bodyLength: 5.0,
      bodyHeight: 0.9,
      color: "#24252C"
    }
  }
};

export function ComponentDesigner() {
  const { schematicData, setSchematicData } = useSchematicStore();
  const { pcbData, setPcbData } = usePcbStore();

  const [datasheetInput, setDatasheetInput] = useState<string>(
    `ESP32-S3 Pinout & Dimensions:\n- Pin 1: GND (GND)\n- Pin 2: 3V3 (Power)\n- Pin 3: EN (Input)\n- Pin 4: GPIO4 (I/O)\n- Pin 5: GPIO5 (I/O)\n- Pin 6: TXD0 (UART TX)\n- Pin 7: RXD0 (UART RX)\n- Pin 8: GPIO15 (I/O)\n- Outer package: SQA-24 SMD, Pitch 1.27mm\n- Body Width: 7.0mm, Body Length: 7.0mm, Height: 1.0mm`
  );

  const [activeModel, setActiveModel] = useState<ComponentModel>(PRESETS.NE555);
  const [activeTab, setActiveTab] = useState<'datasheet' | 'schematic' | 'footprint' | '3d'>('3d');
  const [isParsing, setIsParsing] = useState(false);
  const [parseStatus, setParseStatus] = useState('');
  
  // Dynamic slide values
  const [bodyWidth, setBodyWidth] = useState(activeModel.dimensions.bodyWidth);
  const [bodyLength, setBodyLength] = useState(activeModel.dimensions.bodyLength);
  const [bodyHeight, setBodyHeight] = useState(activeModel.dimensions.bodyHeight);
  const [pitch, setPitch] = useState(activeModel.dimensions.pitch);

  // Rotation angles for interactive Z-rotatable pseudo-3D Canvas
  const [rotX, setRotX] = useState(-20);
  const [rotY, setRotY] = useState(35);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const isDragging = useRef(false);
  const prevMouse = useRef({ x: 0, y: 0 });

  // Sync sliders to active model shifts
  useEffect(() => {
    setBodyWidth(activeModel.dimensions.bodyWidth);
    setBodyLength(activeModel.dimensions.bodyLength);
    setBodyHeight(activeModel.dimensions.bodyHeight);
    setPitch(activeModel.dimensions.pitch);
  }, [activeModel]);

  const loadPreset = (name: string) => {
    if (PRESETS[name]) {
      setActiveModel(PRESETS[name]);
    }
  };

  const handleParseDatasheet = async () => {
    if (!datasheetInput.trim()) return;
    setIsParsing(true);
    setParseStatus('AI Reading electrical specs...');
    try {
      const res = await fetch('/api/datasheet-parser', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ datasheetText: datasheetInput })
      });
      if (!res.ok) throw new Error('Failed to reach datasheet modeling agent.');
      const parsed: ComponentModel = await res.json();
      
      // Inject side properties if missing
      parsed.pins = parsed.pins.map((pin, i) => ({
        ...pin,
        side: pin.side || (i % 2 === 0 ? 'left' : 'right')
      }));

      setActiveModel(parsed);
      setParseStatus('Successfully generated fully customizable CAD footprint, schematic symbol, and interactive 3D model!');
      setActiveTab('3d');
    } catch (e: any) {
      setParseStatus(`Error: ${e.message || 'Verification failed. Using standard compilation fallback.'}`);
    } finally {
      setIsParsing(false);
    }
  };

  // Push component into actual schematic and board layout
  const handleDeployToWorkspace = () => {
    if (!schematicData || !pcbData) return;

    // Build unique ref ID
    const baseRef = activeModel.referencePrefix || 'U';
    let index = 1;
    while (schematicData.components.some(c => c.reference === `${baseRef}${index}`)) {
      index++;
    }
    const finalRef = `${baseRef}${index}`;

    // 1. Add to schematic store
    const newComponent = {
      reference: finalRef,
      value: activeModel.value || activeModel.name,
      type: activeModel.type || 'IC'
    };

    // Link nets
    const newNets = [...schematicData.nets];
    activeModel.pins.forEach((pin) => {
      // Find a net to latch onto or build a custom pinout net
      const targetNetName = pin.type === 'gnd' ? 'GND' : pin.type === 'power' ? 'VCC' : `NET_${pin.name}`;
      let net = newNets.find(n => n.name === targetNetName);
      if (!net) {
        net = { name: targetNetName, connections: [] };
        newNets.push(net);
      }
      net.connections.push({ source: finalRef, target: 'U1' });
    });

    setSchematicData({
      components: [...schematicData.components, newComponent],
      nets: newNets
    });

    // 2. Add footprint to PCB
    const count = pcbData.footprints.length;
    const px = 50 + (count * 12) % 50;
    const py = 50 + (count * 12) % 50;

    const newFootprint = {
      reference: finalRef,
      value: activeModel.value || activeModel.name,
      x: px,
      y: py,
      orientation: 0,
      layer: 'F.Cu',
      pads: activeModel.pins.map((pin, i) => {
        // Arrange pads beautifully in two rows
        const isLeft = i < activeModel.pins.length / 2;
        const offsetMultiplier = i % Math.ceil(activeModel.pins.length / 2);
        const padX = px + (isLeft ? -bodyWidth / 2 - 1 : bodyWidth / 2 + 1);
        const padY = py - (activeModel.pins.length / 4 * pitch) + (offsetMultiplier * pitch);

        return {
          name: pin.num,
          net: pin.type === 'gnd' ? 'GND' : pin.type === 'power' ? 'VCC' : `NET_${pin.name}`,
          x: padX,
          y: padY
        };
      })
    };

    // Add trace wires
    const newTraces = [...pcbData.traces];
    newFootprint.pads.forEach(pad => {
      newTraces.push({
        start_x: pad.x,
        start_y: pad.y,
        end_x: 60,
        end_y: 60,
        net: pad.net,
        width: pad.net === 'GND' || pad.net === 'VCC' ? 0.6 : 0.25,
        layer: 'F.Cu'
      });
    });

    setPcbData({
      ...pcbData,
      footprints: [...pcbData.footprints, newFootprint],
      traces: newTraces
    });

    setParseStatus(`Deployed ${finalRef} successfully to schematic and layout!`);
  };

  // INTERACTIVE 3D PACKAGE CANVAS DRAWER
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Reset dimensions
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    const centerX = canvas.width / 2;
    const centerY = canvas.height / 2;

    // Rotation projection projection function helper
    const project = (x: number, y: number, z: number) => {
      // Rotation on Y axis
      const radY = (rotY * Math.PI) / 180;
      const cosY = Math.cos(radY);
      const sinY = Math.sin(radY);

      let x1 = x * cosY - z * sinY;
      let z1 = x * sinY + z * cosY;

      // Rotation on X axis
      const radX = (rotX * Math.PI) / 180;
      const cosX = Math.cos(radX);
      const sinX = Math.sin(radX);

      let y2 = y * cosX - z1 * sinX;
      let z2 = y * sinX + z1 * cosX;

      // Isometric projection scaling
      const factor = 220 / (220 + z2);
      return {
        x: centerX + x1 * factor * 13,
        y: centerY + y2 * factor * 13,
        zDepth: z2
      };
    };

    // Package dimensions scale (standardization)
    const dw = bodyWidth;
    const dl = bodyLength;
    const dh = bodyHeight;

    // Build the 8 body vertices
    const vertices = [
      { x: -dw/2, y: -dh/2, z: -dl/2 }, // 0
      { x: dw/2,  y: -dh/2, z: -dl/2 }, // 1
      { x: dw/2,  y: dh/2,  z: -dl/2 }, // 2
      { x: -dw/2, y: dh/2,  z: -dl/2 }, // 3
      { x: -dw/2, y: -dh/2, z: dl/2  }, // 4
      { x: dw/2,  y: -dh/2, z: dl/2  }, // 5
      { x: dw/2,  y: dh/2,  z: dl/2  }, // 6
      { x: -dw/2, y: dh/2,  z: dl/2  }  // 7
    ];

    // Project vertices
    const projVertices = vertices.map(v => project(v.x, v.y, v.z));

    // Polygon drawing helper
    const drawFace = (indices: number[], color: string, strokeColor = 'rgba(255,255,255,0.15)') => {
      ctx.beginPath();
      ctx.moveTo(projVertices[indices[0]].x, projVertices[indices[0]].y);
      for (let i = 1; i < indices.length; i++) {
        ctx.lineTo(projVertices[indices[i]].x, projVertices[indices[i]].y);
      }
      ctx.closePath();
      ctx.fillStyle = color;
      ctx.fill();
      ctx.strokeStyle = strokeColor;
      ctx.stroke();
    };

    // Draw leads (metallic lines on both sides)
    const pinCount = activeModel.pins.length;
    const halfCount = Math.ceil(pinCount / 2);

    // Draw leads on left and right sides
    for (let i = 0; i < pinCount; i++) {
      const isLeft = i < halfCount;
      const indexOffset = i % halfCount;
      const zPos = -dl/2 + (indexOffset * pitch);

      if (Math.abs(zPos) <= dl/2 + 0.1) {
        // Form lead vertices
        const leadXStart = isLeft ? -dw/2 : dw/2;
        const leadXEnd = isLeft ? -dw/2 - 1.2 : dw/2 + 1.2;
        const leadYBottom = dh/2 + 0.8;

        const p1 = project(leadXStart, dh/2 - 0.2, zPos);
        const p2 = project(leadXEnd, dh/2, zPos);
        const p3 = project(leadXEnd, leadYBottom, zPos);

        ctx.beginPath();
        ctx.moveTo(p1.x, p1.y);
        ctx.lineTo(p2.x, p2.y);
        ctx.lineTo(p3.x, p3.y);
        ctx.strokeStyle = '#94A3B8';
        ctx.lineWidth = 2.5;
        ctx.stroke();

        // Pin 1 Indicator Circle
        if (i === 0) {
          ctx.beginPath();
          ctx.arc(p1.x, p1.y - 4, 3, 0, Math.PI * 2);
          ctx.fillStyle = '#EF4444';
          ctx.fill();
        }
      }
    }

    // Sort body faces, draw depending on rotation to maintain relative depth
    const faces = [
      { indices: [0, 1, 2, 3], name: 'back', avgZ: (projVertices[0].zDepth + projVertices[1].zDepth + projVertices[2].zDepth + projVertices[3].zDepth) / 4 }, // back
      { indices: [4, 5, 6, 7], name: 'front', avgZ: (projVertices[4].zDepth + projVertices[5].zDepth + projVertices[6].zDepth + projVertices[7].zDepth) / 4 }, // front
      { indices: [0, 4, 7, 3], name: 'left', avgZ: (projVertices[0].zDepth + projVertices[4].zDepth + projVertices[7].zDepth + projVertices[3].zDepth) / 4 }, // left
      { indices: [1, 5, 6, 2], name: 'right', avgZ: (projVertices[1].zDepth + projVertices[5].zDepth + projVertices[6].zDepth + projVertices[2].zDepth) / 4 }, // right
      { indices: [0, 1, 5, 4], name: 'top', avgZ: (projVertices[0].zDepth + projVertices[1].zDepth + projVertices[5].zDepth + projVertices[4].zDepth) / 4 }, // top
      { indices: [3, 2, 6, 7], name: 'bottom', avgZ: (projVertices[3].zDepth + projVertices[2].zDepth + projVertices[6].zDepth + projVertices[7].zDepth) / 4 } // bottom
    ];

    // Sort painters algorithm
    faces.sort((a, b) => b.avgZ - a.avgZ);

    faces.forEach(face => {
      let highlightColor = activeModel.dimensions.color || '#1e1e24';
      if (face.name === 'top') highlightColor = '#2D2E37';
      if (face.name === 'front' || face.name === 'right') highlightColor = '#1A1A1F';
      drawFace(face.indices, highlightColor);
    });

    // Draw Pin Names floating on the body
    ctx.fillStyle = 'rgba(255, 255, 255, 0.4)';
    ctx.font = '7px monospace';
    // Part designation writing on body
    const topProj = project(0, -dh/2, 0);
    ctx.fillStyle = '#14B8A6';
    ctx.fillText(activeModel.name, topProj.x - 12, topProj.y);

  }, [rotX, rotY, bodyWidth, bodyLength, bodyHeight, pitch, activeModel]);

  // DRAG ROTATE LOGIC
  const handleMouseDown = (e: React.MouseEvent) => {
    isDragging.current = true;
    prevMouse.current = { x: e.clientX, y: e.clientY };
  };

  const handleMouseMove = (e: React.MouseEvent) => {
    if (!isDragging.current) return;
    const deltaX = e.clientX - prevMouse.current.x;
    const deltaY = e.clientY - prevMouse.current.y;

    setRotY(prev => prev + deltaX * 1.0);
    setRotX(prev => Math.max(-85, Math.min(85, prev - deltaY * 1.0)));

    prevMouse.current = { x: e.clientX, y: e.clientY };
  };

  const handleMouseUp = () => {
    isDragging.current = false;
  };

  return (
    <div className="flex flex-col h-full bg-[#111113] border border-white/5 rounded-xl overflow-hidden shadow-2xl leading-none">
      {/* Header Tabs */}
      <div className="bg-[#17171B] border-b border-white/5 p-2.5 flex flex-col gap-2">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-1.5">
            <Sparkles className="w-3.5 h-3.5 text-teal-400 animate-pulse" />
            <h2 className="text-[10px] font-bold uppercase tracking-widest text-slate-100">Component Desk</h2>
          </div>
          <div className="flex gap-1">
            <button 
              onClick={() => loadPreset('NE555')} 
              className={`px-1.5 py-0.5 rounded text-[8px] font-bold border transition-all ${activeModel.name === 'NE555' ? 'bg-teal-600/10 border-teal-500/30 text-teal-400' : 'border-white/5 text-slate-400'}`}
            >
              NE555
            </button>
            <button 
              onClick={() => loadPreset('CP2102')} 
              className={`px-1.5 py-0.5 rounded text-[8px] font-bold border transition-all ${activeModel.name === 'CP2102' ? 'bg-teal-600/10 border-teal-500/30 text-teal-400' : 'border-white/5 text-slate-400'}`}
            >
              CP2102
            </button>
          </div>
        </div>

        <div className="flex bg-[#0A0A0C] p-0.5 rounded-lg border border-white/5">
          <button 
            onClick={() => setActiveTab('datasheet')}
            className={`flex-1 py-1 rounded text-[8.5px] uppercase font-bold tracking-wider text-center transition-all cursor-pointer ${activeTab === 'datasheet' ? 'bg-teal-600 text-white shadow' : 'text-slate-400 hover:text-white'}`}
          >
            Datasheet
          </button>
          <button 
            onClick={() => setActiveTab('schematic')}
            className={`flex-1 py-1 rounded text-[8.5px] uppercase font-bold tracking-wider text-center transition-all cursor-pointer ${activeTab === 'schematic' ? 'bg-teal-600 text-white shadow' : 'text-slate-400 hover:text-white'}`}
          >
            Schematic
          </button>
          <button 
            onClick={() => setActiveTab('footprint')}
            className={`flex-1 py-1 rounded text-[8.5px] uppercase font-bold tracking-wider text-center transition-all cursor-pointer ${activeTab === 'footprint' ? 'bg-teal-600 text-white shadow' : 'text-slate-400 hover:text-white'}`}
          >
            Footprint
          </button>
          <button 
            onClick={() => setActiveTab('3d')}
            className={`flex-1 py-1 rounded text-[8.5px] uppercase font-bold tracking-wider text-center transition-all cursor-pointer ${activeTab === '3d' ? 'bg-teal-600 text-white shadow' : 'text-slate-400 hover:text-white'}`}
          >
            3D Model
          </button>
        </div>
      </div>

      {/* Main Panel Content */}
      <div className="flex-1 overflow-y-auto p-3 space-y-3 min-h-0 text-[11px] leading-tight">
        
        {activeTab === 'datasheet' && (
          <div className="space-y-3">
            <div className="bg-[#17171B] border border-white/5 p-2.5 rounded-lg">
              <div className="text-[9px] uppercase tracking-widest text-slate-500 font-semibold mb-1.5 flex items-center gap-1">
                <FileText className="w-3 h-3 text-teal-400" />
                Silicon Specs & Parameters
              </div>
              <textarea
                value={datasheetInput}
                onChange={(e) => setDatasheetInput(e.target.value)}
                className="w-full h-28 bg-[#0A0A0C] text-slate-300 font-mono text-[9px] p-2 rounded border border-white/5 focus:border-teal-500/40 outline-none resize-none leading-normal"
                placeholder="Paste datasheet parameters, mechanical dimensions, pin assignment maps..."
              />
            </div>

            <button 
              onClick={handleParseDatasheet}
              disabled={isParsing}
              className="w-full py-2 bg-teal-600 hover:bg-teal-500 text-white font-bold uppercase tracking-widest rounded-lg flex items-center justify-center gap-1.5 transition-all shadow-lg shadow-teal-500/10 cursor-pointer disabled:bg-teal-850 text-[9px]"
            >
              {isParsing ? (
                <>
                  <RefreshCw className="w-3.5 h-3.5 animate-spin text-white" />
                  PARSING SPECIFICATIONS...
                </>
              ) : (
                <>
                  <Sparkles className="w-3.5 h-3.5 fill-current text-white" />
                  COMPILE WITH GEMINI
                </>
              )}
            </button>
          </div>
        )}

        {activeTab === 'schematic' && (
          <div className="space-y-3">
            <div className="text-[9px] uppercase tracking-widest text-slate-500 font-semibold">Symbol Preview & Functional Types</div>
            <div className="bg-[#0D0D10] border border-white/5 rounded-lg p-3 flex flex-col items-center justify-center relative min-h-[130px]">
              
              {/* Schematic Box */}
              <div className="w-36 border border-teal-500/50 bg-[#141419]/90 rounded p-2.5 relative flex flex-col justify-between shadow-2xl min-h-[100px]">
                {/* Symbol reference indicator */}
                <div className="absolute -top-2.5 left-3 bg-[#111113] border border-teal-500/30 text-[8px] font-mono px-1 py-0.2 text-teal-400 rounded-sm">
                  {activeModel.referencePrefix}?
                </div>

                <div className="text-center font-bold font-mono text-slate-100 text-[10px] mb-1.5">{activeModel.name}</div>
                
                {/* Left Pins */}
                <div className="absolute -left-4 top-3 space-y-1 flex flex-col text-[7px] font-mono">
                  {activeModel.pins.filter(p => !p.side || p.side === 'left').map((pin) => (
                    <div key={pin.num} className="flex items-center gap-0.5">
                      <span className="text-[6px] text-slate-500">{pin.num}</span>
                      <span className="text-slate-300 font-bold bg-white/5 px-0.5 rounded-[2px]">{pin.name}</span>
                      <span className="w-1 h-[1px] bg-slate-500" />
                    </div>
                  ))}
                </div>

                {/* Right Pins */}
                <div className="absolute -right-4 top-3 space-y-1 flex flex-col items-end text-[7px] font-mono">
                  {activeModel.pins.filter(p => p.side === 'right').map((pin) => (
                    <div key={pin.num} className="flex items-center gap-0.5">
                      <span className="w-1 h-[1px] bg-slate-500" />
                      <span className="text-slate-300 font-bold bg-white/5 px-0.5 rounded-[2px]">{pin.name}</span>
                      <span className="text-[6px] text-slate-500">{pin.num}</span>
                    </div>
                  ))}
                </div>

                <div className="text-center text-[8px] text-slate-500 mt-auto font-mono">{activeModel.package}</div>
              </div>

            </div>

            {/* Pins attribute editor */}
            <div className="bg-[#17171B] border border-white/5 p-2 rounded-lg space-y-1.5 max-h-40 overflow-y-auto">
              <div className="text-[9px] uppercase tracking-widest text-slate-500 font-semibold mb-1">Pin Assignment Grid</div>
              {activeModel.pins.map((pin, index) => (
                <div key={index} className="flex items-center justify-between p-1 bg-white/5 rounded border border-white/5">
                  <div className="flex items-center gap-1.5 font-mono text-[10px]">
                    <span className="text-teal-400 font-bold">#{pin.num}</span>
                    <span className="text-slate-200">{pin.name}</span>
                  </div>
                  <span className={`text-[7.5px] uppercase tracking-wider px-1 py-0.5 rounded font-mono ${
                    pin.type === 'power' ? 'bg-red-500/10 text-red-400' :
                    pin.type === 'gnd' ? 'bg-slate-500/15 text-slate-400' :
                    pin.type === 'output' ? 'bg-teal-500/10 text-teal-400' :
                    'bg-slate-500/10 text-slate-300'
                  }`}>
                    {pin.type}
                  </span>
                </div>
              ))}
            </div>
          </div>
        )}

        {activeTab === 'footprint' && (
          <div className="space-y-3">
            <div className="text-[9px] uppercase tracking-widest text-slate-500 font-semibold">Copper Land Pattern (SMD Grid)</div>
            
            <div className="bg-[#0A0A0C] border border-white/5 rounded-lg p-3 flex flex-col items-center justify-center min-h-[140px] relative">
              <div className="absolute top-1.5 left-1.5 text-[7px] font-mono text-slate-500">grid: {pitch}mm</div>
              
              {/* Copper Pads Layout */}
              <div className="border border-red-500/10 bg-red-500/5 rounded p-5 flex justify-between relative" style={{ width: `${bodyWidth * 10}px`, height: `${bodyLength * 10}px` }}>
                <span className="absolute top-1.5 left-1.5 text-[7px] font-mono text-amber-500/40">F.Silk Outline</span>
                
                {/* Left Column Pads */}
                <div className="absolute -left-2 top-2 bottom-2 flex flex-col justify-between">
                  {Array.from({ length: Math.ceil(activeModel.pins.length/2) }).map((_, i) => (
                    <div key={i} className="w-3 h-1.5 bg-amber-600/90 hover:bg-amber-500 border border-amber-500/50 rounded-sm text-[6px] text-white flex items-center justify-center font-mono font-bold shadow-sm cursor-pointer" title={`Pad ${i+1}`}>
                      {i + 1}
                    </div>
                  ))}
                </div>

                {/* Right Column Pads */}
                <div className="absolute -right-2 top-2 bottom-2 flex flex-col justify-between">
                  {Array.from({ length: Math.floor(activeModel.pins.length/2) }).map((_, i) => (
                    <div key={i} className="w-3 h-1.5 bg-amber-600/90 hover:bg-amber-500 border border-amber-500/50 rounded-sm text-[6px] text-white flex items-center justify-center font-mono font-bold shadow-sm cursor-pointer" title={`Pad ${activeModel.pins.length - i}`}>
                      {activeModel.pins.length - i}
                    </div>
                  ))}
                </div>

                {/* center notch */}
                <div className="absolute top-0 left-1/2 -translate-x-1/2 w-3 h-1 bg-[#0A0A0C] border-b border-white/10 rounded-b-sm" />
              </div>
            </div>

            {/* Micro sliders to fine-tune package specs */}
            <div className="bg-[#17171B] border border-white/5 p-2 rounded-lg space-y-2">
              <div className="text-[9px] uppercase tracking-widest text-slate-500 font-semibold mb-1 flex items-center gap-1">
                <Sliders className="w-3 h-3 text-teal-400" />
                MECHANICAL ENVELOPE (MM)
              </div>
              
              <div className="space-y-1.5 text-[10px]">
                <div className="flex justify-between items-center bg-white/5 p-1 rounded">
                  <span className="text-slate-400">Body Width (X)</span>
                  <input 
                    type="range" min="2" max="15" step="0.1" 
                    value={bodyWidth} 
                    onChange={(e) => setBodyWidth(parseFloat(e.target.value))}
                    className="w-20 accent-teal-500 h-1" 
                  />
                  <span className="font-mono text-teal-400 font-bold">{bodyWidth}</span>
                </div>

                <div className="flex justify-between items-center bg-white/5 p-1 rounded">
                  <span className="text-slate-400">Body Length (Y)</span>
                  <input 
                    type="range" min="2" max="15" step="0.1" 
                    value={bodyLength} 
                    onChange={(e) => setBodyLength(parseFloat(e.target.value))}
                    className="w-20 accent-teal-500 h-1" 
                  />
                  <span className="font-mono text-teal-400 font-bold">{bodyLength}</span>
                </div>

                <div className="flex justify-between items-center bg-white/5 p-1 rounded">
                  <span className="text-slate-400">Pin Spacing (Pitch)</span>
                  <input 
                    type="range" min="0.4" max="3" step="0.05" 
                    value={pitch} 
                    onChange={(e) => setPitch(parseFloat(e.target.value))}
                    className="w-20 accent-teal-500 h-1" 
                  />
                  <span className="font-mono text-teal-400 font-bold">{pitch}</span>
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === '3d' && (
          <div className="space-y-3">
            <div className="flex items-center justify-between">
              <div className="text-[9px] uppercase tracking-widest text-[#94A3B8] font-bold flex items-center gap-1">
                <Rotate3d className="w-3.5 h-3.5 text-teal-400 animate-spin" />
                DOCKABLE Rotatable 3D RENDER
              </div>
              <span className="text-[8px] font-mono text-slate-500 hover:text-white transition-all cursor-pointer">
                Drag to orbit package
              </span>
            </div>

            {/* Rotatable Box Canvas container */}
            <div className="relative bg-[#0A0A0C] border border-white/5 rounded-lg flex items-center justify-center p-1 group shadow-inner">
              <canvas
                ref={canvasRef}
                width={200}
                height={150}
                onMouseDown={handleMouseDown}
                onMouseMove={handleMouseMove}
                onMouseUp={handleMouseUp}
                onMouseLeave={handleMouseUp}
                className="cursor-grab active:cursor-grabbing hover:drop-shadow-[0_0_15px_rgba(20,184,166,0.15)] transition-all"
              />
            </div>

            {/* package geometry stats */}
            <div className="grid grid-cols-3 gap-1 text-center text-[9px] font-mono">
              <div className="bg-[#17171B] border border-white/5 p-1.5 rounded">
                <div className="text-[7.5px] text-slate-500 uppercase">Body X</div>
                <div className="text-teal-400 font-bold mt-0.5">{bodyWidth} mm</div>
              </div>
              <div className="bg-[#17171B] border border-white/5 p-1.5 rounded">
                <div className="text-[7.5px] text-slate-500 uppercase">Body Y</div>
                <div className="text-teal-400 font-bold mt-0.5">{bodyLength} mm</div>
              </div>
              <div className="bg-[#17171B] border border-white/5 p-1.5 rounded">
                <div className="text-[7.5px] text-slate-500 uppercase">Body Z</div>
                <div className="text-teal-400 font-bold mt-0.5">{bodyHeight} mm</div>
              </div>
            </div>
          </div>
        )}

        {/* Display Status Alerts If Any */}
        {parseStatus && (
          <div className={`p-3 rounded-lg border text-[11px] leading-relaxed transition-all ${
            parseStatus.includes('Error') 
              ? 'bg-red-500/10 border-red-500/20 text-red-400' 
              : 'bg-teal-500/10 border-teal-500/20 text-teal-400'
          }`}>
            {parseStatus.includes('Error') ? '⚠️ ' : '✅ '}
            {parseStatus}
          </div>
        )}

      </div>

      {/* Footer controls deploy */}
      <div className="p-3 bg-[#111113] border-t border-white/5 mt-auto flex items-center justify-between">
        <button 
          onClick={() => {
            const dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(activeModel, null, 2));
            const downloadAnchor = document.createElement('a');
            downloadAnchor.setAttribute("href", dataStr);
            downloadAnchor.setAttribute("download", `${activeModel.name}.json`);
            document.body.appendChild(downloadAnchor);
            downloadAnchor.click();
            downloadAnchor.remove();
          }}
          className="p-2 bg-white/5 border border-white/10 rounded-lg hover:bg-white/10 text-slate-300 hover:text-white transition-all cursor-pointer"
          title="Export CAD model files"
        >
          <Download className="w-4 h-4" />
        </button>
        <button 
          onClick={handleDeployToWorkspace}
          className="flex-1 ml-2 py-2 bg-teal-600 hover:bg-teal-500 text-white font-bold uppercase tracking-widest text-[10px] rounded-lg transition-all shadow-lg shadow-teal-900/10 flex items-center justify-center gap-1.5 cursor-pointer"
        >
          <PlusCircle className="w-3.5 h-3.5" />
          RELEASE TO ACTIVE DESIGN
        </button>
      </div>

    </div>
  );
}
