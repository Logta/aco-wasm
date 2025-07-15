# ACO TSP Visualizer - Claude Code Documentation

## Project Overview
Ant Colony Optimization for Traveling Salesman Problem visualizer built with Rust WebAssembly and React. Real-time ACO algorithm visualization with interactive city management and parameter tuning.

## Development Commands

### Setup
```bash
bun install
```

### Development
```bash
bun run dev                # Start development server (builds WASM + React)
```

### Building
```bash
bun run build:wasm        # Build WebAssembly only
bun run build:front       # Build frontend only
bun run build             # Build everything
```

### Testing
```bash
bun run test:wasm         # Test Rust code
bun run test:front        # Test React code
bun run test              # Test everything
```

### Linting & Formatting
```bash
bun run lint:front        # Lint frontend with Biome
bun run lint:fix          # Fix linting issues
bun run format            # Format code
```

## Project Structure

```
aco-wasm/
├── wasm/                 # Rust WebAssembly core
│   ├── src/
│   │   ├── aco/         # ACO algorithm (ant.rs, colony.rs, pheromone.rs)
│   │   ├── geometry/    # City and distance calculations
│   │   ├── rendering/   # Canvas rendering
│   │   ├── simulation/  # State management
│   │   └── input/       # Input handling
│   └── Cargo.toml
├── front/               # React frontend
│   ├── src/
│   │   ├── components/  # UI components (ACOVisualizer, ui/)
│   │   ├── hooks/       # React hooks (useACOEngine.ts)
│   │   ├── wasm/        # Generated WASM bindings
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

- `front/src/components/ACOVisualizer.tsx` - Main visualization component
- `front/src/hooks/useACOEngine.ts` - WASM integration hook
- `front/vite.config.ts` - Vite configuration with WASM support
- `wasm/src/lib.rs` - WebAssembly entry point
- `wasm/src/aco/` - ACO algorithm implementation

## Prerequisites

- Rust (1.70+)
- wasm-pack (0.12+)
- Bun (1.0+)

## Current Status
- Modified files: `front/src/hooks/useACOEngine.ts`, `front/vite.config.ts`
- WebAssembly integration functional
- React UI components implemented
- Real-time visualization working