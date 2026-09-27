export const MATERIALS = [
  { id: 'pla', name: 'PLA', color: '#cccccc' },
  { id: 'abs', name: 'ABS', color: '#aaaaaa' },
  { id: 'petg', name: 'PETG', color: '#dddddd' },
  { id: 'aluminum', name: 'Aluminum', color: '#999999' },
] as const;

export type MaterialId = typeof MATERIALS[number]['id'];
