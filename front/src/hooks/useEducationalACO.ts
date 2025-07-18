import { useCallback, useEffect, useRef, useState } from "react";

interface Stats {
  iteration: number;
  best_distance: number;
  cities_count: number;
  ants_count: number;
  state: string;
}

export function useEducationalACO() {
  const [aco, setAco] = useState<any>(null);
  const [stats, setStats] = useState<Stats | null>(null);
  const [isRunning, setIsRunning] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const animationFrameRef = useRef<number>();

  const updateStats = useCallback((acoInstance: any) => {
    if (acoInstance) {
      try {
        const newStats = acoInstance.get_stats();
        setStats(newStats);
      } catch (err) {
        console.error("Failed to get stats:", err);
      }
    }
  }, []);

  const initializeWasm = useCallback(async () => {
    try {
      // Import the education wasm module
      const wasmModule = await import("../education-wasm/education_wasm.js");
      await wasmModule.default();

      const acoInstance = new wasmModule.SafeEducationalACO("education-canvas");
      setAco(acoInstance);

      // Initial stats update
      updateStats(acoInstance);
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

  const animate = useCallback(() => {
    if (!aco || !isRunning) return;

    try {
      // Step the ACO algorithm if we have enough cities and state is running
      if (stats && stats.cities_count >= 1 && stats.state === "running") {
        // Only call step every few frames to reduce WASM calls
        const frameSkip = 1; // Call step every frame
        if (!animationFrameRef.current || animationFrameRef.current % frameSkip === 0) {
          aco.step();
        }
      }

      // Render the current state
      aco.render();

      // Update stats less frequently to reduce WASM calls
      if (!animationFrameRef.current || animationFrameRef.current % 10 === 0) {
        updateStats(aco);
      }

      if (isRunning) {
        animationFrameRef.current = requestAnimationFrame(animate);
      }
    } catch (err) {
      console.error("Animation error:", err);
      setIsRunning(false);
      setError(`Animation failed: ${err}`);
    }
  }, [aco, isRunning, updateStats, stats]);

  useEffect(() => {
    if (isRunning && aco) {
      animationFrameRef.current = requestAnimationFrame(animate);
    } else if (animationFrameRef.current) {
      cancelAnimationFrame(animationFrameRef.current);
    }

    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [isRunning, animate, aco]);

  const start = useCallback(() => {
    if (aco && stats && stats.cities_count >= 1) {
      aco.start();
      setIsRunning(true);
    }
  }, [aco, stats]);

  const pause = useCallback(() => {
    if (aco) {
      aco.pause();
      setIsRunning(false);
    }
  }, [aco]);

  const reset = useCallback(() => {
    if (aco) {
      aco.reset();
      setIsRunning(false);
      updateStats(aco);

      // Render after reset
      try {
        aco.render();
      } catch (err) {
        console.error("Render error after reset:", err);
      }
    }
  }, [aco, updateStats]);

  const addCity = useCallback(
    (x: number, y: number) => {
      if (aco) {
        aco.add_city(x, y);
        updateStats(aco);

        // Render after adding city
        try {
          aco.render();
        } catch (err) {
          console.error("Render error after adding city:", err);
        }
      }
    },
    [aco, updateStats]
  );

  const removeCity = useCallback(
    (x: number, y: number) => {
      if (aco) {
        aco.remove_city(x, y);
        updateStats(aco);

        // Render after removing city
        try {
          aco.render();
        } catch (err) {
          console.error("Render error after removing city:", err);
        }
      }
    },
    [aco, updateStats]
  );

  const clearCities = useCallback(() => {
    if (aco) {
      aco.clear_cities();
      setIsRunning(false);
      updateStats(aco);

      // Render after clearing cities
      try {
        aco.render();
      } catch (err) {
        console.error("Render error after clearing cities:", err);
      }
    }
  }, [aco, updateStats]);

  return {
    aco,
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
  };
}
