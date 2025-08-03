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
    #[serde(rename = "type")]
    pub prop_type: String,
    pub required: bool,
    #[serde(rename = "default")]
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

// Component manifest structure for file-based loading
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ComponentManifest {
    pub name: String,
    pub description: String,
    pub category: String,
    pub tags: Vec<String>,
    pub version: String,
    pub dependencies: HashMap<String, String>,
    pub files: ComponentFiles,
    pub exports: ComponentExports,
    pub props: Vec<ComponentProp>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ComponentFiles {
    pub component: String,
    pub types: String,
    pub documentation: String,
    pub examples: Vec<String>,
    pub utils: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ComponentExports {
    pub main: String,
    pub types: String,
}

// Wrapper types to ensure object schemas for MCP inspector compatibility
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ComponentListResponse {
    pub components: Vec<ComponentMetadata>,
}