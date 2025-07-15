# Ant Colony Optimization TSP Visualizer 仕様書

## 1. プロジェクト概要

巡回セールスマン問題をAnt Colony Optimizationアルゴリズムで解く過程をリアルタイム可視化するWebアプリケーション。高性能計算とCanvas描画をRust WebAssemblyで実装し、UI制御をReactで行う。

## 2. 技術要件

- **フロントエンド**: React 18 + TypeScript + Vite v7
- **UIライブラリ**: shadcn/ui + Tailwind CSS
- **高性能処理**: Rust + WebAssembly (wasm-pack)
- **Canvas描画**: Rust WebAssembly + web-sys
- **状態管理**: React hooks + WebAssembly state

## 3. アーキテクチャ

### 3.1 責務分担
- **React**: パラメータ入力UI、制御ボタン、統計表示、レイアウト
- **Rust WASM**: ACOアルゴリズム、Canvas描画、入力処理、アニメーション制御

### 3.2 通信インターフェース
- React → WASM: パラメータ設定、制御コマンド
- WASM → React: 統計データ、状態更新通知

## 4. 機能要件

### 4.1 都市管理
- クリックで都市追加
- ダブルクリックで都市削除
- ドラッグで都市移動
- ランダム生成機能
- 3-50都市の制限

### 4.2 ACOアルゴリズム
- Ant System実装
- エリート戦略
- 並列アリ実行
- フェロモン蒸発・更新
- 収束判定

### 4.3 可視化
- リアルタイムアリ移動アニメーション
- フェロモン濃度を色・太さで表現
- 最良解経路ハイライト
- 60FPS滑らかな描画

### 4.4 制御機能
- 開始/一時停止/再開/停止/リセット
- アニメーション速度調整（1-10段階）
- ステップ実行モード

### 4.5 パラメータ調整
- アリ数: 10-200
- 最大世代: 10-1000
- 蒸発率: 0.01-0.9
- α(フェロモン重要度): 0.1-5.0
- β(距離重要度): 0.1-5.0
- 初期フェロモン: 0.1-10.0

### 4.6 統計・分析
- リアルタイム統計表示
- 距離推移グラフ
- 収束分析
- パフォーマンス測定

## 5. UI設計

### 5.1 レイアウト
```
┌────────────────────────┬─────────────────┐
│                        │                 │
│      Canvas Area       │  Control Panel  │
│                        │                 │
│                        │                 │
└────────────────────────┴─────────────────┘
```

### 5.2 Control Panel構成
1. **実行制御**: 制御ボタン群 + 速度スライダー
2. **都市設定**: ランダム生成、クリア、都市数指定
3. **ACOパラメータ**: 各種パラメータ入力
4. **統計情報**: 数値データ + 推移グラフ
5. **凡例**: フェロモン表現説明

## 6. データ構造

### 6.1 主要データ型
- **City**: id, x, y座標
- **Ant**: 現在位置、訪問済み都市、総距離
- **PheromoneMatrix**: 上三角行列でメモリ効率化
- **ACOParameters**: 全調整可能パラメータ
- **Statistics**: 世代統計、パフォーマンス指標

### 6.2 WebAssembly API
- 都市操作: add_city, remove_city, clear_cities
- 制御: start, pause, resume, stop, reset, step
- 設定: set_parameters, set_animation_speed
- 取得: get_statistics, get_status, get_best_solution
- Canvas: resize_canvas, render制御

## 7. パフォーマンス要件

### 7.1 計算性能
- WebAssembly実装でJavaScript比5-10倍高速化
- 100都市規模での安定動作
- 並列処理によるUI応答性維持

### 7.2 描画性能
- 60FPS安定維持
- Canvas最適化（Dirty Rectangle等）
- 高DPI対応

### 7.3 メモリ効率
- 総メモリ使用量200MB以下
- 上三角行列によるメモリ節約
- ガベージコレクション最小化

## 8. プロジェクト構成

### 8.1 Rustプロジェクト
```
aco-wasm/
├── src/
│   ├── lib.rs (WebAssemblyエントリーポイント)
│   ├── aco/ (アルゴリズム実装)
│   ├── rendering/ (Canvas描画)
│   ├── input/ (入力処理)
│   ├── geometry/ (幾何計算)
│   └── simulation/ (状態管理)
└── Cargo.toml
```

### 8.2 Reactプロジェクト
```
src/
├── components/
│   ├── ACOVisualizer.tsx (メインコンポーネント)
│   ├── Controls/ (制御UI)
│   ├── Display/ (統計表示)
│   └── ui/ (shadcn/ui)
├── hooks/
│   └── useACOEngine.ts (WASM統合)
├── types/
│   └── aco.ts (型定義)
└── utils/
```

## 9. ビルド・デプロイ

### 9.1 開発フロー
1. Rustでwasm-pack build
2. TypeScript型定義生成
3. Viteでフロントエンドビルド
4. 統合テスト実行

### 9.2 最適化設定
- Rustリリースビルド最適化
- WebAssembly最適化フラグ
- Vite production ビルド設定

## 10. テスト要件

### 10.1 単体テスト
- Rust: アルゴリズム正確性
- TypeScript: UI動作

### 10.2 統合テスト
- WASM-JS連携
- Canvas描画検証
- パフォーマンス測定

### 10.3 ベンチマーク
- 各都市数での性能測定
- メモリ使用量監視
- 描画フレームレート確認
