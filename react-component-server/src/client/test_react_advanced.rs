use anyhow::Result;
use rmcp::{model::*, service::ServiceExt, transport::{TokioChildProcess, ConfigureCommandExt}};
use tokio::process::Command;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Advanced React Component Server Testing...\n");

    // Start the React component server as a child process
    let service = ().serve(TokioChildProcess::new(Command::new("cargo").configure(|cmd| {
        cmd.arg("run")
            .arg("--bin")
            .arg("react-component-server");
    }))?).await?;

    println!("✅ Connected to React Component server");

    // Test 1: Documentation tools
    println!("\n📚 Test 1: Documentation Topics");
    let doc_topics = service
        .call_tool(CallToolRequestParam {
            name: "list_documentation_topics".into(),
            arguments: Some(json!({}).as_object().unwrap().clone()),
        })
        .await?;
    println!("Documentation topics: {:#?}", doc_topics);

    // Test 2: Get specific documentation
    println!("\n📖 Test 2: Get Getting Started Documentation");
    let getting_started = service
        .call_tool(CallToolRequestParam {
            name: "get_documentation".into(),
            arguments: Some(json!({
                "topic": "getting-started"
            }).as_object().unwrap().clone()),
        })
        .await?;
    println!("Getting started docs: {:#?}", getting_started);

    // Test 3: Component categories
    println!("\n🏷️ Test 3: Component Categories");
    let categories = service
        .call_tool(CallToolRequestParam {
            name: "get_component_categories".into(),
            arguments: Some(json!({}).as_object().unwrap().clone()),
        })
        .await?;
    println!("Component categories: {:#?}", categories);

    // Test 4: Component tags
    println!("\n🏷️ Test 4: Component Tags");
    let tags = service
        .call_tool(CallToolRequestParam {
            name: "get_component_tags".into(),
            arguments: Some(json!({}).as_object().unwrap().clone()),
        })
        .await?;
    println!("Component tags: {:#?}", tags);

    // Test 5: List resources
    println!("\n📁 Test 5: List Resources");
    let resources = service.list_resources(Default::default()).await?;
    println!("Found {} resources:", resources.resources.len());
    for resource in &resources.resources {
        println!("  - {}: {}", resource.uri, resource.name);
    }

    // Test 6: Read a component resource
    println!("\n📄 Test 6: Read Button Component Resource");
    let button_resource = service
        .read_resource(ReadResourceRequestParam {
            uri: "component://Button".to_string(),
        })
        .await?;
    println!("Button resource content (first 500 chars):");
    if let Some(content) = button_resource.contents.first() {
        match content {
            ResourceContents::TextResourceContents { text, .. } => {
                println!("{}", &text[..text.len().min(500)]);
                if text.len() > 500 {
                    println!("... (truncated)");
                }
            }
            ResourceContents::BlobResourceContents { .. } => println!("(binary content)"),
        }
    }

    // Test 7: Read documentation resource
    println!("\n📄 Test 7: Read Documentation Resource");
    let docs_resource = service
        .read_resource(ReadResourceRequestParam {
            uri: "docs://getting-started".to_string(),
        })
        .await?;
    println!("Documentation resource content (first 500 chars):");
    if let Some(content) = docs_resource.contents.first() {
        match content {
            ResourceContents::TextResourceContents { text, .. } => {
                println!("{}", &text[..text.len().min(500)]);
                if text.len() > 500 {
                    println!("... (truncated)");
                }
            }
            ResourceContents::BlobResourceContents { .. } => println!("(binary content)"),
        }
    }

    // Test 8: Advanced search with filters
    println!("\n🔍 Test 8: Advanced Search with Filters");
    let filtered_search = service
        .call_tool(CallToolRequestParam {
            name: "search_components".into(),
            arguments: Some(json!({
                "query": "form",
                "categories": ["Form"],
                "limit": 10
            }).as_object().unwrap().clone()),
        })
        .await?;
    println!("Filtered search result: {:#?}", filtered_search);

    println!("\n✅ Advanced tests completed successfully!");
    
    service.cancel().await?;
    Ok(())
}