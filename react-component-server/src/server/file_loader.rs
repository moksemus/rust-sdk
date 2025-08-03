use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use super::types::{ReactComponent, ComponentManifest, ComponentExample, Documentation, DocumentationSection, ComponentFiles, ComponentDependencies};

/// File-based component loader that reads components from the filesystem
pub struct ComponentFileLoader {
    components_dir: PathBuf,
}

impl ComponentFileLoader {
    pub fn new<P: AsRef<Path>>(components_dir: P) -> Self {
        Self {
            components_dir: components_dir.as_ref().to_path_buf(),
        }
    }

    /// Load all components from the components directory
    pub fn load_components(&self) -> Result<HashMap<String, ReactComponent>> {
        let mut components = HashMap::new();

        if !self.components_dir.exists() {
            tracing::error!("Components directory does not exist: {:?}", self.components_dir);
            tracing::info!("Current working directory: {:?}", std::env::current_dir().unwrap_or_default());
            return Ok(components);
        }

        tracing::info!("Found components directory: {:?}", self.components_dir);

        let entries = fs::read_dir(&self.components_dir)
            .with_context(|| format!("Failed to read components directory: {:?}", self.components_dir))?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let component_name = path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                match self.load_component(&path) {
                    Ok(component) => {
                        tracing::info!("Loaded component: {}", component_name);
                        components.insert(component_name, component);
                    }
                    Err(e) => {
                        tracing::error!("Failed to load component {}: {}", component_name, e);
                    }
                }
            }
        }

        tracing::info!("Loaded {} components total", components.len());
        Ok(components)
    }

    /// Load a single component from its directory
    fn load_component(&self, component_dir: &Path) -> Result<ReactComponent> {
        let manifest_path = component_dir.join("manifest.json");
        
        let manifest: ComponentManifest = if manifest_path.exists() {
            let manifest_content = fs::read_to_string(&manifest_path)
                .with_context(|| format!("Failed to read manifest: {:?}", manifest_path))?;
            
            serde_json::from_str(&manifest_content)
                .with_context(|| format!("Failed to parse manifest: {:?}", manifest_path))?
        } else {
            // Create a default manifest if none exists
            self.create_default_manifest(component_dir)?
        };

        // Load component source code
        let component_path = component_dir.join(&manifest.files.component);
        let source_code = fs::read_to_string(&component_path)
            .with_context(|| format!("Failed to read component file: {:?}", component_path))?;

        // Load TypeScript definitions
        let types_path = component_dir.join(&manifest.files.types);
        let typescript_definitions = if types_path.exists() {
            Some(fs::read_to_string(&types_path)
                .with_context(|| format!("Failed to read types file: {:?}", types_path))?)
        } else {
            None
        };

        // Load examples - try manifest list first, then scan examples directory
        let examples = if !manifest.files.examples.is_empty() {
            self.load_examples(component_dir, &manifest.files.examples)?
        } else {
            self.scan_examples_directory(component_dir)?
        };

        Ok(ReactComponent {
            name: manifest.name,
            description: manifest.description,
            source_code,
            props: manifest.props,
            examples,
            category: manifest.category,
            tags: manifest.tags,
            typescript_definitions,
        })
    }

    /// Load examples from the examples directory
    fn load_examples(&self, component_dir: &Path, example_files: &[String]) -> Result<Vec<ComponentExample>> {
        let mut examples = Vec::new();

        for example_file in example_files {
            let example_path = component_dir.join(example_file);
            if example_path.exists() {
                let code = fs::read_to_string(&example_path)
                    .with_context(|| format!("Failed to read example file: {:?}", example_path))?;

                // Extract title and description from the file name and comments
                let title = example_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Example")
                    .replace('_', " ")
                    .split_whitespace()
                    .map(|word| {
                        let mut chars = word.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ");

                // Try to extract description from the first comment block
                let description = self.extract_description_from_code(&code)
                    .unwrap_or_else(|| format!("{} example", title));

                examples.push(ComponentExample {
                    title,
                    description,
                    code,
                    props: HashMap::new(),
                });
            }
        }

        Ok(examples)
    }

    /// Scan examples directory for all .tsx files if no manifest examples list exists
    fn scan_examples_directory(&self, component_dir: &Path) -> Result<Vec<ComponentExample>> {
        let examples_dir = component_dir.join("examples");
        if !examples_dir.exists() {
            return Ok(vec![]);
        }

        let mut examples = Vec::new();
        let entries = fs::read_dir(&examples_dir)
            .with_context(|| format!("Failed to read examples directory: {:?}", examples_dir))?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "tsx" || ext == "ts") {
                if let Some(relative_path) = path.strip_prefix(component_dir).ok() {
                    let example_file = relative_path.to_string_lossy().to_string();
                    
                    let code = fs::read_to_string(&path)
                        .with_context(|| format!("Failed to read example file: {:?}", path))?;

                    let title = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("Example")
                        .replace('_', " ")
                        .split_whitespace()
                        .map(|word| {
                            let mut chars = word.chars();
                            match chars.next() {
                                None => String::new(),
                                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ");

                    let description = self.extract_description_from_code(&code)
                        .unwrap_or_else(|| format!("{} example", title));

                    examples.push(ComponentExample {
                        title,
                        description,
                        code,
                        props: HashMap::new(),
                    });
                }
            }
        }

        Ok(examples)
    }

    /// Extract description from code comments
    fn extract_description_from_code(&self, code: &str) -> Option<String> {
        for line in code.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("/**") || trimmed.starts_with("*") && !trimmed.starts_with("*/") {
                if let Some(desc) = trimmed.strip_prefix("/**").or_else(|| trimmed.strip_prefix("*")) {
                    let cleaned = desc.trim();
                    if !cleaned.is_empty() && !cleaned.starts_with("/") {
                        return Some(cleaned.to_string());
                    }
                }
            }
        }
        None
    }

    /// Create a default manifest for components without one
    fn create_default_manifest(&self, component_dir: &Path) -> Result<ComponentManifest> {
        let component_name = component_dir.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown")
            .to_string();

        // Capitalize first letter for display
        let display_name = component_name.chars()
            .enumerate()
            .map(|(i, c)| if i == 0 { c.to_uppercase().collect::<String>() } else { c.to_string() })
            .collect::<String>();

        Ok(ComponentManifest {
            name: display_name.clone(),
            description: format!("A {} component from Fluent UI", display_name),
            category: "UI".to_string(),
            tags: vec!["component".to_string(), "ui".to_string()],
            version: "1.0.0".to_string(),
            dependencies: ComponentDependencies::List(vec!["@fluentui/react-components".to_string()]),
            files: ComponentFiles {
                component: "component.tsx".to_string(),
                types: "types.ts".to_string(),
                documentation: "README.md".to_string(),
                examples: vec![],
                utils: vec![],
            },
            exports: None,
            props: vec![],
        })
    }

    /// Load documentation from components
    pub fn load_documentation(&self) -> Result<HashMap<String, Documentation>> {
        let mut docs = HashMap::new();

        // For now, return empty documentation
        // This can be expanded to load from README files or dedicated docs
        
        Ok(docs)
    }
}