import { create } from 'zustand';

interface BlenderStoreState {
  isConnected: boolean;
  status: 'idle' | 'generating' | 'rendering' | 'error';
  progress: number;
  lastError: string | null;
  
  setConnectionStatus: (connected: boolean) => void;
  setStatus: (status: BlenderStoreState['status']) => void;
  setProgress: (progress: number) => void;
  setError: (error: string | null) => void;
}

export const useBlenderStore = create<BlenderStoreState>((set) => ({
  isConnected: false,
  status: 'idle',
  progress: 0,
  lastError: null,
  
  setConnectionStatus: (connected) => set({ isConnected: connected }),
  setStatus: (status) => set({ status }),
  setProgress: (progress) => set({ progress }),
  setError: (error) => set({ lastError: error, status: error ? 'error' : 'idle' }),
}));
