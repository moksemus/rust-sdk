use std::collections::HashMap;
use std::future::Future;
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler, Json,
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::*,
    service::RequestContext,
    tool, tool_handler, tool_router,
};
use serde_json::json;

use super::types::{
    ReactComponent, ComponentMetadata, ListComponentsRequest, GetComponentRequest,
    GetDocumentationRequest, Documentation, ComponentListResponse,
};
use super::file_loader::ComponentFileLoader;

#[derive(Clone)]
pub struct ReactComponentServer {
    components: HashMap<String, ReactComponent>,
    documentation: HashMap<String, Documentation>,
    tool_router: ToolRouter<ReactComponentServer>,
}

#[tool_router]
impl ReactComponentServer {
    pub fn new() -> Self {
        // Try to find components directory relative to current working directory
        let components_path = std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .join("components");
        
        tracing::info!("Looking for components in: {:?}", components_path);
        let loader = ComponentFileLoader::new(components_path);
        
        let components = match loader.load_components() {
            Ok(components) => {
                tracing::info!("ReactComponentServer initialized with {} components:", components.len());
                for (name, component) in &components {
                    tracing::info!("  - {}: {} (tags: {:?})", name, component.description, component.tags);
                }
                components
            }
            Err(e) => {
                tracing::error!("Failed to load components: {}", e);
                HashMap::new()
            }
        };

        let documentation = match loader.load_documentation() {
            Ok(docs) => docs,
            Err(e) => {
                tracing::error!("Failed to load documentation: {}", e);
                HashMap::new()
            }
        };
        
        Self {
            components,
            documentation,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "List available React components with optional filtering")]
    async fn list_components(
        &self,
        Parameters(params): Parameters<ListComponentsRequest>,
    ) -> Result<Json<ComponentListResponse>, McpError> {
        tracing::info!("list_components called with {} total components", self.components.len());
        tracing::info!("Filter params: category={:?}, tags={:?}, search={:?}, limit={:?}", 
            params.category, params.tags, params.search, params.limit);
        
        let mut components: Vec<ComponentMetadata> = self
            .components
            .values()
            .filter(|component| {
                // Filter by category if specified
                if let Some(ref category) = params.category {
                    if &component.category != category {
                        return false;
                    }
                }

                // Filter by tags if specified (treat empty array as no filter)
                if let Some(ref tags) = params.tags {
                    if !tags.is_empty() && !tags.iter().any(|tag| component.tags.contains(tag)) {
                        return false;
                    }
                }

                // Filter by search term if specified
                if let Some(ref search) = params.search {
                    let search_lower = search.to_lowercase();
                    if !component.name.to_lowercase().contains(&search_lower)
                        && !component.description.to_lowercase().contains(&search_lower)
                        && !component.tags.iter().any(|tag| tag.to_lowercase().contains(&search_lower))
                    {
                        return false;
                    }
                }

                true
            })
            .map(|component| ComponentMetadata {
                name: component.name.clone(),
                description: component.description.clone(),
                category: component.category.clone(),
                tags: component.tags.clone(),
                has_typescript: component.typescript_definitions.is_some(),
                prop_count: component.props.len(),
                example_count: component.examples.len(),
            })
            .collect();

        // Sort by name for consistent results
        components.sort_by(|a, b| a.name.cmp(&b.name));

        // Apply limit if specified
        if let Some(limit) = params.limit {
            components.truncate(limit);
        }

        tracing::info!("Returning {} components", components.len());
        Ok(Json(ComponentListResponse { components }))
    }

    #[tool(description = "Get detailed information about a specific React component")]
    async fn get_component(
        &self,
        Parameters(params): Parameters<GetComponentRequest>,
    ) -> Result<Json<ReactComponent>, McpError> {
        // First try exact match (for backwards compatibility)
        let component = self.components.get(&params.name)
            .or_else(|| {
                // If exact match fails, try case-insensitive search
                let name_lower = params.name.to_lowercase();
                self.components.iter()
                    .find(|(key, _)| key.to_lowercase() == name_lower)
                    .map(|(_, component)| component)
            })
            .ok_or_else(|| {
                McpError::invalid_params(
                    "Component not found",
                    Some(json!({
                        "component_name": params.name,
                        "available_components": self.components.keys().collect::<Vec<_>>()
                    })),
                )
            })?;

        let mut result = component.clone();

        // Filter examples if requested
        if params.include_examples == Some(false) {
            result.examples.clear();
        }

        // Filter TypeScript definitions if requested
        if params.include_typescript == Some(false) {
            result.typescript_definitions = None;
        }

        Ok(Json(result))
    }


