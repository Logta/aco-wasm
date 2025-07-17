import { useCallback, useEffect, useRef, useState } from "react";

interface Stats {
  iteration: number;
  best_distance: number;
  cities_count: number;
  ants_count: number;
  state: string;
}

export function useEducationalACOGlobal() {
  const [stats, setStats] = useState<Stats | null>(null);
  const [isRunning, setIsRunning] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [isInitialized, setIsInitialized] = useState(false);
  const animationFrameRef = useRef<number>();

  const updateStats = useCallback(async () => {
    try {
      // Import the education wasm module
      const wasmModule = await import("../education-wasm/education_wasm.js");
      const newStats = wasmModule.get_stats();
      setStats(newStats);
    } catch (err) {
      console.error("Failed to get stats:", err);
    }
  }, []);

  const initializeWasm = useCallback(async () => {
    try {
      // Import the education wasm module
      const wasmModule = await import("../education-wasm/education_wasm.js");
      await wasmModule.default();

      await wasmModule.initialize_simulation("education-canvas");
      setIsInitialized(true);

      // Initial stats update
      updateStats();
    } catch (err) {
      console.error("Failed to initialize Educational WASM:", err);
      setError(`Failed to initialize: ${err}`);
    }
  }, [updateStats]);

  useEffect(() => {
    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, []);

  const animate = useCallback(async () => {
    if (!isInitialized || !isRunning) return;

    try {
      const wasmModule = await import("../education-wasm/education_wasm.js");
      
      // Step the ACO algorithm if we have enough cities and state is running
      if (stats && stats.cities_count >= 1 && stats.state === 'running') {
        wasmModule.step_simulation();
      }

      // Render the current state
      wasmModule.render_simulation();
      
      // Update stats less frequently to reduce WASM calls
      if (!animationFrameRef.current || animationFrameRef.current % 10 === 0) {
        updateStats();
      }

      if (isRunning) {
        animationFrameRef.current = requestAnimationFrame(animate);
      }
    } catch (err) {
      console.error("Animation error:", err);
      setIsRunning(false);
      setError(`Animation failed: ${err}`);
    }
  }, [isInitialized, isRunning, updateStats, stats]);

  useEffect(() => {
    if (isRunning && isInitialized) {
      animationFrameRef.current = requestAnimationFrame(animate);
    } else if (animationFrameRef.current) {
      cancelAnimationFrame(animationFrameRef.current);
    }

    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [isRunning, animate, isInitialized]);

  const start = useCallback(async () => {
    if (isInitialized && stats && stats.cities_count >= 1) {
      const wasmModule = await import("../education-wasm/education_wasm.js");
      wasmModule.start_simulation();
      setIsRunning(true);
    }
  }, [isInitialized, stats]);

  const pause = useCallback(async () => {
    if (isInitialized) {
      const wasmModule = await import("../education-wasm/education_wasm.js");
      wasmModule.pause_simulation();
      setIsRunning(false);
    }
  }, [isInitialized]);

  const reset = useCallback(async () => {
    if (isInitialized) {
      const wasmModule = await import("../education-wasm/education_wasm.js");
      wasmModule.reset_simulation();
      setIsRunning(false);
      updateStats();

      // Render after reset
      try {
        wasmModule.render_simulation();
      } catch (err) {
        console.error("Render error after reset:", err);
      }
    }
  }, [isInitialized, updateStats]);

  const addCity = useCallback(
    async (x: number, y: number) => {
      if (isInitialized) {
        const wasmModule = await import("../education-wasm/education_wasm.js");
        wasmModule.add_city(x, y);
        updateStats();

        // Render after adding city
        try {
          wasmModule.render_simulation();
        } catch (err) {
          console.error("Render error after adding city:", err);
        }
      }
    },
    [isInitialized, updateStats]
  );

  const removeCity = useCallback(
    async (x: number, y: number) => {
      if (isInitialized) {
        const wasmModule = await import("../education-wasm/education_wasm.js");
        wasmModule.remove_city(x, y);
        updateStats();

        // Render after removing city
        try {
          wasmModule.render_simulation();
        } catch (err) {
          console.error("Render error after removing city:", err);
        }
      }
    },
    [isInitialized, updateStats]
  );

  const clearCities = useCallback(async () => {
    if (isInitialized) {
      const wasmModule = await import("../education-wasm/education_wasm.js");
      wasmModule.clear_cities();
      setIsRunning(false);
      updateStats();

      // Render after clearing cities
      try {
        wasmModule.render_simulation();
      } catch (err) {
        console.error("Render error after clearing cities:", err);
      }
    }
  }, [isInitialized, updateStats]);

  return {
    stats,
    isRunning,
    start,
    pause,
    reset,
    addCity,
    removeCity,
    clearCities,
    error,
    initializeWasm,
    isInitialized,
  };
}