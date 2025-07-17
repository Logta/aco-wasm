#!/bin/bash

set -e

echo "Building Educational WebAssembly module..."

cd education-wasm

# Build the WebAssembly module
wasm-pack build --target web --out-dir pkg --dev

echo "Educational WebAssembly build completed!"

# Copy the generated files to the front-end
mkdir -p ../front/src/education-wasm
cp -r pkg/* ../front/src/education-wasm/

echo "Educational WebAssembly files copied to front-end!"