import { create } from 'zustand';
import { EnclosureParams } from '../types/blender';
import * as THREE from 'three';

export const DEFAULT_ENCLOSURE: EnclosureParams = {
  dimensions: { width: 100, height: 80, depth: 60 },
  wallThickness: 2.5,
  material: 'pla',
  ventConfig: { holeDiameter: 3, spacing: 5, quantity: 12 }
};

interface EnclosureStoreState {
  currentEnclosure: EnclosureParams;
  generatedMesh: THREE.Group | null;
  meshBinary: ArrayBuffer | null;
  history: EnclosureParams[];
  historyIndex: number;
  
  updateParams: (params: EnclosureParams) => void;
  setMesh: (mesh: THREE.Group, binary: ArrayBuffer) => void;
  undo: () => void;
  redo: () => void;
}

export const useEnclosureStore = create<EnclosureStoreState>((set) => ({
  currentEnclosure: DEFAULT_ENCLOSURE,
  generatedMesh: null,
  meshBinary: null,
  history: [DEFAULT_ENCLOSURE],
  historyIndex: 0,
  
  updateParams: (params) => set((state) => {
    const newHistory = state.history.slice(0, state.historyIndex + 1);
    newHistory.push(params);
    return {
      currentEnclosure: params,
      history: newHistory.slice(-20), // Keep only last 20
      historyIndex: newHistory.length - 1,
    };
  }),
  
  setMesh: (mesh, binary) => set({ generatedMesh: mesh, meshBinary: binary }),
  
  undo: () => set((state) => {
    if (state.historyIndex <= 0) return state;
    return {
      historyIndex: state.historyIndex - 1,
      currentEnclosure: state.history[state.historyIndex - 1],
    };
  }),
  
  redo: () => set((state) => {
    if (state.historyIndex >= state.history.length - 1) return state;
    return {
      historyIndex: state.historyIndex + 1,
      currentEnclosure: state.history[state.historyIndex + 1],
    };
  }),
}));
