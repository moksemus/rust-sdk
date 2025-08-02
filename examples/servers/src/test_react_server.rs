// Simple test to verify React component server functionality

use anyhow::Result;
use rmcp::{model::CallToolRequestParam, service::ServiceExt, transport::stdio};
use tokio::process::Command;
use rmcp::transport::{TokioChildProcess, ConfigureCommandExt};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    // Start the React component server as a child process
    let service = ().serve(TokioChildProcess::new(Command::new("cargo").configure(|cmd| {
        cmd.arg("run")
            .arg("-p")
            .arg("mcp-server-examples")
            .arg("--bin")
            .arg("react_component_server");
    }))?).await?;

    // Initialize
    let server_info = service.peer_info();
    println!("Connected to React Component server: {server_info:#?}");

    // List tools
    let tools = service.list_tools(Default::default()).await?;
    println!("Available tools: {tools:#?}");

    // Test list_components tool
    let component_list = service
        .call_tool(CallToolRequestParam {
            name: "list_components".into(),
            arguments: Some(json!({}).as_object().unwrap().clone()),
        })
        .await?;
    println!("Component list result: {component_list:#?}");

    // Test get_component tool
    let button_component = service
        .call_tool(CallToolRequestParam {
            name: "get_component".into(),
            arguments: Some(json!({
                "name": "Button",
                "include_examples": true,
                "include_typescript": true
            }).as_object().unwrap().clone()),
        })
        .await?;
    println!("Button component result: {button_component:#?}");

    // Test search_components tool
    let search_result = service
        .call_tool(CallToolRequestParam {
            name: "search_components".into(),
            arguments: Some(json!({
                "query": "button",
                "limit": 5
            }).as_object().unwrap().clone()),
        })
        .await?;
    println!("Search result: {search_result:#?}");

    // Test get_documentation tool
    let docs_result = service
        .call_tool(CallToolRequestParam {
            name: "get_documentation".into(),
            arguments: Some(json!({
                "topic": "getting-started"
            }).as_object().unwrap().clone()),
        })
        .await?;
    println!("Documentation result: {docs_result:#?}");

    service.cancel().await?;
    Ok(())
}