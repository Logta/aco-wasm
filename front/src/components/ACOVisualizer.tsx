import { useEffect, useRef, useState, useCallback } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { Button } from "./ui/button";
import { Slider } from "./ui/slider";
import { Label } from "./ui/label";
import { Play, Pause, Square, RotateCcw, Settings } from "lucide-react";
import { useACOEngine } from "../hooks/useACOEngine";

export default function ACOVisualizer() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const animationFrameRef = useRef<number>();
  const { engine, isLoading, error } = useACOEngine();
  
  const [isRunning, setIsRunning] = useState(false);
  const [generation, setGeneration] = useState(0);
  const [bestDistance, setBestDistance] = useState<number | null>(null);
  const [cityCount, setCityCount] = useState(0);
  const [isInitialized, setIsInitialized] = useState(false);
  
  // Parameters
  const [numAnts, setNumAnts] = useState([50]);
  const [maxGenerations, setMaxGenerations] = useState([100]);
  const [evaporationRate, setEvaporationRate] = useState([0.1]);
  const [alpha, setAlpha] = useState([1.0]);
  const [beta, setBeta] = useState([2.0]);
  const [animationSpeed, setAnimationSpeed] = useState([1.0]);

  // Don't auto-initialize, let user control initialization

  const handleInitialize = () => {
    if (!engine || !canvasRef.current) {
      console.error("Engine or canvas not ready for initialization");
      return;
    }

    try {
      console.log("Initializing WebAssembly engine...");
      
      const canvas = canvasRef.current;
      
      // Set canvas internal size first, before any other operations
      canvas.width = 800;
      canvas.height = 600;
      
      console.log(`Canvas setup: ${canvas.width}x${canvas.height}`);
      
      // Initialize canvas renderer in WASM
      engine.initialize_canvas(canvas);
      engine.resize_canvas(800, 600);
      
      // Clear any existing state
      engine.clear_cities();
      
      // Use safe rendering
      engine.render();
      
      setIsInitialized(true);
      setCityCount(0);
      setGeneration(0);
      setBestDistance(null);
      setIsRunning(false);
      
      console.log("WebAssembly engine initialized successfully");
    } catch (err) {
      console.error("Failed to initialize WebAssembly engine:", err);
      setIsInitialized(false);
    }
  };

  // Animation loop
  const animate = useCallback((timestamp: number) => {
    if (!engine || !isRunning) return;

    // Update animation
    engine.update_animation(timestamp);
    
    // Run algorithm iteration
    if (engine.run_iteration()) {
      setGeneration(engine.get_generation());
      setBestDistance(engine.get_best_distance());
    } else {
      setIsRunning(false);
    }
    
    // Render
    engine.render();
    
    if (isRunning) {
      animationFrameRef.current = requestAnimationFrame(animate);
    }
  }, [engine, isRunning]);

  // Start/stop animation loop
  useEffect(() => {
    if (isRunning) {
      animationFrameRef.current = requestAnimationFrame(animate);
    } else {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    }

    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [isRunning, animate]);

  // Update city count when engine changes
  useEffect(() => {
    if (engine) {
      setCityCount(engine.get_city_count());
    }
  }, [engine]);

  const handleCanvasClick = (event: React.MouseEvent<HTMLCanvasElement>) => {
    if (!engine || isRunning) return;
    
    if (!isInitialized) {
      alert("まず初期化ボタンを押してください");
      return;
    }

    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    
    // Get click position relative to the display canvas
    const displayX = event.clientX - rect.left;
    const displayY = event.clientY - rect.top;
    
    // Convert to canvas internal coordinates (800x600 logical coordinates)
    const logicalX = (displayX / rect.width) * 800;
    const logicalY = (displayY / rect.height) * 600;

    console.log(`Click: display(${displayX.toFixed(1)}, ${displayY.toFixed(1)}), logical(${logicalX.toFixed(1)}, ${logicalY.toFixed(1)})`);

    try {
      engine.add_city(logicalX, logicalY);
      setCityCount(engine.get_city_count());
    } catch (err) {
      console.error("Error adding city:", err);
    }
  };

  const handleStart = () => {
    if (!isInitialized) {
      alert("まず初期化ボタンを押してください");
      return;
    }
    
    if (!engine || cityCount < 3) {
      alert("少なくとも3つの都市を追加してください");
      return;
    }

    engine.initialize_colony(
      numAnts[0],
      maxGenerations[0],
      evaporationRate[0],
      alpha[0],
      beta[0],
    );
    
    engine.set_animation_speed(animationSpeed[0]);
    engine.start();
    setIsRunning(true);
    setGeneration(0);
    setBestDistance(null);
  };

  const handleStop = () => {
    if (!engine) return;
    
    engine.stop();
    setIsRunning(false);
  };

  const handleReset = () => {
    if (!engine) return;
    
    try {
      // Don't call stop() if not running
      if (isRunning) {
        engine.stop();
      }
      engine.clear_cities();
      setIsRunning(false);
      setGeneration(0);
      setBestDistance(null);
      setCityCount(0);
      // Don't call render here - clear_cities will handle it safely
    } catch (err) {
      console.error("Error during reset:", err);
      // Try to recover by just updating the UI state
      setIsRunning(false);
      setGeneration(0);
      setBestDistance(null);
      setCityCount(0);
    }
  };

  const generateRandomCities = () => {
    if (!isInitialized) {
      alert("まず初期化ボタンを押してください");
      return;
    }
    
    if (!engine || isRunning) return;

    try {
      // Stop any running animation first only if running
      if (isRunning) {
        engine.stop();
        setIsRunning(false);
      }
      
      // Clear cities
      engine.clear_cities();
      
      // Add random cities using canvas internal coordinates
      for (let i = 0; i < 10; i++) {
        const x = Math.random() * 760 + 20; // 20px margin from 800px width
        const y = Math.random() * 560 + 20; // 20px margin from 600px height
        engine.add_city(x, y);
      }
      
      // Update state
      setCityCount(engine.get_city_count());
      setGeneration(0);
      setBestDistance(null);
    } catch (err) {
      console.error("Error generating random cities:", err);
      // Reset UI state on error
      setCityCount(0);
      setGeneration(0);
      setBestDistance(null);
    }
  };

  const handleAnimationSpeedChange = (newSpeed: number[]) => {
    setAnimationSpeed(newSpeed);
    if (engine) {
      engine.set_animation_speed(newSpeed[0]);
    }
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-lg">WebAssemblyモジュールを読み込み中...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-lg text-red-500">エラー: {error}</div>
      </div>
    );
  }

  return (
    <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
      <div className="lg:col-span-3">
        <Card>
          <CardHeader>
            <CardTitle>ビジュアライゼーション</CardTitle>
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
                ? "キャンバスをクリックして都市を追加" 
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
                  onClick={handleInitialize}
                  disabled={!engine || isLoading}
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
                onClick={handleStart}
                disabled={!isInitialized || isRunning || cityCount < 3}
                className="flex-1"
              >
                <Play className="w-4 h-4 mr-2" />
                開始
              </Button>
              <Button onClick={handleStop} disabled={!isInitialized || !isRunning} className="flex-1">
                <Pause className="w-4 h-4 mr-2" />
                停止
              </Button>
            </div>
            <div className="flex gap-2">
              <Button onClick={handleReset} disabled={!isInitialized || isRunning} variant="outline" className="flex-1">
                <RotateCcw className="w-4 h-4 mr-2" />
                リセット
              </Button>
              <Button onClick={generateRandomCities} disabled={!isInitialized || isRunning} variant="outline" className="flex-1">
                ランダム
              </Button>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>アルゴリズムパラメータ</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div>
              <Label>アリの数: {numAnts[0]}</Label>
              <Slider
                value={numAnts}
                onValueChange={setNumAnts}
                min={10}
                max={200}
                step={10}
                className="mt-2"
                disabled={isRunning}
              />
            </div>
            
            <div>
              <Label>最大世代数: {maxGenerations[0]}</Label>
              <Slider
                value={maxGenerations}
                onValueChange={setMaxGenerations}
                min={10}
                max={1000}
                step={10}
                className="mt-2"
                disabled={isRunning}
              />
            </div>
            
            <div>
              <Label>蒸発率: {evaporationRate[0].toFixed(2)}</Label>
              <Slider
                value={evaporationRate}
                onValueChange={setEvaporationRate}
                min={0.01}
                max={0.9}
                step={0.01}
                className="mt-2"
                disabled={isRunning}
              />
            </div>
            
            <div>
              <Label>アルファ (α): {alpha[0].toFixed(1)}</Label>
              <Slider
                value={alpha}
                onValueChange={setAlpha}
                min={0.1}
                max={5.0}
                step={0.1}
                className="mt-2"
                disabled={isRunning}
              />
            </div>
            
            <div>
              <Label>ベータ (β): {beta[0].toFixed(1)}</Label>
              <Slider
                value={beta}
                onValueChange={setBeta}
                min={0.1}
                max={5.0}
                step={0.1}
                className="mt-2"
                disabled={isRunning}
              />
            </div>
            
            <div>
              <Label>アニメーション速度: {animationSpeed[0].toFixed(1)}倍</Label>
              <Slider
                value={animationSpeed}
                onValueChange={handleAnimationSpeedChange}
                min={0.1}
                max={5.0}
                step={0.1}
                className="mt-2"
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
                <span>都市数:</span>
                <span>{cityCount}</span>
              </div>
              <div className="flex justify-between">
                <span>世代:</span>
                <span>{generation}</span>
              </div>
              <div className="flex justify-between">
                <span>最短距離:</span>
                <span>{bestDistance ? bestDistance.toFixed(2) : "-"}</span>
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
  );
}