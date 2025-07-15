#!/bin/bash

set -e

echo "Building WebAssembly module..."

cd wasm

# Build the WebAssembly module
wasm-pack build --target web --out-dir pkg --dev

echo "WebAssembly build completed!"

# Copy the generated files to the front-end
if [ -d "../front/src/wasm" ]; then
    rm -rf ../front/src/wasm
fi

mkdir -p ../front/src/wasm
cp -r pkg/* ../front/src/wasm/

echo "WebAssembly files copied to front-end!"