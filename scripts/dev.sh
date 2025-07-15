#!/bin/bash

set -e

echo "Starting development server..."

# Function to run commands in parallel
run_in_parallel() {
    local pids=()
    
    # Build WebAssembly in watch mode
    (cd wasm && cargo watch -x "build --lib") &
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

# Build WebAssembly once
echo "Building WebAssembly..."
cd wasm
wasm-pack build --target web --out-dir pkg --dev
cd ..

# Copy files to frontend
if [ -d "front/src/wasm" ]; then
    rm -rf front/src/wasm
fi
mkdir -p front/src/wasm
cp -r wasm/pkg/* front/src/wasm/

# Install frontend dependencies
echo "Installing frontend dependencies..."
cd front
bun install
cd ..

echo "Starting development servers..."
run_in_parallel