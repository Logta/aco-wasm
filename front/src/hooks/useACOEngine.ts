import { useEffect, useState } from "react";

interface ACOEngineInstance {
  add_city: (x: number, y: number) => number;
  clear_cities: () => void;
  get_city_count: () => number;
  initialize_canvas: (canvas: HTMLCanvasElement) => void;
  resize_canvas: (width: number, height: number) => void;
  initialize_colony: (
    num_ants: number,
    max_generations: number,
    evaporation_rate: number,
    alpha: number,
    beta: number
  ) => void;
  start: () => void;
  stop: () => void;
  run_iteration: () => boolean;
  render: () => void;
  update_animation: (timestamp: number) => boolean;
  set_animation_speed: (speed: number) => void;
  get_best_distance: () => number;
  get_generation: () => number;
  get_best_route: () => number[];
  is_complete: () => boolean;
  is_running: () => boolean;
}

export interface ACOEngineWasm {
  ACOEngine: new () => ACOEngineInstance;
  default?: () => Promise<void>;
}

export function useACOEngine() {
  const [wasmModule, setWasmModule] = useState<ACOEngineWasm | null>(null);
  const [engine, setEngine] = useState<ACOEngineInstance | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let isMounted = true;

    async function loadWasm() {
      try {
        // First, try to import the wasm module
        const wasm = await import("../wasm/aco_wasm.js");

        // Initialize the wasm module if needed
        if (wasm.default) {
          await wasm.default();
        }

        if (isMounted) {
          setWasmModule(wasm as ACOEngineWasm);
          const engineInstance = new wasm.ACOEngine();
          setEngine(engineInstance);
          setIsLoading(false);
        }
      } catch (err) {
        if (isMounted) {
          console.error("WebAssembly loading error:", err);
          setError(`Failed to load WebAssembly module: ${err}`);
          setIsLoading(false);
        }
      }
    }

    loadWasm();

    return () => {
      isMounted = false;
    };
  }, []);

  return {
    engine,
    wasmModule,
    isLoading,
    error,
  };
}
