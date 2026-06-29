import { PCBData, Footprint } from '../../types/pcb';

export class PCBRenderer {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private layers: Map<string, boolean> = new Map();
  private transform: { x: number; y: number; scale: number } = { x: 0, y: 0, scale: 1 };
  
  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    this.ctx = canvas.getContext('2d')!;
    this.setupLayers();
  }
  
  private setupLayers() {
    this.layers.set('F.Cu', true);
    this.layers.set('B.Cu', true);
    this.layers.set('F.Silkscreen', true);
    this.layers.set('B.Silkscreen', false);
    this.layers.set('Edge.Cuts', true);
  }
  
  public render(pcbData: PCBData, selectedRefs: Set<string>) {
    const ctx = this.ctx;
    const { x, y, scale } = this.transform;
    
    ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
    
    ctx.save();
    // Center initially or use constraints
    ctx.translate(this.canvas.width / 2 + x, this.canvas.height / 2 + y);
    ctx.scale(scale, scale);
    // Center the board data to 0,0 locally
    const cx = (pcbData.board_bounds.min_x + pcbData.board_bounds.max_x) / 2 || 50;
    const cy = (pcbData.board_bounds.min_y + pcbData.board_bounds.max_y) / 2 || 50;
    ctx.translate(-cx, -cy);
    
    
    // Draw in order: bottom copper, inner planes, top copper, silkscreen
    const layerOrder = ['B.Cu', 'B.Silkscreen', 'F.Cu', 'F.Silkscreen'];
    
    for (const layer of layerOrder) {
      if (!this.layers.get(layer)) continue;
      this.drawLayer(ctx, pcbData, layer, selectedRefs);
    }
    
    ctx.restore();
  }
  
  private drawLayer(
    ctx: CanvasRenderingContext2D,
    pcbData: PCBData,
    layer: string,
    selectedRefs: Set<string>
  ) {
    // Draw traces for this layer
    const layerTraces = pcbData.traces.filter(t => t.layer === layer);
    layerTraces.forEach(trace => {
      ctx.strokeStyle = layer.includes('B') ? 'rgba(59, 130, 246, 0.8)' : 'rgba(249, 115, 22, 0.8)';
      ctx.lineWidth = trace.width;
      ctx.lineCap = 'round';
      ctx.beginPath();
      ctx.moveTo(trace.start_x, trace.start_y);
      ctx.lineTo(trace.end_x, trace.end_y);
      ctx.stroke();
    });
    
    // Draw footprints on this layer
    const layerFootprints = pcbData.footprints.filter(f => f.layer === layer);
    layerFootprints.forEach(footprint => {
      this.drawFootprint(ctx, footprint, selectedRefs.has(footprint.reference), layer);
    });
  }
  
  private drawFootprint(
    ctx: CanvasRenderingContext2D,
    footprint: Footprint,
    isSelected: boolean,
    layerName: string
  ) {
    ctx.save();
    ctx.translate(footprint.x, footprint.y);
    ctx.rotate((footprint.orientation * Math.PI) / 180);
    
    // Draw pads
    footprint.pads.forEach(pad => {
      // Local coordinates for pad relative to footprint
      const padLx = pad.x - footprint.x;
      const padLy = pad.y - footprint.y;
      
      ctx.fillStyle = layerName.includes('B') ? '#6699ff' : '#ffcc00';
      ctx.fillRect(padLx - 2, padLy - 2, 4, 4);
      
      // Pad label
      ctx.fillStyle = '#000000';
      ctx.font = '2px monospace';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      ctx.fillText(pad.name, padLx, padLy);
    });
    
    // Draw outline
    ctx.strokeStyle = isSelected ? '#14b8a6' : 'rgba(255, 255, 255, 0.2)';
    ctx.lineWidth = isSelected ? 0.8 : 0.2;
    ctx.strokeRect(-10, -10, 20, 20);
    
    // Reference designator
    ctx.fillStyle = isSelected ? '#ccfbf1' : '#94a3b8';
    ctx.font = 'bold 4px sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText(footprint.reference, 0, 12);
    
    ctx.restore();
  }
  
  public pan(dx: number, dy: number) {
    this.transform.x += dx;
    this.transform.y += dy;
  }
  
  public zoom(scaleFactor: number) {
    this.transform.scale *= scaleFactor;
  }
  
  public getTransform() {
    return this.transform;
  }

  public setTransform(x: number, y: number, scale: number) {
     this.transform = { x, y, scale };
  }
  
  public setLayerVisibility(layer: string, visible: boolean) {
    this.layers.set(layer, visible);
  }

  public getLayers() {
     return this.layers;
  }
}
