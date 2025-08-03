# Deployment Guide - React Component MCP Server

## Prerequisites

### For Docker Deployment
- Docker Desktop installed and running
- Docker Compose (usually included with Docker Desktop)

### For Direct Deployment  
- Rust 1.85+ installed
- Cargo build tools

## Deployment Options

### Option 1: Docker Deployment (Recommended)

1. **Start Docker Desktop**
   - Ensure Docker is running on your system

2. **Build the Container**
   ```bash
   chmod +x build-docker.sh
   ./build-docker.sh
   ```

3. **Run with Docker**
   ```bash
   # Simple run
   docker run -p 8080:8080 react-component-server:latest
   
   # Run in background with logs
   docker run -d --name mcp-react-server -p 8080:8080 react-component-server:latest
   docker logs -f mcp-react-server
   ```

4. **Run with Docker Compose**
   ```bash
   docker-compose up -d
   docker-compose logs -f
   ```

### Option 2: Direct Binary Deployment

1. **Build the Binary**
   ```bash
   cargo build --release --bin react-component-server
   ```

2. **Run the Server**
   ```bash
   # From the project root
   cd react-component-server
   ../target/release/react-component-server
   
   # Or with custom components directory
   COMPONENTS_DIR=/path/to/components ../target/release/react-component-server
   ```

### Option 3: Development Mode

1. **Run in Development**
   ```bash
   cargo run --bin react-component-server
   ```

2. **With Logging**
   ```bash
   RUST_LOG=debug cargo run --bin react-component-server
   ```

## Testing the Deployment

### Using MCP Inspector

1. **Install MCP Inspector**
   ```bash
   npm install -g @modelcontextprotocol/inspector
   ```

2. **Connect to Server**
   ```bash
   # If running with Docker on port 8080
   npx @modelcontextprotocol/inspector http://localhost:8080
   
   # If running locally with stdio
   npx @modelcontextprotocol/inspector ./target/release/react-component-server
   ```

### Using curl (if HTTP transport is enabled)

```bash
# Test server health
curl http://localhost:8080/health

# List tools (if HTTP endpoints are available)
curl -X POST http://localhost:8080/mcp/tools/list \
  -H "Content-Type: application/json" \
  -d '{}'
```

### Manual Testing

1. **Check Component Loading**
   ```bash
   # Start server and check logs for component loading
   RUST_LOG=info ./target/release/react-component-server
   ```

2. **Verify Components Directory**
   ```bash
   # Ensure all 26 components are present
   ls -la components/
   find components -name "manifest.json" | wc -l  # Should show 26
   ```

## Production Deployment

### Docker in Production

1. **Use Docker Compose with resource limits**
   ```yaml
   services:
     react-component-server:
       build: .
       image: react-component-server:latest
       deploy:
         resources:
           limits:
             memory: 512M
             cpus: '0.5'
       restart: always
       healthcheck:
         test: ["CMD", "pgrep", "-f", "react-component-server"]
         interval: 30s
         timeout: 10s
         retries: 3
   ```

2. **Environment Variables**
   ```bash
   # Set production logging
   export RUST_LOG=warn
   export COMPONENTS_DIR=/app/components
   ```

### Binary in Production

1. **Create systemd service** (Linux)
   ```ini
   [Unit]
   Description=React Component MCP Server
   After=network.target
   
   [Service]
   Type=simple
   User=mcpuser
   WorkingDirectory=/opt/mcp-server
   ExecStart=/opt/mcp-server/react-component-server
   Environment=RUST_LOG=warn
   Environment=COMPONENTS_DIR=/opt/mcp-server/components
   Restart=always
   
   [Install]
   WantedBy=multi-user.target
   ```

## Monitoring

### Health Checks
```bash
# Check if process is running
pgrep -f react-component-server

# Monitor logs
tail -f /var/log/mcp-server.log

# Docker health check
docker health ls
```

### Performance Monitoring
```bash
# Memory usage
docker stats mcp-react-server

# Component loading time
time curl -X POST http://localhost:8080/tools/list_components
```

## Troubleshooting

### Common Issues

1. **Components not loading**
   - Check COMPONENTS_DIR path
   - Verify manifest.json files are valid
   - Check file permissions

2. **Port already in use**
   ```bash
   # Find process using port 8080
   lsof -i :8080
   
   # Use different port
   docker run -p 8081:8080 react-component-server:latest
   ```

3. **Permission denied**
   ```bash
   # Fix Docker permissions
   sudo chown -R $USER:$USER /var/run/docker.sock
   
   # Fix binary permissions
   chmod +x target/release/react-component-server
   ```

### Logs and Debugging

```bash
# Increase log level
export RUST_LOG=debug

# Docker logs
docker logs mcp-react-server

# Follow logs in real time
docker logs -f mcp-react-server
```

## Security Considerations

1. **Run as non-root user** (handled in Docker)
2. **Limit resource usage** (memory, CPU)
3. **Use specific network policies** if needed
4. **Regular security updates** for base images
5. **Scan images for vulnerabilities**

```bash
# Scan Docker image
docker scan react-component-server:latest
```