# ACO TSP Visualizer

Ant Colony Optimization for Traveling Salesman Problem visualizer built with Rust WebAssembly and React.

## Features

- **Real-time ACO Algorithm Visualization**: Watch ants find optimal routes in real-time
- **Interactive City Management**: Click to add cities, drag to move them
- **Parameter Tuning**: Adjust algorithm parameters (α, β, evaporation rate, etc.)
- **Performance Optimized**: Rust WebAssembly for high-performance computation
- **Modern UI**: Built with React, TypeScript, and Tailwind CSS v4

## Architecture

```
aco-wasm/
├── wasm/          # Rust WebAssembly core
│   ├── src/
│   │   ├── aco/           # ACO algorithm implementation
│   │   ├── geometry/      # City and distance calculations
│   │   ├── rendering/     # Canvas rendering
│   │   ├── simulation/    # State management
│   │   └── input/         # Input handling
│   └── Cargo.toml
├── front/         # React frontend
│   ├── src/
│   │   ├── components/    # UI components
│   │   ├── hooks/         # React hooks
│   │   └── types/         # TypeScript types
│   └── package.json
└── scripts/       # Build scripts
```

## Development

### Prerequisites

- **Rust** (1.70+)
- **wasm-pack** (0.12+)
- **Bun** (1.0+)

### Setup

1. Clone the repository
2. Install dependencies:
   ```bash
   bun install
   ```

### Development Server

```bash
bun run dev
```

This will:
- Build the WebAssembly module
- Start the React development server
- Enable hot reloading for both Rust and React code

### Building

```bash
# Build WebAssembly only
bun run build:wasm

# Build frontend only
bun run build:front

# Build everything
bun run build
```

### Testing

```bash
# Test WebAssembly
bun run test:wasm

# Test frontend
bun run test:front

# Test everything
bun run test
```

### Linting and Formatting

```bash
# Lint frontend
bun run lint:front

# Fix linting issues
bun run lint:fix

# Format code
bun run format
```

## Algorithm Parameters

- **Number of Ants**: 10-200 (default: 50)
- **Evaporation Rate**: 0.01-0.9 (default: 0.1)
- **Alpha (α)**: 0.1-5.0 (default: 1.0) - Pheromone importance
- **Beta (β)**: 0.1-5.0 (default: 2.0) - Distance importance

## Technology Stack

- **Core Algorithm**: Rust + WebAssembly
- **Frontend**: React 19 + TypeScript
- **Styling**: Tailwind CSS v4
- **Build Tool**: Vite
- **Package Manager**: Bun
- **Linting**: Biome.js v2
- **UI Components**: Radix UI + shadcn/ui

## Performance

- **WebAssembly**: 5-10x faster than JavaScript implementation
- **60 FPS**: Smooth real-time visualization
- **Memory Efficient**: Optimized data structures
- **Scalable**: Handles 100+ cities efficiently

## License

MIT License