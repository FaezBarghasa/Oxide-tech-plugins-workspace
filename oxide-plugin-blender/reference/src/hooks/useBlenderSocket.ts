import { useEffect, useRef } from 'react';
import { io, Socket } from 'socket.io-client';
import { useBlenderStore } from '../state/blenderStore';
import { v4 as uuidv4 } from 'uuid';
import { BlenderResponse } from '../types/blender';

let globalSocket: Socket | null = null;

export function useBlenderSocket() {
  const { setConnectionStatus, setStatus, setProgress, setError } = useBlenderStore();

  useEffect(() => {
    // We are mocking the socket in the browser simulator.
    // In a real Tauri app, this would connect to the Python backend.
    if (!globalSocket) {
      // Mocking the connection for demonstration
      setConnectionStatus(true);
      globalSocket = {
        connected: true,
        // @ts-ignore
        emit: (event: string, payload: any) => {},
        // @ts-ignore
        on: (event: string, callback: any) => {},
        // @ts-ignore
        once: (event: string, callback: any) => {},
        // @ts-ignore
        off: (event: string, callback: any) => {},
      } as Socket;
    }

    return () => {
      // Do not disconnect global on unmount to keep it alive
    };
  }, [setConnectionStatus]);

  return { socket: globalSocket };
}

export async function sendBlenderRequest<T>(
  operation: string,
  payload: any,
  timeout = 30000
): Promise<T> {
  const requestId = uuidv4();
  // We don't have the real socket here, so we will MOCK the generation locally for preview
  const { setStatus, setProgress } = useBlenderStore.getState();
  
  return new Promise((resolve, reject) => {
    setStatus('generating');
    setProgress(0);
    
    // Simulate generation progress
    let iters = 0;
    const interval = setInterval(() => {
      iters++;
      setProgress(iters * 20);
      if (iters >= 5) {
        clearInterval(interval);
        setStatus('idle');
        setProgress(100);
        // Resolve with a mock response representing binary array
        const mockArray = new ArrayBuffer(0);
        resolve({ mesh_size_bytes: 0, mesh_buffer: mockArray } as unknown as T);
      }
    }, 500);

    /* Real implementation logic below:
    if (!socket?.connected) {
      throw new Error('Blender connection lost. Attempting reconnect...');
    }

    const timer = setTimeout(() => {
      socket.off(`response:${requestId}`);
      reject(new Error(`Blender operation '${operation}' exceeded ${timeout}ms`));
    }, timeout);

    socket.once(`response:${requestId}`, (response: BlenderResponse) => {
      clearTimeout(timer);
      socket.off(`response:${requestId}`);
      
      if (response.status === 'error') {
        reject(new Error(response.message || 'Blender Error'));
      } else {
        resolve(response.data as T);
      }
    });

    socket.emit(operation, { ...payload, request_id: requestId });
    */
  });
}
