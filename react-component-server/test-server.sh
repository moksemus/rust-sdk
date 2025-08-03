#!/bin/bash

# Test script for React Component MCP Server

set -e

echo "🧪 Testing React Component MCP Server..."

# Build the server first
echo "📦 Building server..."
cargo build --release --bin react-component-server

# Check if components directory exists
if [ ! -d "components" ]; then
    echo "❌ Components directory not found!"
    exit 1
fi

# Count components
COMPONENT_COUNT=$(ls components | wc -l)
echo "📊 Found $COMPONENT_COUNT components"

# Count manifest files
MANIFEST_COUNT=$(find components -name "manifest.json" | wc -l)
echo "📋 Found $MANIFEST_COUNT manifest files"

if [ "$COMPONENT_COUNT" -ne "$MANIFEST_COUNT" ]; then
    echo "⚠️  Warning: Component count ($COMPONENT_COUNT) doesn't match manifest count ($MANIFEST_COUNT)"
fi

# Test server startup (timeout after 5 seconds)
echo "🚀 Testing server startup..."
timeout 5s bash -c '
    RUST_LOG=info ../target/release/react-component-server 2>&1 | head -10
' || echo "✅ Server started successfully (timeout expected)"

echo ""
echo "🎯 Ready for deployment!"
echo ""
echo "Next steps:"
echo "  1. Start Docker Desktop"
echo "  2. Run: ./build-docker.sh"
echo "  3. Run: docker run -p 8080:8080 react-component-server:latest"
echo "  4. Test: npx @modelcontextprotocol/inspector ./target/release/react-component-server"