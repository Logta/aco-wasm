import { useState } from "react";
import ACOVisualizer from "./components/ACOVisualizer";

function App() {
  return (
    <div className="min-h-screen bg-background text-foreground">
      <header className="border-b border-border">
        <div className="container mx-auto px-4 py-4">
          <h1 className="text-2xl font-bold">ACO TSP Visualizer</h1>
          <p className="text-muted-foreground">
            Ant Colony Optimization for Traveling Salesman Problem
          </p>
        </div>
      </header>
      
      <main className="container mx-auto px-4 py-6">
        <ACOVisualizer />
      </main>
    </div>
  );
}

export default App;