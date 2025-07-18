import { Link } from "@tanstack/react-router";
import { Home } from "lucide-react";
import EducationalACO from "../components/EducationalACO";
import { Button } from "../components/ui/button";

export default function EducationPage() {
  return (
    <div className="min-h-screen bg-background text-foreground">
      <header className="border-b border-border">
        <div className="container mx-auto px-4 py-4 flex justify-between items-center">
          <div>
            <h1 className="text-2xl font-bold">教育用ACO可視化ツール</h1>
            <p className="text-muted-foreground">蟻コロニー最適化の動作を段階的に学習できます</p>
          </div>
          <div className="flex gap-2">
            <Link to="/">
              <Button variant="ghost" size="sm">
                <Home className="h-4 w-4 mr-2" />
                メインページ
              </Button>
            </Link>
            <Link to="/tsp">
              <Button variant="outline">TSP解決モードに切り替え</Button>
            </Link>
          </div>
        </div>
      </header>

      <main>
        <EducationalACO />
      </main>
    </div>
  );
}
