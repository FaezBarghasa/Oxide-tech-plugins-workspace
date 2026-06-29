import React from 'react';
import { ParametricInputPanel } from '../panels/ParametricInputPanel';
import { Scene } from '../3d/Scene';
import { StatusIndicator } from './StatusIndicator';
import { ChatPanel } from '../panels/ChatPanel';
import { BlenderExtensionModal } from '../modals/BlenderExtensionModal';
import { Box, Settings2, Download, Play, Puzzle } from 'lucide-react';

export function MainLayout() {
  const [isModalOpen, setIsModalOpen] = React.useState(false);

  return (
    <div className="flex flex-col h-screen w-screen bg-[#09090b] text-[#fafafa] overflow-hidden font-sans p-3 gap-3">
      {/* Header Pipeline Toolbar */}
      <header className="flex items-center justify-between px-5 py-3 bg-[#18181b] border border-[#27272a] rounded-2xl shrink-0">
        <div className="flex items-center gap-3">
          <div className="bg-[#27272a] p-1.5 rounded-lg flex items-center justify-center">
            <Box size={18} className="text-white" />
          </div>
          <div>
            <h1 className="text-sm font-bold tracking-tight text-white leading-none">CAD Control Plane</h1>
            <p className="text-[#a1a1aa] text-[10px] font-mono mt-0.5">SYSTEM_VER ✨ v2.1.0</p>
          </div>
        </div>

        {/* Integrated Status indicator directly in header */}
        <div className="hidden md:flex items-center">
          <StatusIndicator />
        </div>
        
        <div className="flex items-center gap-2">
          <button 
            onClick={() => setIsModalOpen(true)}
            className="flex items-center gap-1.5 px-3 py-1.5 text-xs bg-white hover:bg-gray-200 text-black font-semibold rounded-lg transition shadow-sm"
          >
            <Puzzle size={14} /> Blender Extension
          </button>
          <div className="w-px h-5 bg-[#27272a] mx-1"></div>
          <button className="flex items-center gap-1.5 px-3 py-1.5 text-xs bg-[#09090b] hover:bg-[#27272a] text-[#fafafa] rounded-lg border border-[#27272a] transition">
            <Settings2 size={14} /> Rule Book
          </button>
          <button className="flex items-center gap-1.5 px-3 py-1.5 text-xs bg-[#09090b] hover:bg-[#27272a] text-[#fafafa] rounded-lg border border-[#27272a] transition">
            <Play size={14} /> Simulate
          </button>
          <button className="flex items-center gap-1.5 px-3 py-1.5 text-xs bg-[#09090b] hover:bg-[#27272a] text-[#fafafa] rounded-lg border border-[#27272a] transition">
            <Download size={14} /> STEP
          </button>
        </div>
      </header>

      {/* Main Content Split */}
      <main className="flex flex-1 overflow-hidden gap-3">
        {/* Left Panel: Params */}
        <div className="bg-[#18181b] border border-[#27272a] rounded-2xl overflow-hidden flex flex-col w-72 shrink-0">
          <ParametricInputPanel />
        </div>
        
        {/* Center/Right: 3D Viewport */}
        <div className="flex-grow relative flex flex-col min-w-0 bg-[#18181b] border border-[#27272a] rounded-2xl overflow-hidden">
          <div className="absolute top-4 left-4 z-10 flex gap-1.5">
            <div className="px-2.5 py-1 bg-[#09090b]/80 backdrop-blur border border-[#27272a] rounded-lg font-mono text-[10px] text-[#a1a1aa] font-bold tracking-widest uppercase shadow-sm">
              Perspective
            </div>
            <div className="px-2.5 py-1 bg-[#09090b]/80 backdrop-blur border border-[#27272a] rounded-lg font-mono text-[10px] text-[#a1a1aa] font-bold tracking-widest uppercase shadow-sm">
              Shaded
            </div>
          </div>
          <Scene />
        </div>

        {/* Right Panel: AI Chat assistant */}
        <ChatPanel />
      </main>

      {/* Blender Extension Modal */}
      <BlenderExtensionModal isOpen={isModalOpen} onClose={() => setIsModalOpen(false)} />
    </div>
  );
}
