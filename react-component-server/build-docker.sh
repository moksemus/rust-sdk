#!/bin/bash

# Build script for React Component MCP Server Docker container

set -e

echo "🚀 Building React Component MCP Server Docker container..."

# Build the Docker image
docker build -t react-component-server:latest -f Dockerfile ..

echo "✅ Docker image built successfully!"

# Show image info
echo "📊 Image information:"
docker images react-component-server:latest

echo ""
echo "🎯 To run the container:"
echo "  docker run -p 8080:8080 react-component-server:latest"
echo ""
echo "🐳 To run with docker-compose:"
echo "  docker-compose up -d"
echo ""
echo "🔧 To run interactively:"
echo "  docker run -it react-component-server:latest /bin/bash"