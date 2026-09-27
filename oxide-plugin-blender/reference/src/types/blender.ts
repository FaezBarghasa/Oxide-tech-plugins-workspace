export interface Dimensions {
  width: number;
  height: number;
  depth: number;
}

export interface VentConfig {
  holeDiameter: number;
  spacing: number;
  quantity: number;
}

export interface EnclosureParams {
  dimensions: Dimensions;
  wallThickness: number;
  material: string;
  ventConfig: VentConfig;
  pcbWidth?: number;
}

export interface BlenderResponse<T = any> {
  status: 'success' | 'error';
  data?: T;
  error_code?: string;
  message?: string;
}

export interface BlenderResult {
  mesh_size_bytes: number;
  mesh_buffer: ArrayBuffer;
}

export class BlenderError extends Error {
  constructor(public code: string, message: string) {
    super(message);
    this.name = 'BlenderError';
  }
}
