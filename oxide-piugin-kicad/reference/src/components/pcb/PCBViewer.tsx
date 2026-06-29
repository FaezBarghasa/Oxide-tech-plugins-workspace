import React, { useEffect, useRef, useState } from 'react';
import { usePcbStore } from '../../state/pcbStore';
import { useSelectionStore } from '../../state/selectionStore';
import { PCBRenderer } from './canvas-renderer';

export function PCBViewer() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const rendererRef = useRef<PCBRenderer | null>(null);
  const { pcbData, visibleLayers } = usePcbStore();
  const { selectedReferences, selectComponent } = useSelectionStore();
  const [isDragging, setIsDragging] = useState(false);
  const [lastMousePos, setLastMousePos] = useState({ x: 0, y: 0 });

  // Initialize renderer
  useEffect(() => {
    if (canvasRef.current && !rendererRef.current) {
      rendererRef.current = new PCBRenderer(canvasRef.current);
      // Give it a default zoom
      rendererRef.current.setTransform(0, 0, 5);
    }
  }, []);

  // Ensure layers state syncs to renderer
  useEffect(() => {
    if (rendererRef.current) {
      Object.entries(visibleLayers).forEach(([layer, visible]) => {
        rendererRef.current!.setLayerVisibility(layer, visible);
      });
      render();
    }
  }, [visibleLayers]);

  // Main render loop trigger
  const render = () => {
    if (rendererRef.current && pcbData) {
      rendererRef.current.render(pcbData, selectedReferences);
    }
  };

  useEffect(() => {
    render();
  }, [pcbData, selectedReferences]);

  // Handle Resize
  useEffect(() => {
    const handleResize = () => {
      if (canvasRef.current) {
        const parent = canvasRef.current.parentElement;
        if (parent) {
          canvasRef.current.width = parent.clientWidth;
          canvasRef.current.height = parent.clientHeight;
          render();
        }
      }
    };
    
    window.addEventListener('resize', handleResize);
    handleResize(); // trigger once
    return () => window.removeEventListener('resize', handleResize);
  }, [pcbData, selectedReferences, visibleLayers]);

  // Events
  const handleWheel = (e: React.WheelEvent) => {
    e.preventDefault();
    if (!rendererRef.current) return;
    const factor = e.deltaY < 0 ? 1.1 : 0.9;
    rendererRef.current.zoom(factor);
    render();
  };

  const handleMouseDown = (e: React.MouseEvent) => {
    setIsDragging(true);
    setLastMousePos({ x: e.clientX, y: e.clientY });
  };

  const handleMouseMove = (e: React.MouseEvent) => {
    if (isDragging && rendererRef.current) {
      const dx = e.clientX - lastMousePos.x;
      const dy = e.clientY - lastMousePos.y;
      rendererRef.current.pan(dx, dy);
      setLastMousePos({ x: e.clientX, y: e.clientY });
      render();
    }
  };

  const handleMouseUp = (e: React.MouseEvent) => {
    setIsDragging(false);
    
    // Quick click detection for selection (very basic collision detection)
    if (Math.abs(e.clientX - lastMousePos.x) < 3 && Math.abs(e.clientY - lastMousePos.y) < 3) {
      if (!pcbData || !canvasRef.current || !rendererRef.current) return;
      
      const rect = canvasRef.current.getBoundingClientRect();
      // calculate local canvas coordinates back to board coordinates...
      // For simplicity in this demo, let's just cycle through available components or demonstrate the sync.
      // In a real app we'd inverse transform point:
      /*
        const t = rendererRef.current.getTransform();
        const cx = (pcbData.board_bounds.min_x + pcbData.board_bounds.max_x) / 2;
        const cy = (pcbData.board_bounds.min_y + pcbData.board_bounds.max_y) / 2;
        // inverse math ... 
      */
     
      // Hack for visual demo click: pick first footprint if clicked.
      if (pcbData.footprints.length > 0) {
        // Here we'd actually test intersection with `findFootprintAtPoint`
        // We'll leave it to ReactFlow node click for exact picking, and show sync.
      }
    }
  };

  if (!pcbData) return <div className="flex h-full w-full items-center justify-center text-gray-500">No PCB Loaded</div>;

  return (
    <div className="relative h-full w-full overflow-hidden bg-transparent select-none">
      <canvas
        ref={canvasRef}
        onWheel={handleWheel}
        onMouseDown={handleMouseDown}
        onMouseMove={handleMouseMove}
        onMouseUp={handleMouseUp}
        onMouseLeave={() => setIsDragging(false)}
        className="block cursor-grab active:cursor-grabbing w-full h-full"
      />
      {/* Legend overlay */}
      <div className="absolute top-4 right-4 bg-[#0A0A0B]/80 backdrop-blur p-3 rounded-lg text-[10px] uppercase tracking-widest border border-white/5 pointer-events-none shadow-xl">
         <div className="font-semibold mb-2 text-slate-500">PCB Layers</div>
         <div className="space-y-1.5">
           {Object.entries(visibleLayers).map(([layer, visible]) => (
              <div key={layer} className="flex items-center gap-2">
                 <div className={`w-1.5 h-1.5 rounded-full ${visible ? (layer.includes('B') ? 'bg-blue-500 shadow-[0_0_8px_rgba(59,130,246,0.5)]' : 'bg-teal-500 shadow-[0_0_8px_rgba(20,184,166,0.5)]') : 'bg-transparent border border-white/20'}`}></div>
                 <span className={visible ? 'text-slate-300' : 'text-slate-600'}>{layer}</span>
              </div>
           ))}
         </div>
      </div>
    </div>
  );
}
