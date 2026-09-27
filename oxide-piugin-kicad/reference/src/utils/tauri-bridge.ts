import { invoke as tauriInvoke } from '@tauri-apps/api/core';

// This wrapper allows the UI to run in a web browser for previews
// while attempting to use Tauri functions if available.
export async function invoke<T>(cmd: string, args?: Record<string, any>): Promise<T> {
  // @ts-ignore
  if (window.__TAURI__ || window.__TAURI_INTERNALS__) {
    try {
      return await tauriInvoke<T>(cmd, args);
    } catch (err) {
      console.error(`Tauri invoke error [${cmd}]:`, err);
      throw err;
    }
  }

  console.log(`[MOCK TAURI] Triggered cmd: ${cmd}`, args);
  
  // Return mocked data based on the command for web preview functionality
  switch (cmd) {
    case 'kicad_load_board':
      return {
        board_name: 'test_board.kicad_pcb',
        board_bounds: { min_x: 0, min_y: 0, max_x: 100, max_y: 100 },
        footprints: [
          { reference: 'U1', value: 'MCU', x: 50, y: 50, orientation: 0, layer: 'F.Cu', pads: [
             { name: '1', net: 'GND', x: 45, y: 45 },
             { name: '2', net: 'VCC', x: 45, y: 55 },
             { name: '3', net: 'SIG', x: 55, y: 45 },
             { name: '4', net: 'CLK', x: 55, y: 55 }
          ] },
          { reference: 'C1', value: '100nF', x: 30, y: 30, orientation: 90, layer: 'F.Cu', pads: [
             { name: '1', net: 'GND', x: 30, y: 28 },
             { name: '2', net: 'VCC', x: 30, y: 32 }
          ] },
          { reference: 'R1', value: '10k', x: 70, y: 70, orientation: 0, layer: 'F.Cu', pads: [
             { name: '1', net: 'SIG', x: 68, y: 70 },
             { name: '2', net: 'GND', x: 72, y: 70 }
          ] }
        ],
        traces: [
          { start_x: 45, start_y: 45, end_x: 30, end_y: 28, net: 'GND', width: 0.5, layer: 'F.Cu' },
          { start_x: 45, start_y: 55, end_x: 30, end_y: 32, net: 'VCC', width: 0.5, layer: 'F.Cu' },
          { start_x: 55, start_y: 45, end_x: 68, end_y: 70, net: 'SIG', width: 0.25, layer: 'F.Cu' }
        ]
      } as any;
      
    case 'cmd_read_file':
      if (args?.path?.endsWith('.kicad_sch')) {
        return JSON.stringify({
          components: [
            { reference: 'U1', value: 'MCU', type: 'Microcontroller' },
            { reference: 'C1', value: '100nF', type: 'Capacitor' },
            { reference: 'R1', value: '10k', type: 'Resistor' }
          ],
          nets: [
            { name: 'GND', connections: [{ source: 'U1', target: 'C1' }, { source: 'U1', target: 'R1' }] },
            { name: 'VCC', connections: [{ source: 'U1', target: 'C1' }] },
            { name: 'SIG', connections: [{ source: 'U1', target: 'R1' }] }
          ]
        }) as any;
      }
      return '' as any;
      
    default:
      throw new Error(`Mock command not implemented: ${cmd}`);
  }
}
