import { Link } from "@tanstack/react-router";
import ACOVisualizer from "../components/ACOVisualizer";
import { Button } from "../components/ui/button";

export default function TSPPage() {
  return (
    <div className="min-h-screen bg-background text-foreground">
      <header className="border-b border-border">
        <div className="container mx-auto px-4 py-4 flex justify-between items-center">
          <div>
            <h1 className="text-2xl font-bold">ACO TSP可視化ツール</h1>
            <p className="text-muted-foreground">巡回セールスマン問題の蟻コロニー最適化</p>
          </div>
          <Link to="/education">
            <Button variant="outline">教育モードに切り替え</Button>
          </Link>
        </div>
      </header>

      <main className="container mx-auto px-4 py-6">
        <ACOVisualizer />
      </main>
    </div>
  );
}
