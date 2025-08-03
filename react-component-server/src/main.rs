use anyhow::Result;
use rmcp::{
    ServiceExt,
    transport::{
        stdio,
        streamable_http_server::{StreamableHttpService, session::local::LocalSessionManager}
    }
};
use tracing_subscriber::{self, EnvFilter};
use std::env;

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

    // Check if we should use HTTP or stdio transport
    let use_http = env::var("MCP_HTTP_PORT").is_ok() || env::args().any(|arg| arg == "--http");
    let port = env::var("MCP_HTTP_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    // Create an instance of our React component server
    let service = ReactComponentServer::new();

    if use_http {
        tracing::info!("Starting HTTP transport on port {}", port);
        let addr = format!("0.0.0.0:{}", port);
        
        let http_service = StreamableHttpService::new(
            || Ok(ReactComponentServer::new()),
            LocalSessionManager::default().into(),
            Default::default(),
        );
        
        let router = axum::Router::new().nest_service("/mcp", http_service);
        let tcp_listener = tokio::net::TcpListener::bind(&addr).await?;
        
        tracing::info!("MCP HTTP server listening on {}", addr);
        
        axum::serve(tcp_listener, router)
            .with_graceful_shutdown(async {
                tokio::signal::ctrl_c().await.unwrap();
                tracing::info!("Received Ctrl+C, shutting down HTTP server");
            })
            .await?;
    } else {
        tracing::info!("Starting stdio transport");
        let service = service
            .serve(stdio())
            .await
            .inspect_err(|e| {
                tracing::error!("serving error: {:?}", e);
            })?;
        service.waiting().await?;
    }

    Ok(())
}