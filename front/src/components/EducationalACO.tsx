import { Eye, EyeOff, Pause, Play, RotateCcw, Settings, ToggleLeft, ToggleRight } from "lucide-react";
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
  const [torusMode, setTorusMode] = useState(false);
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

  const generateRandomCities = () => {
    if (!isInitialized) {
      alert("まず初期化ボタンを押してください");
      return;
    }

    if (isRunning) {
      alert("シミュレーションを停止してください");
      return;
    }

    try {
      // Clear existing food sources
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
          let x: number, y: number, distanceFromNest: number;

          // Keep generating positions until we find one far enough from the nest
          do {
            x = margin + Math.random() * (800 - 2 * margin);
            y = margin + Math.random() * (600 - 2 * margin);
            distanceFromNest = Math.sqrt((x - nestX) ** 2 + (y - nestY) ** 2);
          } while (distanceFromNest < minDistanceFromNest);

          addCity(x, y);
        }
      }, 50);
    } catch (err) {
      console.error("Error generating random food sources:", err);
    }
  };

  useEffect(() => {
    if (canvasRef.current && !canvasReady) {
      const canvas = canvasRef.current;
      canvas.id = "education-canvas";
      canvas.width = 800;
      canvas.height = 600;
      setCanvasReady(true);

      // Initialize WASM after canvas is ready
      if (initializeWasm) {
        initializeWasm();
      }
    }
  }, [canvasReady, initializeWasm]);

  const handleCanvasClick = (event: React.MouseEvent<HTMLCanvasElement>) => {
    if (!canvasRef.current || isRunning) return;

    if (!isInitialized) {
      alert("まず初期化ボタンを押してください");
      return;
    }

    const rect = canvasRef.current.getBoundingClientRect();

    // Get click position relative to the display canvas
    const displayX = event.clientX - rect.left;
    const displayY = event.clientY - rect.top;

    // Convert to canvas internal coordinates (800x600 logical coordinates)
    const logicalX = (displayX / rect.width) * 800;
    const logicalY = (displayY / rect.height) * 600;

    try {
      addCity(logicalX, logicalY);
    } catch (err) {
      console.error("Error adding food source:", err);
    }
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

  const toggleTorusMode = async () => {
    const newTorusMode = !torusMode;
    setTorusMode(newTorusMode);
    if (isInitialized) {
      const wasmModule = await import("../education-wasm/education_wasm.js");
      wasmModule.set_torus_mode(newTorusMode);
    }
  };

  if (error) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-lg text-red-500">エラー: {error}</div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50 p-4">
      <div className="max-w-7xl mx-auto">
        <h1 className="text-3xl font-bold text-center mb-8">蟻の餌探索シミュレーション</h1>

        <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
          <div className="lg:col-span-3">
            <Card>
              <CardHeader>
                <CardTitle>餌探索シミュレーション</CardTitle>
              </CardHeader>
              <CardContent>
                <canvas
                  ref={canvasRef}
                  width={800}
                  height={600}
                  onClick={handleCanvasClick}
                  className="border border-border rounded-lg cursor-crosshair w-full"
                  style={{ maxWidth: "100%", height: "auto" }}
                />
                <p className="text-sm text-muted-foreground mt-2">
                  {isInitialized
                    ? "キャンバスをクリックして餌場を追加"
                    : "まず初期化ボタンを押してください"}
                </p>
              </CardContent>
            </Card>
          </div>

          <div className="space-y-6">
            <Card>
              <CardHeader>
                <CardTitle>コントロール</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                {!isInitialized && (
                  <div className="mb-4 p-4 bg-yellow-100 border border-yellow-400 rounded-lg">
                    <p className="text-sm text-yellow-800 mb-2">
                      WebAssemblyエンジンの初期化が必要です
                    </p>
                    <Button
                      onClick={initializeWasm}
                      disabled={!initializeWasm}
                      className="w-full"
                      variant="default"
                    >
                      <Settings className="w-4 h-4 mr-2" />
                      初期化
                    </Button>
                  </div>
                )}

                <div className="flex gap-2">
                  <Button
                    onClick={start}
                    disabled={!isInitialized || isRunning || (stats?.cities_count || 0) < 1}
                    className="flex-1"
                  >
                    <Play className="w-4 h-4 mr-2" />
                    開始
                  </Button>
                  <Button
                    onClick={pause}
                    disabled={!isInitialized || !isRunning}
                    className="flex-1"
                  >
                    <Pause className="w-4 h-4 mr-2" />
                    一時停止
                  </Button>
                </div>
                <div className="flex gap-2">
                  <Button
                    onClick={reset}
                    disabled={!isInitialized || isRunning}
                    variant="outline"
                    className="flex-1"
                  >
                    <RotateCcw className="w-4 h-4 mr-2" />
                    リセット
                  </Button>
                  <Button
                    onClick={generateRandomCities}
                    disabled={!isInitialized || isRunning}
                    variant="outline"
                    className="flex-1"
                  >
                    ランダム
                  </Button>
                </div>
                <Button
                  onClick={clearCities}
                  disabled={!isInitialized || isRunning}
                  variant="destructive"
                  className="w-full"
                >
                  全餌場削除
                </Button>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle>表示設定</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div>
                  <Label>アニメーション速度: {animationSpeed[0].toFixed(1)}倍</Label>
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
                    {showTrails ? (
                      <Eye className="w-4 h-4 mr-2" />
                    ) : (
                      <EyeOff className="w-4 h-4 mr-2" />
                    )}
                    蟻の軌跡を{showTrails ? "隠す" : "表示"}
                  </Button>
                  <Button
                    onClick={togglePheromones}
                    variant={showPheromones ? "default" : "outline"}
                    size="sm"
                    className="w-full"
                  >
                    {showPheromones ? (
                      <Eye className="w-4 h-4 mr-2" />
                    ) : (
                      <EyeOff className="w-4 h-4 mr-2" />
                    )}
                    フェロモンを{showPheromones ? "隠す" : "表示"}
                  </Button>
                  <Button
                    onClick={toggleTorusMode}
                    variant={torusMode ? "default" : "outline"}
                    size="sm"
                    className="w-full"
                  >
                    {torusMode ? (
                      <ToggleRight className="w-4 h-4 mr-2" />
                    ) : (
                      <ToggleLeft className="w-4 h-4 mr-2" />
                    )}
                    トーラス平面{torusMode ? "無効" : "有効"}
                  </Button>
                </div>
              </CardContent>
            </Card>

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
                    disabled={isRunning}
                  />
                  <p className="text-xs text-muted-foreground mt-1">フェロモンの重要度</p>
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
                    disabled={isRunning}
                  />
                  <p className="text-xs text-muted-foreground mt-1">距離の重要度</p>
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
                    disabled={isRunning}
                  />
                  <p className="text-xs text-muted-foreground mt-1">フェロモンの蒸発率</p>
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
                    disabled={isRunning}
                  />
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle>統計</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-2 text-sm">
                  <div className="flex justify-between">
                    <span>初期化:</span>
                    <span className={isInitialized ? "text-green-600" : "text-red-600"}>
                      {isInitialized ? "完了" : "未完了"}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span>餌場数:</span>
                    <span>{stats?.cities_count || 0}</span>
                  </div>
                  <div className="flex justify-between">
                    <span>蟻数:</span>
                    <span>{stats?.ants_count || 0}</span>
                  </div>
                  <div className="flex justify-between">
                    <span>集めた餌の量:</span>
                    <span>{(stats?.best_distance || 0).toFixed(1)} 単位</span>
                  </div>
                  <div className="flex justify-between">
                    <span>ステータス:</span>
                    <span className={isRunning ? "text-green-600" : "text-gray-600"}>
                      {isRunning ? "実行中" : "停止中"}
                    </span>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>
        </div>
      </div>
    </div>
  );
}
