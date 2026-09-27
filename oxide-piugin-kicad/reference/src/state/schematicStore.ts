import { create } from 'zustand';
import { SchematicData } from '../types/schematic';

interface SchematicState {
  schematicData: SchematicData | null;
  isLoading: boolean;
  error: string | null;
  setSchematicData: (data: SchematicData) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
}

export const useSchematicStore = create<SchematicState>((set) => ({
  schematicData: null,
  isLoading: false,
  error: null,
  
  setSchematicData: (data) => set({ schematicData: data, error: null }),
  setLoading: (loading) => set({ isLoading: loading }),
  setError: (error) => set({ error, isLoading: false }),
}));
