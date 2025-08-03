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
    #[serde(alias = "default_value", rename = "default")]
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
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub dependencies: ComponentDependencies,
    pub files: ComponentFiles,
    #[serde(default)]
    pub exports: Option<ComponentExports>,
    #[serde(default)]
    pub props: Vec<ComponentProp>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ComponentFiles {
    #[serde(default = "default_component_file")]
    pub component: String,
    #[serde(default = "default_types_file")]
    pub types: String,
    #[serde(default = "default_documentation")]
    pub documentation: String,
    #[serde(default)]
    pub examples: Vec<String>,
    #[serde(default)]
    pub utils: Vec<String>,
}

impl Default for ComponentFiles {
    fn default() -> Self {
        Self {
            component: default_component_file(),
            types: default_types_file(),
            documentation: default_documentation(),
            examples: vec![],
            utils: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ComponentExports {
    pub main: String,
    pub types: String,
}

// Allow both HashMap and Vec format for dependencies
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ComponentDependencies {
    Map(HashMap<String, String>),
    List(Vec<String>),
}

impl Default for ComponentDependencies {
    fn default() -> Self {
        ComponentDependencies::Map(HashMap::new())
    }
}

fn default_version() -> String {
    "1.0.0".to_string()
}

fn default_component_file() -> String {
    "component.tsx".to_string()
}

fn default_types_file() -> String {
    "types.ts".to_string()
}

fn default_documentation() -> String {
    "README.md".to_string()
}

// Wrapper types to ensure object schemas for MCP inspector compatibility
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ComponentListResponse {
    pub components: Vec<ComponentMetadata>,
}