export interface Pad {
  name: string;
  net: string;
  x: number; // in mm
  y: number; // in mm
}

export interface Footprint {
  reference: string;
  value: string;
  x: number; // in mm
  y: number; // in mm
  orientation: number; // in degrees
  layer: string; // e.g., 'F.Cu', 'B.Cu'
  pads: Pad[];
}

export interface Trace {
  start_x: number;
  start_y: number;
  end_x: number;
  end_y: number;
  net: string;
  width: number;
  layer: string;
}

export interface PCBBounds {
  min_x: number;
  min_y: number;
  max_x: number;
  max_y: number;
}

export interface PCBData {
  board_name: string;
  footprints: Footprint[];
  traces: Trace[];
  board_bounds: PCBBounds;
}
