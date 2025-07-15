import { useEffect, useRef, useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { Button } from "./ui/button";
import { Slider } from "./ui/slider";
import { Label } from "./ui/label";
import { Play, Pause, Square, RotateCcw } from "lucide-react";

interface City {
  id: number;
  x: number;
  y: number;
}

export default function ACOVisualizer() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [cities, setCities] = useState<City[]>([]);
  const [isRunning, setIsRunning] = useState(false);
  const [numAnts, setNumAnts] = useState([50]);
  const [evaporationRate, setEvaporationRate] = useState([0.1]);
  const [alpha, setAlpha] = useState([1.0]);
  const [beta, setBeta] = useState([2.0]);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    canvas.width = 800;
    canvas.height = 600;

    const drawCanvas = () => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      
      ctx.fillStyle = "#1f2937";
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      cities.forEach((city) => {
        ctx.beginPath();
        ctx.arc(city.x, city.y, 8, 0, 2 * Math.PI);
        ctx.fillStyle = "#3b82f6";
        ctx.fill();
        ctx.strokeStyle = "#ffffff";
        ctx.lineWidth = 2;
        ctx.stroke();

        ctx.fillStyle = "#ffffff";
        ctx.font = "12px Arial";
        ctx.textAlign = "center";
        ctx.fillText(city.id.toString(), city.x, city.y + 4);
      });
    };

    drawCanvas();
  }, [cities]);

  const handleCanvasClick = (event: React.MouseEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    const newCity: City = {
      id: cities.length,
      x,
      y,
    };

    setCities((prev) => [...prev, newCity]);
  };

  const handleStart = () => {
    if (cities.length < 3) {
      alert("Please add at least 3 cities");
      return;
    }
    setIsRunning(true);
  };

  const handleStop = () => {
    setIsRunning(false);
  };

  const handleReset = () => {
    setIsRunning(false);
    setCities([]);
  };

  const generateRandomCities = () => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const newCities: City[] = [];
    for (let i = 0; i < 10; i++) {
      newCities.push({
        id: i,
        x: Math.random() * (canvas.width - 40) + 20,
        y: Math.random() * (canvas.height - 40) + 20,
      });
    }
    setCities(newCities);
  };

  return (
    <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
      <div className="lg:col-span-3">
        <Card>
          <CardHeader>
            <CardTitle>Visualization Canvas</CardTitle>
          </CardHeader>
          <CardContent>
            <canvas
              ref={canvasRef}
              onClick={handleCanvasClick}
              className="border border-border rounded-lg cursor-crosshair w-full"
              style={{ maxWidth: "100%", height: "auto" }}
            />
            <p className="text-sm text-muted-foreground mt-2">
              Click on the canvas to add cities
            </p>
          </CardContent>
        </Card>
      </div>

      <div className="space-y-6">
        <Card>
          <CardHeader>
            <CardTitle>Controls</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex gap-2">
              <Button
                onClick={handleStart}
                disabled={isRunning || cities.length < 3}
                className="flex-1"
              >
                <Play className="w-4 h-4 mr-2" />
                Start
              </Button>
              <Button onClick={handleStop} disabled={!isRunning} className="flex-1">
                <Pause className="w-4 h-4 mr-2" />
                Stop
              </Button>
            </div>
            <div className="flex gap-2">
              <Button onClick={handleReset} variant="outline" className="flex-1">
                <RotateCcw className="w-4 h-4 mr-2" />
                Reset
              </Button>
              <Button onClick={generateRandomCities} variant="outline" className="flex-1">
                Random
              </Button>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Parameters</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div>
              <Label>Number of Ants: {numAnts[0]}</Label>
              <Slider
                value={numAnts}
                onValueChange={setNumAnts}
                min={10}
                max={200}
                step={10}
                className="mt-2"
              />
            </div>
            
            <div>
              <Label>Evaporation Rate: {evaporationRate[0].toFixed(2)}</Label>
              <Slider
                value={evaporationRate}
                onValueChange={setEvaporationRate}
                min={0.01}
                max={0.9}
                step={0.01}
                className="mt-2"
              />
            </div>
            
            <div>
              <Label>Alpha (α): {alpha[0].toFixed(1)}</Label>
              <Slider
                value={alpha}
                onValueChange={setAlpha}
                min={0.1}
                max={5.0}
                step={0.1}
                className="mt-2"
              />
            </div>
            
            <div>
              <Label>Beta (β): {beta[0].toFixed(1)}</Label>
              <Slider
                value={beta}
                onValueChange={setBeta}
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
            <CardTitle>Statistics</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-2 text-sm">
              <div className="flex justify-between">
                <span>Cities:</span>
                <span>{cities.length}</span>
              </div>
              <div className="flex justify-between">
                <span>Generation:</span>
                <span>0</span>
              </div>
              <div className="flex justify-between">
                <span>Best Distance:</span>
                <span>-</span>
              </div>
              <div className="flex justify-between">
                <span>Status:</span>
                <span>{isRunning ? "Running" : "Stopped"}</span>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}