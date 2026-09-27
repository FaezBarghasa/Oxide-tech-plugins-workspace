import React from 'react';
import { useBlenderStore } from '../../state/blenderStore';

export function StatusIndicator() {
  const { isConnected, status, progress, lastError } = useBlenderStore();

  return (
    <div className="flex items-center gap-3">
      {/* Node status */}
      <div className="flex items-center gap-1.5 px-2.5 py-1 bg-[#09090b] border border-[#27272a] rounded-lg">
        <div className={`w-1.5 h-1.5 rounded-full ${isConnected ? 'bg-green-500 animate-pulse' : 'bg-red-500'}`} />
        <span className="text-[10px] uppercase tracking-wider font-mono font-bold text-[#fafafa]">
          {isConnected ? 'LIVE' : 'OFFLINE'}
        </span>
      </div>
      
      {/* Pipeline progress */}
      <div className="flex items-center gap-2">
        <span className="text-[10px] font-bold uppercase tracking-widest text-[#71717a]">
          {status}
        </span>
        {status === 'generating' && (
          <div className="w-16 bg-[#09090b] rounded-full h-1 overflow-hidden border border-[#27272a]">
            <div 
              className="bg-white h-full transition-all duration-300"
              style={{ width: `${progress}%` }}
            />
          </div>
        )}
      </div>

      {lastError && (
        <span className="text-[10px] text-red-500 font-mono font-medium">
          ERR
        </span>
      )}
    </div>
  );
}
