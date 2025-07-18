import { Link } from "@tanstack/react-router";
import { GraduationCap, MapPin } from "lucide-react";
import { Button } from "../components/ui/button";

export default function MainPage() {
  return (
    <div className="min-h-screen bg-background text-foreground">
      <header className="border-b border-border">
        <div className="container mx-auto px-4 py-6">
          <div className="text-center">
            <h1 className="text-4xl font-bold mb-2">ACO 可視化ツール</h1>
            <p className="text-xl text-muted-foreground">
              蟻コロニー最適化アルゴリズムの世界へようこそ
            </p>
          </div>
        </div>
      </header>

      <main className="container mx-auto px-4 py-12">
        <div className="max-w-4xl mx-auto">
          <div className="text-center mb-12">
            <h2 className="text-2xl font-semibold mb-4">モードを選択してください</h2>
            <p className="text-lg text-muted-foreground">
              目的に応じて最適なモードを選んでACOアルゴリズムを体験しましょう
            </p>
          </div>

          <div className="grid md:grid-cols-2 gap-8">
            <div className="bg-card text-card-foreground p-6 rounded-lg border border-border">
              <div className="flex items-center mb-4">
                <MapPin className="h-8 w-8 text-primary mr-3" />
                <h3 className="text-xl font-semibold">TSP解決モード</h3>
              </div>
              <p className="text-muted-foreground mb-6">
                巡回セールスマン問題（TSP）を高速に解決するモードです。
                最適化されたACOアルゴリズムが実際の問題を効率的に解決する様子をリアルタイムで観察できます。
              </p>
              <div className="mb-6">
                <h4 className="font-medium mb-2">特徴:</h4>
                <ul className="text-sm text-muted-foreground space-y-1">
                  <li>• 高速な最適化アルゴリズム</li>
                  <li>• リアルタイムな結果表示</li>
                  <li>• パラメータ調整機能</li>
                  <li>• 複数都市での実用的な解決</li>
                </ul>
              </div>
              <Link to="/tsp">
                <Button className="w-full" size="lg">
                  TSP解決モードを開始
                </Button>
              </Link>
            </div>

            <div className="bg-card text-card-foreground p-6 rounded-lg border border-border">
              <div className="flex items-center mb-4">
                <GraduationCap className="h-8 w-8 text-primary mr-3" />
                <h3 className="text-xl font-semibold">教育モード</h3>
              </div>
              <p className="text-muted-foreground mb-6">
                ACOアルゴリズムの動作原理を段階的に学習できるモードです。
                蟻の行動、フェロモンの蒸発、経路の発見プロセスを詳しく理解できます。
              </p>
              <div className="mb-6">
                <h4 className="font-medium mb-2">特徴:</h4>
                <ul className="text-sm text-muted-foreground space-y-1">
                  <li>• ステップ実行機能</li>
                  <li>• 詳細なアルゴリズム解説</li>
                  <li>• 蟻の行動パターン可視化</li>
                  <li>• 学習向けの丁寧な説明</li>
                </ul>
              </div>
              <Link to="/education">
                <Button className="w-full" size="lg" variant="outline">
                  教育モードを開始
                </Button>
              </Link>
            </div>
          </div>

          <div className="mt-12 text-center">
            <div className="bg-muted/50 p-6 rounded-lg">
              <h3 className="text-lg font-semibold mb-3">蟻コロニー最適化（ACO）について</h3>
              <p className="text-muted-foreground">
                蟻コロニー最適化は、蟻の集団行動を模倣した最適化アルゴリズムです。
                蟻が餌を探すときに最短経路を発見する能力を活用して、
                巡回セールスマン問題などの組合せ最適化問題を効率的に解決します。
                フェロモンの蒸発と蓄積により、時間とともに最適解に収束していく様子を観察できます。
              </p>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}
