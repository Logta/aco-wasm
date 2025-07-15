#!/bin/bash

set -e

echo "Building React frontend..."

cd front

# Install dependencies
bun install

# Run type checking
bun run tsc

# Build the frontend
bun run build

echo "Frontend build completed!"