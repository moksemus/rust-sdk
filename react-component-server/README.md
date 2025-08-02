# React Component MCP Server

A Model Context Protocol (MCP) server for serving React components and their documentation. This server provides programmatic access to a library of React components, making it easy for AI assistants and other tools to discover, search, and retrieve component information.

## Features

- **Component Library**: Access to a curated set of React components (Button, Card, Input, etc.)
- **Rich Metadata**: Each component includes props, examples, TypeScript definitions, and documentation
- **Search & Discovery**: Search components by name, description, tags, or category
- **Documentation**: Access to component usage guides and best practices
- **Resource Access**: Read component source code and documentation via MCP resources

## Available Tools

### Component Tools

1. **`list_components`** - List available React components with optional filtering
   - Filter by category, tags, or search terms
   - Limit results for pagination
   - Returns component metadata (name, description, category, etc.)

2. **`get_component`** - Get detailed information about a specific component
   - Full source code
   - Complete prop definitions with types and descriptions
   - Usage examples
   - TypeScript definitions (optional)

3. **`search_components`** - Search components by query string
   - Search across component names, descriptions, and props
   - Filter by categories and tags
   - Relevance-based sorting

4. **`get_component_categories`** - Get all available categories and their counts

5. **`get_component_tags`** - Get all available component tags

### Documentation Tools

6. **`get_documentation`** - Get documentation for specific topics
   - Getting started guides
   - Theming and customization
   - Best practices
   - Optional section filtering

7. **`list_documentation_topics`** - List all available documentation topics

## Usage

### Running the Server

```bash
# From this directory
cargo run

# Or run a specific binary
cargo run --bin react-component-server
```

The server communicates via stdio and follows the MCP specification.

### Testing the Server

Run the included test clients to verify functionality:

```bash
# Basic functionality tests
cargo run --bin test-basic

# Advanced features (documentation, resources, search)
cargo run --bin test-advanced

# Error handling tests
cargo run --bin test-errors
```

### Example Tool Calls

#### List All Components
```json
{
  "method": "tools/call",
  "params": {
    "name": "list_components",
    "arguments": {}
  }
}
```

#### Get a Specific Component
```json
{
  "method": "tools/call",
  "params": {
    "name": "get_component",
    "arguments": {
      "name": "Button",
      "include_examples": true,
      "include_typescript": true
    }
  }
}
```

#### Search Components
```json
{
  "method": "tools/call",
  "params": {
    "name": "search_components",
    "arguments": {
      "query": "form input",
      "categories": ["Form"],
      "limit": 10
    }
  }
}
```

#### Get Documentation
```json
{
  "method": "tools/call",
  "params": {
    "name": "get_documentation",
    "arguments": {
      "topic": "getting-started",
      "section": "installation"
    }
  }
}
```

## Available Components

The server currently includes these sample components:

- **Button** - Customizable button with variants (primary, secondary, danger) and sizes
- **Card** - Flexible container component with header, content, and footer
- **Input** - Controlled input component with validation and various types

Each component includes:
- Complete TypeScript/JSX source code
- Detailed prop definitions with types and descriptions
- Multiple usage examples
- Category and tag classification

## Available Documentation

- **getting-started** - Installation and basic usage guide
- **theming** - Customization and styling guide

## Resources

The server also exposes components and documentation as MCP resources:

- `component://ComponentName` - Access component source and documentation
- `docs://topic` - Access documentation content

## Architecture

The server is built using the RMCP (Rust MCP) SDK and follows these patterns:

- **Modular Design**: Separate modules for types, server logic, and component library
- **Type Safety**: Full TypeScript-like type definitions for all components
- **Error Handling**: Comprehensive error messages with helpful context
- **Extensible**: Easy to add new components and documentation

## Project Structure

```
react-component-server/
├── src/
│   ├── main.rs              # Server entry point
│   ├── server/              # Server implementation
│   │   ├── mod.rs           # Module exports
│   │   ├── server.rs        # Main server logic with tools
│   │   ├── types.rs         # Type definitions and schemas
│   │   └── library.rs       # Sample component library
│   └── client/              # Test clients
│       ├── test_react_component_server.rs  # Basic tests
│       ├── test_react_advanced.rs          # Advanced tests
│       └── test_react_errors.rs            # Error handling tests
├── Cargo.toml               # Project configuration
└── README.md               # This file
```

## Development

To extend the server:

1. Add new components to `src/server/library.rs`
2. Update types in `src/server/types.rs` if needed
3. Add new tools to `src/server/server.rs`
4. Test with the provided test clients

The server uses the `#[tool_router]` and `#[tool]` macros from RMCP for automatic tool registration and JSON schema generation.

## Dependencies

This project requires the RMCP (Rust MCP) SDK, which should be available in the parent directory structure. The server is built on:

- **RMCP SDK**: Core MCP protocol implementation
- **Tokio**: Async runtime
- **Serde**: JSON serialization
- **Schemars**: JSON schema generation
- **Tracing**: Structured logging