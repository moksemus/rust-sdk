use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{self, EnvFilter};

mod server;

use server::ReactComponentServer;

/// React Component MCP Server
/// 
/// A Model Context Protocol server for serving React components and documentation.
/// This server provides programmatic access to a curated library of React components,
/// making it easy for AI assistants and other tools to discover, search, and retrieve
/// component information including source code, props, examples, and documentation.
/// 
/// Usage: cargo run
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the tracing subscriber with file and stdout logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting React Component MCP server");

    // Create an instance of our React component server
    let service = ReactComponentServer::new()
        .serve(stdio())
        .await
        .inspect_err(|e| {
            tracing::error!("serving error: {:?}", e);
        })?;

    service.waiting().await?;
    Ok(())
}