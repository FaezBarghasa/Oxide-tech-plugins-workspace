import React, { useEffect } from 'react';
import {
  ReactFlow,
  Controls,
  Background,
  useNodesState,
  useEdgesState,
  Node,
  Edge
} from 'reactflow';
import 'reactflow/dist/style.css';

import { useSchematicStore } from '../../state/schematicStore';
import { useSelectionStore } from '../../state/selectionStore';

function getComponentStyle(type: string, isSelected: boolean) {
  const base = {
    background: '#141417',
    color: '#f8fafc',
    border: isSelected ? '1px solid #14b8a6' : '1px solid rgba(255, 255, 255, 0.05)',
    borderRadius: '8px',
    padding: '10px 15px',
    boxShadow: isSelected ? '0 0 20px rgba(20, 184, 166, 0.15)' : 'none',
  };
  
  if (type === 'Microcontroller') {
     return { ...base, border: isSelected ? '1px solid #14b8a6' : '1px solid rgba(20, 184, 166, 0.3)', minWidth: '120px' };
  }
  return base;
}

export function SchematicViewer() {
  const { schematicData } = useSchematicStore();
  const { selectedReferences, selectComponent } = useSelectionStore();
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);

  useEffect(() => {
    if (!schematicData) return;

    const newNodes: Node[] = schematicData.components.map((comp, idx) => ({
      id: comp.reference,
      data: {
        label: (
          <div className="flex flex-col text-center">
            <span className="font-bold font-mono text-xs text-teal-400">{comp.reference}</span>
            <span className="text-[10px] text-slate-400">{comp.value}</span>
          </div>
        )
      },
      position: { x: idx * 250, y: (idx % 2) * 100 },
      style: getComponentStyle(comp.type, selectedReferences.has(comp.reference))
    }));

    const newEdges: Edge[] = schematicData.nets.flatMap(net =>
      net.connections.map((conn, idx) => ({
        id: `${net.name}-${conn.source}-${conn.target}-${idx}`,
        source: conn.source,
        target: conn.target,
        label: net.name,
        type: 'smoothstep',
        animated: selectedReferences.has(conn.source) || selectedReferences.has(conn.target),
        style: { stroke: '#14b8a6', strokeWidth: 2 },
        labelStyle: { fill: '#14b8a6', fontWeight: 600, fontSize: 10 },
        labelBgStyle: { fill: '#0F0F11', fillOpacity: 0.8 },
      }))
    );

    setNodes(newNodes);
    setEdges(newEdges);
  }, [schematicData]);

  // Sync selected visual state efficiently
  useEffect(() => {
    setNodes((nds) =>
      nds.map((n) => ({
        ...n,
        style: getComponentStyle(
            schematicData?.components.find(c => c.reference === n.id)?.type || '',
            selectedReferences.has(n.id)
        ),
      }))
    );
     setEdges((eds) => 
        eds.map(e => ({
           ...e,
           animated: selectedReferences.has(e.source) || selectedReferences.has(e.target)
        }))
     )
  }, [selectedReferences]);

  const onNodeClick = (_: React.MouseEvent, node: Node) => {
    selectComponent(node.id, false); // select and clear others
  };

  const onPaneClick = () => {
    useSelectionStore.getState().clearSelection();
  };

  if (!schematicData) {
     return <div className="flex h-full w-full items-center justify-center text-gray-500">No Schematic Loaded</div>;
  }

  return (
    <div className="h-full w-full bg-transparent">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onNodeClick={onNodeClick}
        onPaneClick={onPaneClick}
        fitView
      >
        <Controls />
        <Background color="rgba(255, 255, 255, 0.05)" gap={24} size={1} />
      </ReactFlow>
    </div>
  );
}
