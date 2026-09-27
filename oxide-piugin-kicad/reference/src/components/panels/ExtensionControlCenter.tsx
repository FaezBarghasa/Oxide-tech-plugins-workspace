import React, { useState } from 'react';
import { usePcbStore } from '../../state/pcbStore';
import { useSchematicStore } from '../../state/schematicStore';
import { useSelectionStore } from '../../state/selectionStore';
import { CadExchanger } from './CadExchanger';
import { 
  Cpu, 
  Terminal, 
  Layers, 
  CheckCircle2, 
  AlertTriangle, 
  Play, 
  Code,
  ArrowRight,
  Database,
  Unplug,
  Settings,
  Flame,
  FileCheck2,
  RefreshCw
} from 'lucide-react';

interface Violation {
  id: string;
  type: string;
  severity: 'error' | 'warning';
  message: string;
  refElement?: string;
}

export function ExtensionControlCenter() {
  const { schematicData, setSchematicData } = useSchematicStore();
  const { pcbData, setPcbData } = usePcbStore();
  const { selectComponent, clearSelection } = useSelectionStore();

  const [activeTab, setActiveTab] = useState<'bridge' | 'skidl' | 'drc' | 'io'>('io');
  const [selectedEdaTool, setSelectedEdaTool] = useState<'kicad' | 'altium'>('kicad');
  const [bridgeConnected, setBridgeConnected] = useState(true);
  const [bridgeLogs, setBridgeLogs] = useState<string[]>([
    "INFO: Initialized unified EDA communication layer",
    "INFO: Detecting system Python environment... found Python 3.11.4",
    "INFO: KiCad pcbnew dynamic library found inside library path",
    "SUCCESS: Bridge listening on port 3000 (Socket.IO enabled)"
  ]);

  const [skidlScript, setSkidlScript] = useState<string>(
`from skidl import *

# Create templates
mcu = Part('MCU_Microchip_ATmega', 'ATmega328P-P')
caps = Part('Device', 'C_Small', value='100nF')
res = Part('Device', 'R_Small', value='10k')

# Bind nets programmatically
gnd_net = Net('GND')
vcc_net = Net('VCC')
sig_net = Net('SIG')

gnd_net += mcu['GND'], caps[1], res[2]
vcc_net += mcu['VCC'], caps[2]
sig_net += mcu['PC0'], res[1]

generate_schematic()`
  );

  const [isCompilingSkidl, setIsCompilingSkidl] = useState(false);
  const [drcReport, setDrcReport] = useState<{
    ran: boolean;
    errors: Violation[];
  }>({
    ran: true,
    errors: [
      { id: '1', type: 'clearance', severity: 'error', message: 'Trace clearance violation: U1 pad 1 is too close to C1 pad 2 (0.12mm < 0.20mm limit)', refElement: 'U1' },
      { id: '2', type: 'unrouted', severity: 'warning', message: 'Net SIG is partially unrouted between U1 pad 3 and R1 pad 1', refElement: 'R1' }
    ]
  });

  const runLiveDrc = () => {
    setBridgeLogs(prev => [...prev, `[DRC] Running local rule check suite via ${selectedEdaTool.toUpperCase()} API...`]);
    setTimeout(() => {
      setDrcReport({
        ran: true,
        errors: [
          { id: '1', type: 'clearance', severity: 'error', message: 'Trace clearance violation: U1 pad 1 is too close to C1 pad 2 (0.12mm < 0.20mm limit)', refElement: 'U1' },
          { id: '2', type: 'unrouted', severity: 'warning', message: 'Net SIG is partially unrouted between U1 pad 3 and R1 pad 1', refElement: 'R1' }
        ]
      });
      setBridgeLogs(prev => [...prev, `[DRC] Analysis finished: 1 error, 1 warning found.`]);
    }, 600);
  };

  const executeSkidlCompilation = () => {
    setIsCompilingSkidl(true);
    setBridgeLogs(prev => [...prev, `[SKiDL] Parsing script and resolving schematic netlist tree...`]);
    
    setTimeout(() => {
      // Simulate modifying the design in-place!
      const newSchematic = {
        components: [
          { reference: 'U1', value: 'MCU', type: 'Microcontroller' },
          { reference: 'C1', value: '100nF', type: 'Capacitor' },
          { reference: 'R1', value: '10k', type: 'Resistor' },
          { reference: 'C2', value: '10uF', type: 'Capacitor' }, // Generated!
        ],
        nets: [
          { name: 'GND', connections: [{ source: 'U1', target: 'C1' }, { source: 'U1', target: 'R1' }, { source: 'C2', target: 'U1' }] },
          { name: 'VCC', connections: [{ source: 'U1', target: 'C1' }, { source: 'C2', target: 'U1' }] },
          { name: 'SIG', connections: [{ source: 'U1', target: 'R1' }] }
        ]
      };

      const newPcb = {
        board_name: 'generated_board.kicad_pcb',
        board_bounds: { min_x: 0, min_y: 0, max_x: 120, max_y: 120 },
        footprints: [
          { reference: 'U1', value: 'MCU', x: 60, y: 60, orientation: 0, layer: 'F.Cu', pads: [
             { name: '1', net: 'GND', x: 55, y: 55 },
             { name: '2', net: 'VCC', x: 55, y: 65 },
             { name: '3', net: 'SIG', x: 65, y: 55 },
             { name: '4', net: 'CLK', x: 65, y: 65 }
          ] },
          { reference: 'C1', value: '100nF', x: 35, y: 35, orientation: 90, layer: 'F.Cu', pads: [
             { name: '1', net: 'GND', x: 35, y: 33 },
             { name: '2', net: 'VCC', x: 35, y: 37 }
          ] },
          { reference: 'R1', value: '10k', x: 85, y: 85, orientation: 0, layer: 'F.Cu', pads: [
             { name: '1', net: 'SIG', x: 83, y: 85 },
             { name: '2', net: 'GND', x: 87, y: 85 }
          ] },
          { reference: 'C2', value: '10uF', x: 60, y: 90, orientation: 0, layer: 'F.Cu', pads: [
             { name: '1', net: 'GND', x: 58, y: 90 },
             { name: '2', net: 'VCC', x: 62, y: 90 }
          ] }
        ],
        traces: [
          { start_x: 55, start_y: 55, end_x: 35, end_y: 33, net: 'GND', width: 0.6, layer: 'F.Cu' },
          { start_x: 55, start_y: 65, end_x: 35, end_y: 37, net: 'VCC', width: 0.6, layer: 'F.Cu' },
          { start_x: 65, start_y: 55, end_x: 83, end_y: 85, net: 'SIG', width: 0.25, layer: 'F.Cu' },
          { start_x: 58, start_y: 90, end_x: 55, end_y: 65, net: 'GND', width: 0.5, layer: 'F.Cu' } // Additional trace
        ]
      };

      setSchematicData(newSchematic);
      setPcbData(newPcb);
      setBridgeLogs(prev => [
        ...prev, 
        `[SKiDL] SUCCESS: Compiled 4 components, 3 nets into active workspace!`,
        `[SKiDL] Synchronized layout network into SurrealDB local graph cache.`
      ]);
      setIsCompilingSkidl(false);
    }, 1200);
  };

  return (
    <div className="flex flex-col h-full bg-[#141417]/80 backdrop-blur rounded-xl border border-white/5 overflow-hidden shadow-2xl">
      {/* Header with Active Tool Select */}
      <div className="h-12 border-b border-white/5 flex items-center justify-between px-4 bg-[#0F0F11]">
        <div className="flex items-center gap-2">
          <Layers className="w-4 h-4 text-teal-400" />
          <span className="text-xs uppercase tracking-widest text-slate-200 font-bold">Extension Desk</span>
        </div>
        <div className="flex bg-white/5 p-1 rounded-md border border-white/5">
          <button 
            onClick={() => setSelectedEdaTool('kicad')}
            className={`px-2 py-0.5 rounded text-[9px] uppercase tracking-wider transition-all cursor-pointer ${selectedEdaTool === 'kicad' ? 'bg-teal-600 text-white font-bold' : 'text-slate-400 hover:text-white'}`}
          >
            KiCad 8
          </button>
          <button 
            onClick={() => setSelectedEdaTool('altium')}
            className={`px-2 py-0.5 rounded text-[9px] uppercase tracking-wider transition-all cursor-pointer ${selectedEdaTool === 'altium' ? 'bg-teal-600 text-white font-bold' : 'text-slate-400 hover:text-white'}`}
          >
            Altium
          </button>
        </div>
      </div>

      {/* Internal Tabs */}
      <div className="flex bg-[#0F0F11] border-b border-white/5 px-1.5">
        <button 
          onClick={() => setActiveTab('bridge')}
          className={`flex-1 py-2 text-[9px] uppercase tracking-wider font-bold transition-all border-b-2 text-center cursor-pointer ${activeTab === 'bridge' ? 'border-teal-500 text-white' : 'border-transparent text-slate-500 hover:text-slate-300'}`}
        >
          API Bridge
        </button>
        <button 
          onClick={() => setActiveTab('skidl')}
          className={`flex-1 py-2 text-[9px] uppercase tracking-wider font-bold transition-all border-b-2 text-center cursor-pointer ${activeTab === 'skidl' ? 'border-teal-500 text-white' : 'border-transparent text-slate-500 hover:text-slate-300'}`}
        >
          SKiDL
        </button>
        <button 
          onClick={() => setActiveTab('drc')}
          className={`flex-1 py-2 text-[9px] uppercase tracking-wider font-bold transition-all border-b-2 text-center cursor-pointer ${activeTab === 'drc' ? 'border-teal-500 text-white' : 'border-transparent text-slate-500 hover:text-slate-300'}`}
        >
          Live DRC
        </button>
        <button 
          onClick={() => setActiveTab('io')}
          className={`flex-1 py-2 text-[9px] uppercase tracking-wider font-bold transition-all border-b-2 text-center cursor-pointer ${activeTab === 'io' ? 'border-teal-500 text-white' : 'border-transparent text-slate-500 hover:text-slate-300'}`}
        >
          Import / Export
        </button>
      </div>

      {/* Tab Panels */}
      <div className="flex-1 overflow-y-auto p-4 space-y-4 min-h-0 text-xs">
        
        {/* BRIDGE TAB */}
        {activeTab === 'bridge' && (
          <div className="space-y-4">
            <div className="bg-[#0F0F11] border border-white/5 p-3 rounded-lg flex items-center justify-between">
              <div className="space-y-0.5">
                <div className="font-semibold text-slate-200">Active RPC Integration</div>
                <div className="text-[10px] text-slate-500 font-mono">
                  {selectedEdaTool === 'kicad' ? 'Python pcbnew COM' : 'Windows .NET COM Host'}
                </div>
              </div>
              <button 
                onClick={() => {
                  setBridgeConnected(!bridgeConnected);
                  setBridgeLogs(prev => [...prev, `${bridgeConnected ? 'WARNING: Integration disconnected manually' : 'SUCCESS: Real-time sync connection restored'}`]);
                }}
                className={`flex items-center gap-1.5 px-2 bg-transparent text-[10px] uppercase tracking-wider py-1 rounded border transition-all cursor-pointer ${
                  bridgeConnected 
                    ? 'border-teal-500/30 text-teal-400 bg-teal-500/5 hover:bg-teal-500/10' 
                    : 'border-red-500/20 text-red-400 bg-red-500/5 hover:bg-red-500/10'
                }`}
              >
                {bridgeConnected ? <Database className="w-3 h-3" /> : <Unplug className="w-3 h-3" />}
                {bridgeConnected ? 'CONNECTED' : 'DISCONNECTED'}
              </button>
            </div>

            {/* Config details */}
            <div className="bg-[#0F0F11] border border-white/5 p-3 rounded-lg space-y-2">
              <div className="text-[10px] uppercase tracking-widest text-slate-500 font-semibold flex items-center gap-1">
                <Settings className="w-3.5 h-3.5 text-teal-500" />
                ENVIRONMENT SETTINGS
              </div>
              <div className="grid grid-cols-2 gap-2 text-[11px] font-mono">
                <div className="bg-white/5 p-1.5 rounded">
                  <div className="text-[9px] text-slate-500">BIN_PATH</div>
                  <div className="text-slate-300 truncate">/usr/bin/{selectedEdaTool}</div>
                </div>
                <div className="bg-white/5 p-1.5 rounded">
                  <div className="text-[9px] text-slate-500">SURREALDB_URI</div>
                  <div className="text-slate-300 truncate">ws://localhost:8000</div>
                </div>
              </div>
            </div>

            {/* Terminal logs */}
            <div className="space-y-1.5">
              <div className="text-[10px] uppercase tracking-widest text-slate-500 font-semibold flex items-center gap-1">
                <Terminal className="w-3.5 h-3.5 text-teal-500" />
                TELEMETRY LOGS
              </div>
              <div className="bg-[#0A0A0B] border border-white/5 p-3 rounded-lg font-mono text-[10px] text-slate-400 space-y-1.5 max-h-48 overflow-y-auto">
                {bridgeLogs.map((log, i) => (
                  <div key={i} className={`leading-relaxed border-l-2 pl-2 ${
                    log.includes('SUCCESS') ? 'border-teal-500 text-teal-400' :
                    log.includes('WARNING') ? 'border-red-500 text-red-400 font-bold' :
                    log.includes('[DRC]') ? 'border-orange-500 text-orange-400' :
                    'border-slate-800 text-slate-400'
                  }`}>
                    {log}
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}

        {/* SKIDL SCHEMATIC GENERATOR */}
        {activeTab === 'skidl' && (
          <div className="space-y-4">
            <div className="space-y-1">
              <div className="text-[10px] uppercase tracking-widest text-slate-500 font-semibold flex items-center gap-1">
                <Code className="w-3.5 h-3.5 text-teal-500" />
                Python programmatic engine
              </div>
              <div className="text-[10px] text-slate-400">Generate footprints and nets directly inside the active workspace.</div>
            </div>

            <textarea
              value={skidlScript}
              onChange={(e) => setSkidlScript(e.target.value)}
              className="w-full h-44 bg-[#0A0A0B] text-slate-300 font-mono text-[10px] p-3 rounded-lg border border-white/5 focus:border-teal-500/40 outline-none resize-none leading-relaxed"
            />

            <button 
              onClick={executeSkidlCompilation}
              disabled={isCompilingSkidl}
              className="w-full py-2.5 bg-teal-600 hover:bg-teal-500 disabled:bg-teal-800 text-white text-xs font-bold uppercase tracking-widest rounded-lg transition-all shadow-lg shadow-teal-900/20 flex items-center justify-center gap-2 cursor-pointer"
            >
              {isCompilingSkidl ? (
                <>
                  <RefreshCw className="w-3.5 h-3.5 animate-spin" />
                  COMPILING CIRCUIT...
                </>
              ) : (
                <>
                  <Play className="w-3.5 h-3.5 fill-current" />
                  GENERATE & SYNC WORKSHEETS
                </>
              )}
            </button>
          </div>
        )}

        {/* REAL-TIME DRC CHECKER */}
        {activeTab === 'drc' && (
          <div className="space-y-4">
            <div className="flex justify-between items-center">
              <div className="text-[10px] uppercase tracking-widest text-slate-500 font-semibold flex items-center gap-1">
                <FileCheck2 className="w-3.5 h-3.5 text-teal-500" />
                Electrical & Trace Rules
              </div>
              <button 
                onClick={runLiveDrc}
                className="px-2 py-1 bg-white/5 border border-white/10 rounded-md hover:bg-white/10 transition-colors cursor-pointer text-[9px] uppercase tracking-wider text-slate-300 font-bold"
              >
                Trigger Check
              </button>
            </div>

            {drcReport.ran && (
              <div className="space-y-2">
                {drcReport.errors.map((err) => (
                  <div 
                    key={err.id}
                    onClick={() => err.refElement && selectComponent(err.refElement)}
                    className={`p-3 rounded-lg border transition-all cursor-pointer flex gap-3 ${
                      err.severity === 'error' 
                        ? 'bg-red-500/5 border-red-500/10 hover:border-red-500/30' 
                        : 'bg-orange-500/5 border-orange-500/10 hover:border-orange-500/30'
                    }`}
                  >
                    <div className="pt-0.5 shrink-0">
                      {err.severity === 'error' ? (
                        <Flame className="w-4 h-4 text-red-500" />
                      ) : (
                        <AlertTriangle className="w-4 h-4 text-orange-500" />
                      )}
                    </div>
                    <div className="space-y-1">
                      <div className="flex items-center gap-1.5">
                        <span className={`text-[9px] uppercase tracking-wider font-bold ${err.severity === 'error' ? 'text-red-400' : 'text-orange-400'}`}>
                          {err.severity}
                        </span>
                        {err.refElement && (
                          <span className="bg-white/5 px-1 rounded text-[9px] font-mono text-slate-400">
                            {err.refElement}
                          </span>
                        )}
                      </div>
                      <div className="text-[11px] text-slate-300 leading-normal">
                        {err.message}
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}

        {/* CAD EXCHANGE SYSTEM */}
        {activeTab === 'io' && (
          <div className="h-full">
            <CadExchanger />
          </div>
        )}


      </div>
    </div>
  );
}
