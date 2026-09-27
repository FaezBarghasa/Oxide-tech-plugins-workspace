import React, { Suspense } from 'react';
import { Canvas } from '@react-three/fiber';
import { OrbitControls, Grid, Environment } from '@react-three/drei';
import { Lights } from './Lights';
import { EnclosureModel } from './EnclosureModel';

export function Scene() {
  return (
    <div className="w-full h-full bg-[#18181b] rounded-[2rem] overflow-hidden">
      <Canvas shadows camera={{ position: [20, 20, 20], fov: 45 }}>
        <color attach="background" args={['#18181b']} />
        
        <Suspense fallback={null}>
          <Lights />
          <Environment preset="city" />
          
          <EnclosureModel />
          
          <Grid 
            infiniteGrid 
            fadeDistance={50} 
            sectionColor="#27272a" 
            cellColor="#09090b" 
            cellSize={1} 
            sectionSize={5} 
          />
          
          <OrbitControls 
            makeDefault 
            minPolarAngle={0} 
            maxPolarAngle={Math.PI / 2 + 0.1}
            maxDistance={100}
            minDistance={2}
          />
        </Suspense>
      </Canvas>
    </div>
  );
}
