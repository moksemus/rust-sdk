use serde::{Deserialize, Serialize};
use rmcp::schemars::{self, JsonSchema};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReactComponent {
    pub name: String,
    pub description: String,
    pub source_code: String,
    pub props: Vec<ComponentProp>,
    pub examples: Vec<ComponentExample>,
    pub category: String,
    pub tags: Vec<String>,
    pub typescript_definitions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ComponentProp {
    pub name: String,
    pub prop_type: String,
    pub required: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub default_value: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ComponentExample {
    pub title: String,
    pub description: String,
    pub code: String,
    pub props: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ComponentMetadata {
    pub name: String,
    pub description: String,
    pub category: String,
    pub tags: Vec<String>,
    pub has_typescript: bool,
    pub prop_count: usize,
    pub example_count: usize,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListComponentsRequest {
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub search: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetComponentRequest {
    pub name: String,
    pub include_examples: Option<bool>,
    pub include_typescript: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchComponentsRequest {
    pub query: String,
    pub categories: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetDocumentationRequest {
    pub topic: String,
    pub section: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Documentation {
    pub topic: String,
    pub title: String,
    pub content: String,
    pub sections: Vec<DocumentationSection>,
    pub examples: Vec<String>,
    pub related_components: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DocumentationSection {
    pub id: String,
    pub title: String,
    pub content: String,
    pub code_examples: Vec<String>,
}

// Wrapper types to ensure object schemas for MCP inspector compatibility
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ComponentListResponse {
    pub components: Vec<ComponentMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DocumentationTopicsResponse {
    pub topics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ComponentTagsResponse {
    pub tags: Vec<String>,
}