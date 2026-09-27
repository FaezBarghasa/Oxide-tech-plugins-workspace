import React, { useEffect, useState } from 'react';
import { SchematicViewer } from '../schematic/SchematicViewer';
import { PCBViewer } from '../pcb/PCBViewer';
import { ChatAssistant } from '../panels/ChatAssistant';
import { ExtensionControlCenter } from '../panels/ExtensionControlCenter';
import { ComponentLibraryPanel } from '../panels/ComponentLibraryPanel';
import { ComponentDesigner } from '../panels/ComponentDesigner';
import { useSchematicStore } from '../../state/schematicStore';
import { usePcbStore } from '../../state/pcbStore';
import { useSelectionStore } from '../../state/selectionStore';
import { loadPCB, loadSchematic } from '../../services/kicad';
import { CircuitBoard, Waypoints, Play, Box, Sparkles, Terminal, Cpu } from 'lucide-react';

export function MainLayout() {
  const { setSchematicData, isLoading: schLoading } = useSchematicStore();
  const { setPcbData, visibleLayers, toggleLayer, isLoading: pcbLoading } = usePcbStore();
  const { selectedReferences } = useSelectionStore();
  const [showChat, setShowChat] = useState(false);
  const [showExtensions, setShowExtensions] = useState(false);
  const [showLibrary, setShowLibrary] = useState(false);
  const [showDesigner, setShowDesigner] = useState(true);

  useEffect(() => {
    // Initial Load Demonstration
    const initLoad = async () => {
      try {
        const sch = await loadSchematic('test.kicad_sch');
        setSchematicData(sch);
        const pcb = await loadPCB('test.kicad_pcb');
        setPcbData(pcb);
      } catch (err) {
        console.error("Failed to load initial mock files", err);
      }
    };
    initLoad();
  }, [setSchematicData, setPcbData]);

  return (
    <div className="flex h-screen bg-[#0A0A0B] text-slate-300 font-sans overflow-hidden">
      {/* Sidebar */}
      <aside className="w-12 bg-[#0F0F11] border-r border-white/5 flex flex-col items-center py-4 space-y-5 shrink-0 z-10 shadow-lg animate-fade-in">
        <button className="text-teal-500 hover:text-teal-400 transition-colors cursor-pointer" title="Project"><CircuitBoard size={18} /></button>
        <button className="text-slate-500 hover:text-white transition-colors cursor-pointer" title="Schematic"><Waypoints size={18} /></button>
        <button 
          onClick={() => setShowExtensions(!showExtensions)}
          className={`transition-all cursor-pointer ${showExtensions ? 'text-teal-400 scale-110 drop-shadow-[0_0_8px_rgba(20,184,166,0.4)]' : 'text-slate-500 hover:text-white'}`} 
          title="Extension Desk / Live DRC"
        >
          <Play size={18} />
        </button>
        <button 
          onClick={() => setShowChat(!showChat)}
          className={`transition-all cursor-pointer ${showChat ? 'text-teal-400 scale-110 drop-shadow-[0_0_8px_rgba(20,184,166,0.4)]' : 'text-slate-500 hover:text-white'}`} 
          title="AI Command Chat"
        >
          <Sparkles size={18} />
        </button>
        <button 
          onClick={() => setShowDesigner(!showDesigner)}
          className={`transition-all cursor-pointer ${showDesigner ? 'text-teal-400 scale-110 drop-shadow-[0_0_8px_rgba(20,184,166,0.4)]' : 'text-slate-500 hover:text-white'}`} 
          title="Datasheet Component Designer"
        >
          <Cpu size={18} />
        </button>
        <button 
          onClick={() => setShowLibrary(!showLibrary)}
          className={`transition-all mt-auto mb-2 cursor-pointer ${showLibrary ? 'text-teal-400 scale-110 drop-shadow-[0_0_8px_rgba(20,184,166,0.4)]' : 'text-slate-500 hover:text-white'}`} 
          title="Library & Tree"
        >
          <Box size={18} />
        </button>
      </aside>

      {/* Main Content Area */}
      <main className="flex-1 flex flex-col min-w-0">
         {/* Top Toolbar */}
         <header className="h-11 bg-[#0F0F11] border-b border-[#1f1f23]/40 flex items-center px-4 justify-between shrink-0">
            <div className="flex items-center space-x-4 text-xs font-medium">
               <div className="flex items-center gap-1.5 animate-fade-in">
                 <div className="w-3.5 h-3.5 bg-teal-500 rounded-sm rotate-45 shadow-[0_0_8px_rgba(20,184,166,0.3)]"></div>
                 <span className="font-bold text-white tracking-tight uppercase text-xs">Build.OS</span>
               </div>
               <div className="h-3 w-[1px] bg-white/10"></div>
               <span className="text-teal-400 font-mono text-[10px] uppercase tracking-widest">
                  {schLoading || pcbLoading ? 'Loading Engine...' : 'Engine Online'}
               </span>
            </div>
            
            <div className="flex gap-1.5">
               {['F.Cu', 'B.Cu', 'F.Silkscreen'].map(layer => (
                  <button 
                     key={layer}
                     onClick={() => toggleLayer(layer)}
                     className={`px-2.5 py-0.5 rounded text-[8px] font-bold uppercase tracking-widest transition-all cursor-pointer ${visibleLayers[layer] ? 'bg-teal-600 text-white shadow shadow-teal-900/20' : 'bg-white/5 text-slate-400 border border-white/5 hover:bg-white/10'}`}
                  >
                     {layer}
                  </button>
               ))}
            </div>
         </header>

         {/* Workspace Split */}
         <div className="flex-1 flex min-h-0 bg-[#0A0A0B] p-2 gap-2 overflow-hidden">
            {/* Library Panel (Left) */}
            {showLibrary && (
              <div className="w-64 lg:w-72 flex flex-col shrink-0 text-slate-300">
                <ComponentLibraryPanel />
              </div>
            )}

            {/* Left: Schematic */}
            <div className="flex-1 flex flex-col bg-[#141417] border border-white/5 rounded-lg relative overflow-hidden">
               <div className="absolute top-2.5 left-2.5 z-10 px-2 py-0.5 bg-white/5 rounded text-[8px] uppercase tracking-widest text-slate-400 border border-white/5 backdrop-blur pointer-events-none font-bold">
                  Schematic Entry
               </div>
               <SchematicViewer />
            </div>

            {/* Right: PCB Layout */}
            <div className="flex-1 flex flex-col bg-[#141417] border border-white/5 rounded-lg relative overflow-hidden">
               <div className="absolute top-2.5 left-2.5 z-10 px-2 py-0.5 bg-white/5 rounded text-[8px] uppercase tracking-widest text-slate-400 border border-white/5 backdrop-blur pointer-events-none font-bold">
                  PCB Canvas
               </div>
               <PCBViewer />
            </div>

            {/* Component Designer (Datasheet to CAD) */}
            {showDesigner && (
              <div className="w-72 lg:w-80 flex flex-col shrink-0">
                <ComponentDesigner />
              </div>
            )}

            {/* Chat Assistant */}
            {showChat && (
              <div className="w-72 lg:w-80 flex flex-col shrink-0">
                <ChatAssistant />
              </div>
            )}

            {/* Extension Control Desk */}
            {showExtensions && (
              <div className="w-72 lg:w-80 flex flex-col shrink-0">
                <ExtensionControlCenter />
              </div>
            )}
         </div>
         
         {/* Status Bar */}
         <footer className="h-7 bg-[#0F0F11] border-t border-white/5 px-4 flex items-center justify-between text-[9px] font-mono text-slate-500 shrink-0 uppercase">
            <div className="flex gap-4">
               <span className="flex items-center gap-1">
                  <span className="w-1 h-1 rounded-full bg-teal-500 shadow-[0_0_8px_rgba(20,184,166,0.5)]"></span>
                  SYSTEM READY
               </span>
               <span className="text-slate-500 ml-2">Selected Node: <span className="text-teal-400">{selectedReferences.size > 0 ? Array.from(selectedReferences).join(', ') : 'None'}</span></span>
            </div>
            <div className="flex gap-4">
               <span>UTC-08:00</span>
               <span className="text-white">Build v4.2.0.881</span>
            </div>
         </footer>
      </main>
    </div>
  );
}
