import { create } from 'zustand';
import { PCBData } from '../types/pcb';

interface PCBState {
  pcbData: PCBData | null;
  isLoading: boolean;
  error: string | null;
  visibleLayers: Record<string, boolean>;
  setPcbData: (data: PCBData) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  toggleLayer: (layer: string) => void;
  setLayerVisibility: (layer: string, visible: boolean) => void;
}

export const usePcbStore = create<PCBState>((set) => ({
  pcbData: null,
  isLoading: false,
  error: null,
  visibleLayers: {
    'F.Cu': true,
    'B.Cu': true,
    'F.Silkscreen': true,
    'B.Silkscreen': false,
    'Edge.Cuts': true,
  },
  
  setPcbData: (data) => set({ pcbData: data, error: null }),
  setLoading: (loading) => set({ isLoading: loading }),
  setError: (error) => set({ error, isLoading: false }),
  
  toggleLayer: (layer) => set((state) => ({
    visibleLayers: {
      ...state.visibleLayers,
      [layer]: !state.visibleLayers[layer]
    }
  })),
  
  setLayerVisibility: (layer, visible) => set((state) => ({
    visibleLayers: {
      ...state.visibleLayers,
      [layer]: visible
    }
  })),
}));
