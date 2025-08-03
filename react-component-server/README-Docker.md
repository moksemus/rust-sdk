# React Component MCP Server

A Model Context Protocol (MCP) server that provides access to a comprehensive library of Fluent UI React components through standardized tools and resources.

## Features

- **26 Fluent UI Components** with complete manifests and documentation
- **Flexible Architecture** supporting both manifest-based and automatic component discovery
- **Rich Metadata** including props, examples, categories, and dependencies
- **MCP Protocol Compliance** with tools, resources, and proper schema validation
- **Docker Support** for easy deployment and distribution

## Components Available

### Input Components
- Button, Checkbox, Radio, Switch, Input, Textarea, Label, Combobox, Field

### Display Components  
- Text, Image, Card, Avatar, Badge, Progress, Spinner

### Navigation Components
- Link, Accordion, Breadcrumb, Menu, Tabs

### Layout Components
- Dialog, Popover, Divider, Drawer, Carousel

## Quick Start

### Using Docker

1. **Build the container:**
   ```bash
   ./build-docker.sh
   ```

2. **Run with Docker:**
   ```bash
   docker run -p 8080:8080 react-component-server:latest
   ```

3. **Run with Docker Compose:**
   ```bash
   docker-compose up -d
   ```

### Using Rust directly

1. **Build:**
   ```bash
   cargo build --release --bin react-component-server
   ```

2. **Run:**
   ```bash
   ./target/release/react-component-server
   ```

## MCP Tools Available

### `list_components`
Lists available React components with optional filtering by category, tags, search terms, and limits.

**Parameters:**
- `category` (optional): Filter by component category
- `tags` (optional): Filter by component tags
- `search` (optional): Search in component names and descriptions
- `limit` (optional): Maximum number of components to return

### `get_component`
Retrieves detailed information about a specific React component including props, examples, and TypeScript definitions.

**Parameters:**
- `name` (required): Name of the component (case-insensitive)
- `include_examples` (optional): Whether to include code examples
- `include_typescript` (optional): Whether to include TypeScript definitions

### `get_documentation`
Retrieves documentation for specific topics related to the component library.

**Parameters:**
- `topic` (required): Documentation topic name
- `section` (optional): Specific section within the topic

## Component Structure

Each component includes:
- **Manifest file** (`manifest.json`) with metadata, props, and dependencies
- **Component file** (`component.tsx`) with the React implementation
- **Types file** (`types.ts`) with TypeScript definitions
- **Examples** (`examples/*.tsx`) with usage examples

## Environment Variables

- `RUST_LOG`: Log level (default: `info`)
- `COMPONENTS_DIR`: Path to components directory (default: `./components`)

## Development

### Adding New Components

1. Create component directory: `components/{component-name}/`
2. Add files:
   - `manifest.json` - Component metadata
   - `component.tsx` - React component
   - `types.ts` - TypeScript definitions
   - `examples/` - Usage examples
3. Server will automatically discover and load the component

### Component Categories

- **Input**: Form controls and user input elements
- **Display**: Content presentation and media components  
- **Navigation**: Routing and navigation elements
- **Layout**: Container and positioning components
- **Feedback**: Loading indicators and status elements
- **Controls**: Interactive action elements

## Docker Configuration

The Docker container:
- Uses multi-stage build for optimized size
- Runs as non-root user for security
- Includes health checks
- Supports volume mounting for component updates
- Exposes port 8080 for network access

## License

This project provides tooling for Fluent UI components. Please refer to Microsoft's Fluent UI license for component usage terms.