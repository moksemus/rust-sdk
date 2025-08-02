use anyhow::Result;
use rmcp::{model::*, service::ServiceExt, transport::{TokioChildProcess, ConfigureCommandExt}};
use tokio::process::Command;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Starting React Component Server Test...\n");

    // Start the React component server as a child process
    let service = ().serve(TokioChildProcess::new(Command::new("cargo").configure(|cmd| {
        cmd.arg("run")
            .arg("--bin")
            .arg("react-component-server");
    }))?).await?;

    println!("✅ Connected to React Component server");
    
    // Test 1: Check server info and capabilities
    println!("\n📋 Test 1: Server Information");
    let server_info = service.peer_info();
    println!("Server info: {:#?}", server_info);

    // Test 2: List available tools
    println!("\n🔧 Test 2: Available Tools");
    let tools = service.list_tools(Default::default()).await?;
    println!("Found {} tools:", tools.tools.len());
    for tool in &tools.tools {
        println!("  - {}: {}", tool.name, tool.description.as_deref().unwrap_or("No description"));
    }

    // Test 3: List components
    println!("\n📦 Test 3: List Components");
    let component_list = service
        .call_tool(CallToolRequestParam {
            name: "list_components".into(),
            arguments: Some(json!({}).as_object().unwrap().clone()),
        })
        .await?;
    println!("Component list result: {:#?}", component_list);

    // Test 4: Get specific component (Button)
    println!("\n🔘 Test 4: Get Button Component");
    let button_component = service
        .call_tool(CallToolRequestParam {
            name: "get_component".into(),
            arguments: Some(json!({
                "name": "Button",
                "include_examples": true,
                "include_typescript": false
            }).as_object().unwrap().clone()),
        })
        .await?;
    println!("Button component result: {:#?}", button_component);

    // Test 5: Search components
    println!("\n🔍 Test 5: Search Components");
    let search_result = service
        .call_tool(CallToolRequestParam {
            name: "search_components".into(),
            arguments: Some(json!({
                "query": "button",
                "limit": 5
            }).as_object().unwrap().clone()),
        })
        .await?;
    println!("Search result: {:#?}", search_result);

    println!("\n✅ Basic tests completed successfully!");
    
    service.cancel().await?;
    Ok(())
}