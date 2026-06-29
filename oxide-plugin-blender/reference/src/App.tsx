import React from 'react';
import { MainLayout } from './components/layout/MainLayout';
import { useBlenderSocket } from './hooks/useBlenderSocket';

export default function App() {
  // Initialize mock connection
  useBlenderSocket();
  
  return (
    <div className="w-full h-screen">
      <MainLayout />
    </div>
  );
}
