#!/bin/bash

set -e

echo "Building TSP WebAssembly module..."

cd tsp-wasm

# Build the WebAssembly module
wasm-pack build --target web --out-dir pkg --dev

echo "TSP WebAssembly build completed!"

# Copy the generated files to the front-end
if [ -d "../front/src/wasm" ]; then
    rm -rf ../front/src/wasm
fi

mkdir -p ../front/src/wasm
cp -r pkg/* ../front/src/wasm/

echo "TSP WebAssembly files copied to front-end!"