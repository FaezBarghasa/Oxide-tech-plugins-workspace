import { EnclosureParams } from '../types/blender';

export const DIMENSION_CONSTRAINTS = {
  MIN_WIDTH: 50,  // mm
  MAX_WIDTH: 500,
  MIN_HEIGHT: 50,
  MAX_HEIGHT: 500,
  MIN_DEPTH: 30,
  MAX_DEPTH: 500,
  MIN_WALL_THICKNESS: 1.5,
  MAX_WALL_THICKNESS: 10,
  MIN_VENT_HOLE_DIAMETER: 2,
  MAX_VENT_HOLE_DIAMETER: 50,
};

export interface ValidationResult {
  valid: boolean;
  errors: string[];
}

export function validateEnclosureParams(params: EnclosureParams): ValidationResult {
  const errors: string[] = [];
  
  if (params.dimensions.width < DIMENSION_CONSTRAINTS.MIN_WIDTH) {
    errors.push(`Width must be ≥ ${DIMENSION_CONSTRAINTS.MIN_WIDTH}mm`);
  }
  
  if (params.dimensions.width > DIMENSION_CONSTRAINTS.MAX_WIDTH) {
    errors.push(`Width must be ≤ ${DIMENSION_CONSTRAINTS.MAX_WIDTH}mm`);
  }

  if (params.dimensions.height < DIMENSION_CONSTRAINTS.MIN_HEIGHT) {
    errors.push(`Height must be ≥ ${DIMENSION_CONSTRAINTS.MIN_HEIGHT}mm`);
  }
  
  if (params.dimensions.depth < DIMENSION_CONSTRAINTS.MIN_DEPTH) {
    errors.push(`Depth must be ≥ ${DIMENSION_CONSTRAINTS.MIN_DEPTH}mm`);
  }
  
  const MIN_CLEARANCE = 5;
  if (params.pcbWidth && params.dimensions.width < params.pcbWidth + 2 * MIN_CLEARANCE) {
    errors.push(`Enclosure width insufficient for PCB + clearance`);
  }
  
  const shortestDim = Math.min(params.dimensions.width, params.dimensions.height, params.dimensions.depth);
  if (params.ventConfig.holeDiameter > shortestDim * 0.3) {
    errors.push(`Vent holes too large for enclosure dimensions`);
  }
  
  return {
    valid: errors.length === 0,
    errors,
  };
}
