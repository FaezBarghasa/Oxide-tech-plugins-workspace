import { create } from 'zustand';

interface SelectionState {
  selectedReferences: Set<string>;
  selectedNets: Set<string>;
  selectedPins: Set<string>;
  
  selectComponent: (ref: string, addToSelection?: boolean) => void;
  selectNet: (netName: string, addToSelection?: boolean) => void;
  clearSelection: () => void;
  toggleComponent: (ref: string) => void;
}

export const useSelectionStore = create<SelectionState>((set) => ({
  selectedReferences: new Set(),
  selectedNets: new Set(),
  selectedPins: new Set(),
  
  selectComponent: (ref, addToSelection = false) => set((state) => {
    const newRefs = new Set(addToSelection ? state.selectedReferences : []);
    newRefs.add(ref);
    return { selectedReferences: newRefs };
  }),
  
  selectNet: (netName, addToSelection = false) => set((state) => {
    const newNets = new Set(addToSelection ? state.selectedNets : []);
    newNets.add(netName);
    return { selectedNets: newNets };
  }),
  
  clearSelection: () => set({
    selectedReferences: new Set(),
    selectedNets: new Set(),
    selectedPins: new Set(),
  }),
  
  toggleComponent: (ref) => set((state) => {
    const newRefs = new Set(state.selectedReferences);
    newRefs.has(ref) ? newRefs.delete(ref) : newRefs.add(ref);
    return { selectedReferences: newRefs };
  }),
}));
