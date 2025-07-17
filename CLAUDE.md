# ACO TSP Visualizer - Claude Code Documentation

## Project Overview
Ant Colony Optimization for Traveling Salesman Problem visualizer built with Rust WebAssembly and React. Features two modes:
1. **TSP Solver Mode**: Optimized ACO algorithm for solving traveling salesman problems
2. **Educational Mode**: Step-by-step ACO visualization for learning how ants find optimal paths

## Development Commands

### Setup
```bash
bun install
```

### Development
```bash
bun run dev                      # Start development server (builds both WASM modules + React)
```

### Building
```bash
bun run build:tsp-wasm          # Build TSP WebAssembly only
bun run build:education-wasm    # Build Educational WebAssembly only
bun run build:wasm              # Build both WebAssembly modules
bun run build:front             # Build frontend only
bun run build                   # Build everything
```

### Testing
```bash
bun run test:tsp-wasm           # Test TSP Rust code
bun run test:education-wasm     # Test Educational Rust code
bun run test:wasm               # Test all Rust code
bun run test:front              # Test React code
bun run test                    # Test everything
```

### Linting & Formatting
```bash
bun run lint:front             # Lint frontend with Biome
bun run lint:fix               # Fix linting issues
bun run format                 # Format code
```

## Project Structure

```
aco-wasm/
├── tsp-wasm/            # Rust WebAssembly for TSP solving
│   ├── src/
│   │   ├── aco/         # Optimized ACO algorithm
│   │   ├── geometry/    # City and distance calculations
│   │   ├── rendering/   # Canvas rendering
│   │   ├── simulation/  # State management
│   │   └── input/       # Input handling
│   └── Cargo.toml
├── education-wasm/      # Rust WebAssembly for educational visualization
│   ├── src/
│   │   ├── aco/         # Educational ACO with step-by-step visualization
│   │   ├── rendering/   # Canvas rendering with animations
│   │   └── types/       # Shared types and configurations
│   └── Cargo.toml
├── front/               # React frontend
│   ├── src/
│   │   ├── components/  # UI components (ACOVisualizer, EducationalACO, ui/)
│   │   ├── hooks/       # React hooks (useACOEngine.ts, useEducationalACO.ts)
│   │   ├── wasm/        # Generated TSP WASM bindings
│   │   ├── education-wasm/ # Generated Educational WASM bindings
│   │   └── lib/         # Utilities
│   └── package.json
└── scripts/             # Build scripts
```

## Technology Stack

- **Core Algorithm**: Rust + WebAssembly
- **Frontend**: React 19 + TypeScript
- **Styling**: Tailwind CSS v4
- **Build Tool**: Vite
- **Package Manager**: Bun
- **Linting**: Biome.js v2
- **UI Components**: Radix UI + shadcn/ui

## Key Files

### TSP Mode
- `front/src/components/ACOVisualizer.tsx` - TSP visualization component
- `front/src/hooks/useACOEngine.ts` - TSP WASM integration hook
- `tsp-wasm/src/lib.rs` - TSP WebAssembly entry point
- `tsp-wasm/src/aco/` - Optimized ACO algorithm implementation

### Educational Mode
- `front/src/components/EducationalACO.tsx` - Educational visualization component
- `front/src/hooks/useEducationalACO.ts` - Educational WASM integration hook
- `education-wasm/src/lib.rs` - Educational WebAssembly entry point
- `education-wasm/src/aco/` - Educational ACO with step-by-step features

### Common
- `front/src/App.tsx` - Main app with mode switching
- `front/vite.config.ts` - Vite configuration with WASM support

## Prerequisites

- Rust (1.70+)
- wasm-pack (0.12+)
- Bun (1.0+)

## Current Status
- Modified files: `front/src/hooks/useACOEngine.ts`, `front/vite.config.ts`
- WebAssembly integration functional
- React UI components implemented
- Real-time visualization working