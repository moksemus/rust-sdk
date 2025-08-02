# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

RMCP is the official Rust SDK for the Model Context Protocol (MCP), providing both client and server implementations with tokio async runtime. This is a workspace containing two main crates:

- `rmcp` (crates/rmcp): Core MCP protocol implementation
- `rmcp-macros` (crates/rmcp-macros): Procedural macros for generating MCP tool implementations

## Common Development Commands

Use [just](https://github.com/casey/just) for common development tasks:

```bash
# Format code (requires nightly Rust)
just fmt

# Run clippy with warnings as errors
just check

# Auto-fix formatting and clippy issues
just fix

# Run all tests with all features
just test

# Generate code coverage report
just cov
```

Direct cargo commands:
```bash
# Format (nightly required due to unstable features)
cargo +nightly fmt --all

# Clippy
cargo clippy --all-targets --all-features -- -D warnings

# Test
cargo test --all-features

# Coverage (requires cargo-llvm-cov)
cargo llvm-cov --lcov --output-path target/llvm-cov-target/coverage.lcov
```

## Code Architecture

### Core Components

1. **Service Layer** (`crates/rmcp/src/service.rs`):
   - `Service<R>` trait: Core service interface for handling requests/notifications
   - `ServiceRole` trait: Defines client/server roles with associated types
   - `RunningService<R, S>`: Manages running service instances with cancellation
   - `Peer<R>`: Interface for communication with remote peers

2. **Handler Layer** (`crates/rmcp/src/handler/`):
   - `ServerHandler`: Server-side request/notification handling
   - `ClientHandler`: Client-side communication interface
   - Tool routing system for server implementations

3. **Transport Layer** (`crates/rmcp/src/transport/`):
   - Multiple transport implementations: stdio, SSE, HTTP, WebSocket, child processes
   - Authentication support including OAuth
   - Streaming and batch message handling

4. **Model Layer** (`crates/rmcp/src/model/`):
   - JSON-RPC message types and serialization
   - MCP-specific data structures (tools, resources, prompts)
   - Schema generation support

### Macro System

The `rmcp-macros` crate provides:
- `#[tool_router]`: Generates tool routing implementations
- `#[tool]`: Automatically creates tool definitions from methods
- Support for structured input/output with JSON schemas
- Parameter extraction and validation

### Key Patterns

1. **Service Implementation**: Implement `Service<R>` trait where `R` is either `RoleClient` or `RoleServer`
2. **Tool Creation**: Use `#[tool_router]` on impl blocks and `#[tool]` on methods
3. **Structured Data**: Use `Json<T>` wrapper and `Parameters<T>` for type-safe tool parameters
4. **Transport Selection**: Choose appropriate transport based on deployment (stdio for CLI, HTTP for web services)

## Testing

- Integration tests in `crates/rmcp/tests/`
- Cross-language compatibility tests with JavaScript and Python
- Complex schema validation tests
- Transport-specific test suites

## Build Configuration

- **Rust Edition**: 2024
- **MSRV**: 1.85
- **Clippy**: Custom configuration in `clippy.toml` with MSRV 1.85, 10 argument threshold
- **Rustfmt**: Nightly features enabled, Unix line endings, 100 char width, organized imports

The project uses workspace-level dependency management and supports multiple optional features for different transport and functionality combinations.