use anyhow::Result;
use rmcp::{model::*, service::ServiceExt, transport::{TokioChildProcess, ConfigureCommandExt}};
use tokio::process::Command;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Error Handling Tests for React Component Server...\n");

    // Start the React component server as a child process
    let service = ().serve(TokioChildProcess::new(Command::new("cargo").configure(|cmd| {
        cmd.arg("run")
            .arg("-p")
            .arg("mcp-server-examples")
            .arg("--bin")
            .arg("react_component_server");
    }))?).await?;

    println!("✅ Connected to React Component server");

    // Test 1: Try to get non-existent component
    println!("\n❌ Test 1: Get Non-existent Component");
    let result = service
        .call_tool(CallToolRequestParam {
            name: "get_component".into(),
            arguments: Some(json!({
                "name": "NonExistentComponent"
            }).as_object().unwrap().clone()),
        })
        .await;
    
    match result {
        Ok(response) => {
            println!("Response: {:#?}", response);
            if let Some(is_error) = response.is_error {
                if is_error {
                    println!("✅ Correctly returned error for non-existent component");
                } else {
                    println!("❌ Should have returned an error");
                }
            }
        }
        Err(e) => println!("Error calling tool: {:#?}", e),
    }

    // Test 2: Try to get non-existent documentation
    println!("\n❌ Test 2: Get Non-existent Documentation");
    let result = service
        .call_tool(CallToolRequestParam {
            name: "get_documentation".into(),
            arguments: Some(json!({
                "topic": "non-existent-topic"
            }).as_object().unwrap().clone()),
        })
        .await;
    
    match result {
        Ok(response) => {
            println!("Response: {:#?}", response);
            if let Some(is_error) = response.is_error {
                if is_error {
                    println!("✅ Correctly returned error for non-existent documentation");
                } else {
                    println!("❌ Should have returned an error");
                }
            }
        }
        Err(e) => println!("Error calling tool: {:#?}", e),
    }

    // Test 3: Try to get non-existent documentation section
    println!("\n❌ Test 3: Get Non-existent Documentation Section");
    let result = service
        .call_tool(CallToolRequestParam {
            name: "get_documentation".into(),
            arguments: Some(json!({
                "topic": "getting-started",
                "section": "non-existent-section"
            }).as_object().unwrap().clone()),
        })
        .await;
    
    match result {
        Ok(response) => {
            println!("Response: {:#?}", response);
            if let Some(is_error) = response.is_error {
                if is_error {
                    println!("✅ Correctly returned error for non-existent section");
                } else {
                    println!("❌ Should have returned an error");
                }
            }
        }
        Err(e) => println!("Error calling tool: {:#?}", e),
    }

    // Test 4: Try to read non-existent resource
    println!("\n❌ Test 4: Read Non-existent Resource");
    let result = service
        .read_resource(ReadResourceRequestParam {
            uri: "component://NonExistent".to_string(),
        })
        .await;
    
    match result {
        Ok(response) => println!("❌ Should have failed: {:#?}", response),
        Err(e) => println!("✅ Correctly failed to read non-existent resource: {:#?}", e),
    }

    // Test 5: Try invalid tool call
    println!("\n❌ Test 5: Invalid Tool Call");
    let result = service
        .call_tool(CallToolRequestParam {
            name: "non_existent_tool".into(),
            arguments: Some(json!({}).as_object().unwrap().clone()),
        })
        .await;
    
    match result {
        Ok(response) => println!("❌ Should have failed: {:#?}", response),
        Err(e) => println!("✅ Correctly failed for non-existent tool: {:#?}", e),
    }

    println!("\n✅ Error handling tests completed!");
    
    service.cancel().await?;
    Ok(())
}