    #[tool(description = "Get documentation for a specific topic")]
    async fn get_documentation(
        &self,
        Parameters(params): Parameters<GetDocumentationRequest>,
    ) -> Result<Json<Documentation>, McpError> {
        let documentation = self
            .documentation
            .get(&params.topic)
            .ok_or_else(|| {
                McpError::invalid_params(
                    "Documentation topic not found",
                    Some(json!({
                        "topic": params.topic,
                        "available_topics": self.documentation.keys().collect::<Vec<_>>()
                    })),
                )
            })?;

        let mut result = documentation.clone();

        // Filter to specific section if requested
        if let Some(ref section_id) = params.section {
            result.sections.retain(|section| &section.id == section_id);
            if result.sections.is_empty() {
                return Err(McpError::invalid_params(
                    "Section not found in topic",
                    Some(json!({
                        "section": section_id,
                        "topic": params.topic,
                        "available_sections": documentation.sections.iter().map(|s| &s.id).collect::<Vec<_>>()
                    })),
                ));
            }
        }

        Ok(Json(result))
    }

}

#[tool_handler]
impl ServerHandler for ReactComponentServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .build(),
            server_info: Implementation {
                name: "react-component-server".to_string(),
                version: "1.0.0".to_string(),
            },
            instructions: Some(
                "This server provides access to a library of React components and their documentation. \
                Use the available tools to list, search, and get detailed information about components. \
                You can also access documentation on various topics related to the component library."
                    .to_string(),
            ),
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        let mut resources = Vec::new();

        // Add component resources
        for component_name in self.components.keys() {
            resources.push(RawResource::new(
                &format!("component://{}", component_name),
                format!("{} Component", component_name),
            ).no_annotation());
        }

        // Add documentation resources
        for doc_topic in self.documentation.keys() {
            resources.push(RawResource::new(
                &format!("docs://{}", doc_topic),
                format!("Documentation: {}", doc_topic),
            ).no_annotation());
        }

        Ok(ListResourcesResult {
            resources,
            next_cursor: None,
        })
    }

    async fn read_resource(
        &self,
        ReadResourceRequestParam { uri }: ReadResourceRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        if let Some(component_name) = uri.strip_prefix("component://") {
            if let Some(component) = self.components.get(component_name) {
                let content = format!(
                    "# {} Component\n\n{}\n\n## Source Code\n\n```tsx\n{}\n```\n\n## Props\n\n{}\n\n## Examples\n\n{}",
                    component.name,
                    component.description,
                    component.source_code,
                    component.props.iter()
                        .map(|prop| format!("- **{}** ({}): {}{}", 
                            prop.name, 
                            prop.prop_type, 
                            prop.description,
                            if prop.required { " *Required*" } else { "" }
                        ))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    component.examples.iter()
                        .map(|example| format!("### {}\n\n{}\n\n```tsx\n{}\n```", 
                            example.title, 
                            example.description, 
                            example.code
                        ))
                        .collect::<Vec<_>>()
                        .join("\n\n")
                );

                return Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(&content, uri)],
                });
            }
        }

        if let Some(doc_topic) = uri.strip_prefix("docs://") {
            if let Some(documentation) = self.documentation.get(doc_topic) {
                let content = format!(
                    "# {}\n\n{}\n\n{}\n\n## Related Components\n\n{}",
                    documentation.title,
                    documentation.content,
                    documentation.sections.iter()
                        .map(|section| format!("## {}\n\n{}\n\n{}", 
                            section.title,
                            section.content,
                            section.code_examples.iter()
                                .map(|code| format!("```\n{}\n```", code))
                                .collect::<Vec<_>>()
                                .join("\n\n")
                        ))
                        .collect::<Vec<_>>()
                        .join("\n\n"),
                    documentation.related_components.join(", ")
                );

                return Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(&content, uri)],
                });
            }
        }

        Err(McpError::resource_not_found(
            "Resource not found",
            Some(json!({ "uri": uri })),
        ))
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        Ok(ListResourceTemplatesResult {
            next_cursor: None,
            resource_templates: Vec::new(),
        })
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        Ok(self.get_info())
    }
}