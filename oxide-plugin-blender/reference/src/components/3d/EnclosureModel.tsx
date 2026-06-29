import React from 'react';
import { useEnclosureStore } from '../../state/enclosureStore';
import { MATERIALS } from '../../types/materials';
import * as THREE from 'three';

export function EnclosureModel() {
  const { currentEnclosure } = useEnclosureStore();
  const { dimensions, wallThickness, material } = currentEnclosure;
  
  // Find color from materials list
  const matConfig = MATERIALS.find(m => m.id === material);
  const color = matConfig ? matConfig.color : '#cccccc';

  // Scale down dimensions for visualization (from mm to cm approx)
  const scale = 0.1;
  const w = dimensions.width * scale;
  const h = dimensions.height * scale;
  const d = dimensions.depth * scale;
  const t = wallThickness * scale;

  return (
    <group position={[0, h/2, 0]}>
      {/* Outer Shell */}
      <mesh castShadow receiveShadow>
        <boxGeometry args={[w, h, d]} />
        <meshStandardMaterial 
          color={color} 
          roughness={0.6}
          metalness={material === 'aluminum' ? 0.8 : 0.1}
          transparent={true}
          opacity={0.8} // Allow seeing inside
        />
      </mesh>
      
      {/* Inner Hollow representation (visual only for mockup) */}
      <mesh>
        <boxGeometry args={[w - t*2, h - t*2, d - t*2]} />
        <meshBasicMaterial color="#0f172a" wireframe opacity={0.2} transparent />
      </mesh>
      
      {/* Vents Mockup */}
      {currentEnclosure.ventConfig.quantity > 0 && Array.from({ length: currentEnclosure.ventConfig.quantity }).map((_, i) => {
        const span = (currentEnclosure.ventConfig.quantity * currentEnclosure.ventConfig.spacing * scale);
        const startX = -span / 2 + (currentEnclosure.ventConfig.spacing * scale) / 2;
        const xPos = startX + (i * currentEnclosure.ventConfig.spacing * scale);
        
        return (
          <mesh key={i} position={[xPos, h/2 + 0.1, 0]}>
            <cylinderGeometry args={[
              currentEnclosure.ventConfig.holeDiameter * scale / 2, 
              currentEnclosure.ventConfig.holeDiameter * scale / 2, 
              t * 2, 
              16
            ]} />
            <meshBasicMaterial color="#1e293b" />
          </mesh>
        );
      })}
    </group>
  );
}
