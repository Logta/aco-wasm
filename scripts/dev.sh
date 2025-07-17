#!/bin/bash

set -e

echo "Starting development server..."

# Function to run commands in parallel
run_in_parallel() {
    local pids=()
    
    # Build TSP WebAssembly in watch mode
    (cd tsp-wasm && cargo watch -x "build --lib") &
    pids+=($!)
    
    # Build Education WebAssembly in watch mode
    (cd education-wasm && cargo watch -x "build --lib") &
    pids+=($!)
    
    # Start React development server
    (cd front && bun run dev) &
    pids+=($!)
    
    # Wait for all processes
    wait "${pids[@]}"
}

# Check if cargo-watch is installed
if ! command -v cargo-watch &> /dev/null; then
    echo "cargo-watch is not installed. Installing..."
    cargo install cargo-watch
fi

# Build both WebAssembly modules once
echo "Building TSP WebAssembly..."
cd tsp-wasm
wasm-pack build --target web --out-dir pkg --dev || { echo "TSP WASM build failed"; exit 1; }
cd ..

echo "Building Education WebAssembly..."
cd education-wasm
wasm-pack build --target web --out-dir pkg --dev || { echo "Education WASM build failed"; exit 1; }
cd ..

# Copy files to frontend
if [ -d "front/src/wasm" ]; then
    rm -rf front/src/wasm
fi
mkdir -p front/src/wasm
cp -r tsp-wasm/pkg/* front/src/wasm/

if [ -d "front/src/education-wasm" ]; then
    rm -rf front/src/education-wasm
fi
mkdir -p front/src/education-wasm
cp -r education-wasm/pkg/* front/src/education-wasm/

# Install frontend dependencies
echo "Installing frontend dependencies..."
cd front
bun install
cd ..

echo "Starting development servers..."
run_in_parallel