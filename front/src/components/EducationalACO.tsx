import type React from "react";
import { useEffect, useRef, useState } from "react";
import { useEducationalACOGlobal } from "../hooks/useEducationalACOGlobal";
import { Button } from "./ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { Label } from "./ui/label";
import { Slider } from "./ui/slider";

export default function EducationalACO() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [canvasReady, setCanvasReady] = useState(false);
  const [animationSpeed, setAnimationSpeed] = useState([1]);
  const [showTrails, setShowTrails] = useState(true);
  const [showPheromones, setShowPheromones] = useState(true);
  const [alpha, setAlpha] = useState([1]);
  const [beta, setBeta] = useState([2]);
  const [evaporation, setEvaporation] = useState([0.1]);
  const [numAnts, setNumAnts] = useState([20]);

  const {
    stats,
    isRunning,
    start,
    pause,
    reset,
    addCity,
    clearCities,
    error,
    initializeWasm,
    isInitialized,
  } = useEducationalACOGlobal();

  const setPresetCities = () => {
    if (isInitialized) {
      // Use clearCities from the hook instead of calling WASM directly
      clearCities();

      // Wait a bit for the clear to complete, then add food sources
      setTimeout(() => {
        // Add random food sources around the canvas
        const nestX = 400;
        const nestY = 300;
        const minDistanceFromNest = 100; // Minimum distance from nest
        const margin = 50; // Margin from canvas edges
        
        // Generate 4-8 random food sources
        const numFoodSources = Math.floor(Math.random() * 5) + 4;
        
        for (let i = 0; i < numFoodSources; i++) {
          let x, y, distanceFromNest;
          
          // Keep generating positions until we find one far enough from the nest
          do {
            x = margin + Math.random() * (800 - 2 * margin);
            y = margin + Math.random() * (600 - 2 * margin);
            distanceFromNest = Math.sqrt((x - nestX) ** 2 + (y - nestY) ** 2);
          } while (distanceFromNest < minDistanceFromNest);
          
          addCity(x, y);
        }
      }, 50);
    }
  };

  useEffect(() => {
    if (canvasRef.current && !canvasReady) {
      const canvas = canvasRef.current;
      canvas.id = "education-canvas";
      setCanvasReady(true);

      // Initialize WASM after canvas is ready
      if (initializeWasm) {
        initializeWasm();
      }
    }
  }, [canvasReady, initializeWasm]);

  const handleCanvasClick = (event: React.MouseEvent<HTMLCanvasElement>) => {
    if (!canvasRef.current) return;

    const rect = canvasRef.current.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    addCity(x, y);
  };

  const updateAnimationSpeed = async (value: number[]) => {
    setAnimationSpeed(value);
    if (isInitialized) {
      const wasmModule = await import("../education-wasm/education_wasm.js");
      wasmModule.set_animation_speed(value[0]);
    }
  };

  const updateAlpha = async (value: number[]) => {
    setAlpha(value);
    if (isInitialized) {
      const wasmModule = await import("../education-wasm/education_wasm.js");
      wasmModule.set_aco_param("alpha", value[0]);
    }
  };

  const updateBeta = async (value: number[]) => {
    setBeta(value);
    if (isInitialized) {
      const wasmModule = await import("../education-wasm/education_wasm.js");
      wasmModule.set_aco_param("beta", value[0]);
    }
  };

  const updateEvaporation = async (value: number[]) => {
    setEvaporation(value);
    if (isInitialized) {
      const wasmModule = await import("../education-wasm/education_wasm.js");
      wasmModule.set_aco_param("evaporation", value[0]);
    }
  };

  const updateNumAnts = async (value: number[]) => {
    setNumAnts(value);
    if (isInitialized) {
      const wasmModule = await import("../education-wasm/education_wasm.js");
      wasmModule.set_aco_param("num_ants", value[0]);
    }
  };

  const toggleTrails = async () => {
    const newShowTrails = !showTrails;
    setShowTrails(newShowTrails);
    if (isInitialized) {
      const wasmModule = await import("../education-wasm/education_wasm.js");
      wasmModule.set_show_ant_trails(newShowTrails);
    }
  };

  const togglePheromones = async () => {
    const newShowPheromones = !showPheromones;
    setShowPheromones(newShowPheromones);
    if (isInitialized) {
      const wasmModule = await import("../education-wasm/education_wasm.js");
      wasmModule.set_show_pheromone_levels(newShowPheromones);
    }
  };


  if (error) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen p-4">
        <Card className="max-w-md">
          <CardHeader>
            <CardTitle className="text-red-600">エラー</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-gray-600">{error}</p>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-100 p-4">
      <div className="max-w-7xl mx-auto">
        <h1 className="text-3xl font-bold text-center mb-8">蟻の餌探索シミュレーション</h1>

        <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
          {/* Canvas */}
          <div className="lg:col-span-3">
            <Card>
              <CardHeader>
                <CardTitle>餌探索シミュレーション</CardTitle>
                <p className="text-sm text-gray-600">
                  キャンバスをクリックして餌場を追加してください。蟻が巣から餌を運ぶ様子を観察できます！
                </p>
              </CardHeader>
              <CardContent>
                <canvas
                  ref={canvasRef}
                  width={800}
                  height={600}
                  className="border border-gray-300 rounded cursor-crosshair bg-gray-900"
                  onClick={handleCanvasClick}
                />

                <div className="flex gap-2 mt-4 flex-wrap">
                  <Button
                    onClick={start}
                    disabled={!isInitialized || isRunning || (stats?.cities_count || 0) < 1}
                    className="bg-green-600 hover:bg-green-700"
                  >
                    開始
                  </Button>
                  <Button
                    onClick={pause}
                    disabled={!isInitialized || !isRunning}
                    className="bg-yellow-600 hover:bg-yellow-700"
                  >
                    一時停止
                  </Button>
                  <Button onClick={reset} disabled={!isInitialized} className="bg-blue-600 hover:bg-blue-700">
                    リセット
                  </Button>
                  <Button
                    onClick={setPresetCities}
                    disabled={!isInitialized}
                    className="bg-purple-600 hover:bg-purple-700"
                  >
                    初期配置
                  </Button>
                  <Button onClick={clearCities} disabled={!isInitialized} variant="destructive">
                    全餌場削除
                  </Button>
                </div>
              </CardContent>
            </Card>
          </div>

          {/* Controls */}
          <div className="space-y-6">
            {/* Statistics */}
            <Card>
              <CardHeader>
                <CardTitle>統計情報</CardTitle>
              </CardHeader>
              <CardContent className="space-y-2">
                <div className="text-sm">
                  <div>餌場数: {stats?.cities_count || 0}</div>
                  <div>蟻数: {stats?.ants_count || 0}</div>
                  <div>
                    集めた餌の量:{" "}
                    {(stats?.best_distance || 0).toFixed(1)} 単位
                  </div>
                  <div>
                    状態:{" "}
                    {stats?.state === "idle"
                      ? "待機中"
                      : stats?.state === "running"
                        ? "実行中"
                        : stats?.state === "paused"
                          ? "一時停止中"
                          : stats?.state || "不明"}
                  </div>
                </div>
              </CardContent>
            </Card>

            {/* Animation Controls */}
            <Card>
              <CardHeader>
                <CardTitle>アニメーション</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div>
                  <Label>速度: {animationSpeed[0].toFixed(1)}x</Label>
                  <Slider
                    value={animationSpeed}
                    onValueChange={updateAnimationSpeed}
                    min={0.1}
                    max={5}
                    step={0.1}
                    className="mt-2"
                  />
                </div>

                <div className="space-y-2">
                  <Button
                    onClick={toggleTrails}
                    variant={showTrails ? "default" : "outline"}
                    size="sm"
                    className="w-full"
                  >
                    蟻の軌跡を{showTrails ? "隠す" : "表示"}
                  </Button>
                  <Button
                    onClick={togglePheromones}
                    variant={showPheromones ? "default" : "outline"}
                    size="sm"
                    className="w-full"
                  >
                    フェロモン濃度を{showPheromones ? "隠す" : "表示"}
                  </Button>
                </div>
              </CardContent>
            </Card>

            {/* ACO Parameters */}
            <Card>
              <CardHeader>
                <CardTitle>ACOパラメータ</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div>
                  <Label>アルファ (α): {alpha[0].toFixed(1)}</Label>
                  <Slider
                    value={alpha}
                    onValueChange={updateAlpha}
                    min={0.1}
                    max={5}
                    step={0.1}
                    className="mt-2"
                  />
                  <p className="text-xs text-gray-600 mt-1">フェロモンの重要度</p>
                </div>

                <div>
                  <Label>ベータ (β): {beta[0].toFixed(1)}</Label>
                  <Slider
                    value={beta}
                    onValueChange={updateBeta}
                    min={0.1}
                    max={5}
                    step={0.1}
                    className="mt-2"
                  />
                  <p className="text-xs text-gray-600 mt-1">距離の重要度</p>
                </div>

                <div>
                  <Label>蒸発率: {evaporation[0].toFixed(2)}</Label>
                  <Slider
                    value={evaporation}
                    onValueChange={updateEvaporation}
                    min={0.01}
                    max={0.5}
                    step={0.01}
                    className="mt-2"
                  />
                  <p className="text-xs text-gray-600 mt-1">フェロモンの蒸発率</p>
                </div>

                <div>
                  <Label>蟻の数: {numAnts[0]}</Label>
                  <Slider
                    value={numAnts}
                    onValueChange={updateNumAnts}
                    min={5}
                    max={50}
                    step={1}
                    className="mt-2"
                  />
                </div>
              </CardContent>
            </Card>
          </div>
        </div>
      </div>
    </div>
  );
}
