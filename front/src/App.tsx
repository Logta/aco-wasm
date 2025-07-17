import { useState } from "react";
import ACOVisualizer from "./components/ACOVisualizer";
import EducationalACO from "./components/EducationalACO";
import { Button } from "./components/ui/button";

type PageType = "tsp" | "education";

function App() {
  const [currentPage, setCurrentPage] = useState<PageType>("tsp");

  if (currentPage === "education") {
    return (
      <div className="min-h-screen bg-background text-foreground">
        <header className="border-b border-border">
          <div className="container mx-auto px-4 py-4 flex justify-between items-center">
            <div>
              <h1 className="text-2xl font-bold">教育用ACO可視化ツール</h1>
              <p className="text-muted-foreground">蟻コロニー最適化の動作を段階的に学習できます</p>
            </div>
            <Button onClick={() => setCurrentPage("tsp")} variant="outline">
              TSP解決モードに切り替え
            </Button>
          </div>
        </header>

        <main>
          <EducationalACO />
        </main>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-background text-foreground">
      <header className="border-b border-border">
        <div className="container mx-auto px-4 py-4 flex justify-between items-center">
          <div>
            <h1 className="text-2xl font-bold">ACO TSP可視化ツール</h1>
            <p className="text-muted-foreground">巡回セールスマン問題の蟻コロニー最適化</p>
          </div>
          <Button onClick={() => setCurrentPage("education")} variant="outline">
            教育モードに切り替え
          </Button>
        </div>
      </header>

      <main className="container mx-auto px-4 py-6">
        <ACOVisualizer />
      </main>
    </div>
  );
}

export default App;
