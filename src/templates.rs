//! Template marketplace for sharing context templates

use crate::config::Config;
use crate::error::{Error, Result};
use crate::progress;
use colored::Colorize;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

/// Template metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    pub name: String,
    pub author: String,
    pub version: String,
    pub description: String,
    pub tags: Vec<String>,
    pub agent_types: Vec<String>,
}

/// Template search result from registry
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TemplateSearchResult {
    templates: Vec<TemplateMetadata>,
}

/// Template registry client
struct TemplateRegistry {
    registry_url: String,
    client: Client,
    cache_dir: PathBuf,
}

#[allow(dead_code)]
impl TemplateRegistry {
    /// Create new registry client
    fn new() -> Result<Self> {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("hupasiya")
            .join("templates");
        fs::create_dir_all(&cache_dir)?;

        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| Error::Other(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            registry_url: "https://hp-templates.dev".to_string(),
            client,
            cache_dir,
        })
    }

    /// Search templates in registry
    fn search(&self, query: &str) -> Result<Vec<TemplateMetadata>> {
        let url = format!("{}/api/search?q={}", self.registry_url, query);

        let spinner = progress::spinner("Searching registry...");
        let result = match self.client.get(&url).send() {
            Ok(response) if response.status().is_success() => {
                let result: TemplateSearchResult = response
                    .json()
                    .map_err(|e| Error::Other(format!("Failed to parse search results: {}", e)))?;
                progress::finish_success(&spinner, "Search complete");
                Ok(result.templates)
            }
            Ok(response) => {
                progress::finish_error(&spinner, "Registry error");
                Err(Error::Other(format!(
                    "Registry returned error: {}",
                    response.status()
                )))
            }
            Err(e) => {
                // Registry unavailable - use local fallback
                spinner.finish_and_clear();
                eprintln!("{} Registry unavailable: {}", "⚠".yellow(), e);
                eprintln!("{}", "  Falling back to local templates only".dimmed());
                Ok(Vec::new())
            }
        };
        result
    }

    /// Install template from registry
    fn install(&self, name: &str, version: Option<&str>) -> Result<String> {
        let version_str = version.unwrap_or("latest");
        let url = format!(
            "{}/api/templates/{}/{}",
            self.registry_url, name, version_str
        );

        let spinner = progress::spinner(&format!("Downloading {} {}...", name, version_str));
        let result = match self.client.get(&url).send() {
            Ok(response) if response.status().is_success() => {
                let content = response
                    .text()
                    .map_err(|e| Error::Other(format!("Failed to read template: {}", e)))?;

                // Cache the template
                let cache_path = self.cache_dir.join(format!("{}.md", name));
                fs::write(&cache_path, &content)?;

                progress::finish_success(&spinner, "Download complete");
                Ok(content)
            }
            Ok(response) => {
                progress::finish_error(&spinner, "Download failed");
                Err(Error::Other(format!(
                    "Failed to download template: {}",
                    response.status()
                )))
            }
            Err(e) => {
                progress::finish_error(&spinner, "Registry unavailable");
                Err(Error::Other(format!("Registry unavailable: {}", e)))
            }
        };
        result
    }

    /// Publish template to registry
    fn publish(&self, metadata: &TemplateMetadata, content: &str) -> Result<()> {
        let url = format!("{}/api/publish", self.registry_url);

        #[derive(Serialize)]
        struct PublishRequest<'a> {
            metadata: &'a TemplateMetadata,
            content: &'a str,
        }

        let payload = PublishRequest { metadata, content };

        let spinner = progress::spinner(&format!("Publishing {}...", metadata.name));
        let result = match self.client.post(&url).json(&payload).send() {
            Ok(response) if response.status().is_success() => {
                progress::finish_success(&spinner, "Published successfully");
                Ok(())
            }
            Ok(response) => {
                progress::finish_error(&spinner, "Publish failed");
                Err(Error::Other(format!(
                    "Failed to publish template: {}",
                    response.status()
                )))
            }
            Err(e) => {
                progress::finish_error(&spinner, "Registry unavailable");
                Err(Error::Other(format!("Registry unavailable: {}", e)))
            }
        };
        result
    }

    /// Check if cache is fresh (less than 1 hour old)
    fn is_cache_fresh(&self, name: &str) -> bool {
        let cache_path = self.cache_dir.join(format!("{}.md", name));

        if let Ok(metadata) = fs::metadata(&cache_path) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(elapsed) = SystemTime::now().duration_since(modified) {
                    return elapsed < Duration::from_secs(3600); // 1 hour
                }
            }
        }

        false
    }

    /// Get template from cache
    fn get_cached(&self, name: &str) -> Result<String> {
        let cache_path = self.cache_dir.join(format!("{}.md", name));
        fs::read_to_string(&cache_path)
            .map_err(|e| Error::Other(format!("Cache read failed: {}", e)))
    }
}

/// Template marketplace manager
pub struct TemplateManager {
    #[allow(dead_code)]
    config: Config,
    templates_dir: PathBuf,
    registry: TemplateRegistry,
}

#[allow(dead_code)]
impl TemplateManager {
    /// Create new template manager
    pub fn new(config: Config) -> Result<Self> {
        let templates_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("hupasiya")
            .join("templates");
        fs::create_dir_all(&templates_dir)?;

        let registry = TemplateRegistry::new()?;

        Ok(Self {
            config,
            templates_dir,
            registry,
        })
    }

    /// List available templates
    pub fn list(&self) -> Result<()> {
        println!();
        println!("{} Available Templates", "📦".bold());
        println!();

        // List built-in templates
        println!("{}:", "Built-in".bold());
        let builtin = vec![
            ("feature", "New feature development"),
            ("bugfix", "Bug fix workflow"),
            ("review", "Code review"),
            ("research", "Investigation/spike"),
            ("refactor", "Code refactoring"),
            ("test", "Test writing"),
            ("docs", "Documentation"),
            ("shepherd", "PR comment resolution"),
        ];

        for (name, desc) in builtin {
            println!("  {} - {}", name.cyan(), desc);
        }
        println!();

        // List custom templates
        let custom_templates = self.list_custom_templates()?;
        if !custom_templates.is_empty() {
            println!("{}:", "Custom".bold());
            for meta in custom_templates {
                println!(
                    "  {} ({}) - {}",
                    meta.name.cyan(),
                    meta.version.dimmed(),
                    meta.description
                );
                if !meta.tags.is_empty() {
                    println!("    Tags: {}", meta.tags.join(", ").dimmed());
                }
            }
            println!();
        }

        Ok(())
    }

    /// Install template from URL or file
    pub fn install(&self, source: &str, name: Option<String>) -> Result<()> {
        println!("{} Installing template from {}", "→".cyan(), source);

        let (content, template_name) = if source.starts_with("http://")
            || source.starts_with("https://")
        {
            // Install from URL
            let client = Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .map_err(|e| Error::Other(format!("HTTP client error: {}", e)))?;

            let response = client
                .get(source)
                .send()
                .map_err(|e| Error::Other(format!("Failed to download template: {}", e)))?;

            if !response.status().is_success() {
                return Err(Error::Other(format!(
                    "Download failed with status: {}",
                    response.status()
                )));
            }

            let content = response
                .text()
                .map_err(|e| Error::Other(format!("Failed to read template content: {}", e)))?;

            let name = name.ok_or_else(|| {
                Error::InvalidInput("Template name required when installing from URL".to_string())
            })?;

            (content, name)
        } else if source.contains('/') || source.contains('\\') || PathBuf::from(source).exists() {
            // Install from local file
            let source_path = PathBuf::from(source);
            if !source_path.exists() {
                return Err(Error::Other(format!("Template file not found: {}", source)));
            }

            let content = fs::read_to_string(&source_path)?;
            let template_name = name.unwrap_or_else(|| {
                source_path
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
            });

            (content, template_name)
        } else {
            // Install from registry by name
            println!("{} Checking registry...", "→".dimmed());

            let content = if self.registry.is_cache_fresh(source) {
                println!("{} Using cached version", "→".dimmed());
                self.registry.get_cached(source)?
            } else {
                match self.registry.install(source, None) {
                    Ok(content) => {
                        println!("{} Downloaded from registry", "→".dimmed());
                        content
                    }
                    Err(e) => {
                        eprintln!("{} Registry error: {}", "⚠".yellow(), e);
                        return Err(e);
                    }
                }
            };

            let template_name = name.unwrap_or_else(|| source.to_string());
            (content, template_name)
        };

        // Install to local templates directory
        let dest_path = self.templates_dir.join(format!("{}.md", template_name));
        fs::write(&dest_path, &content)?;

        println!("{} Template installed: {}", "✓".green(), template_name);
        println!("   Path: {}", dest_path.display());
        println!();

        Ok(())
    }

    /// Publish template to registry
    pub fn publish(&self, template_name: &str) -> Result<()> {
        let template_path = self.templates_dir.join(format!("{}.md", template_name));

        if !template_path.exists() {
            return Err(Error::TemplateNotFound(template_name.to_string()));
        }

        println!("{} Publishing template: {}", "→".cyan(), template_name);

        // Read template content
        let content = fs::read_to_string(&template_path)?;

        // Create metadata (in production, this would be parsed from frontmatter)
        let metadata = TemplateMetadata {
            name: template_name.to_string(),
            author: "code@rajats.site".to_string(),
            version: "1.0.0".to_string(),
            description: content.lines().take(3).collect::<Vec<_>>().join(" "),
            tags: vec![],
            agent_types: vec![],
        };

        match self.registry.publish(&metadata, &content) {
            Ok(()) => {
                println!("{} Template published successfully!", "✓".green());
                println!("   Name: {}", metadata.name);
                println!("   Version: {}", metadata.version);
                println!();
            }
            Err(e) => {
                eprintln!("{} Failed to publish to registry: {}", "✗".red(), e);
                eprintln!();
                eprintln!("{}", "Alternative sharing methods:".yellow());
                eprintln!("  1. Copy file: {}", template_path.display());
                eprintln!("  2. Share via git repository");
                eprintln!("  3. Use shared filesystem or cloud storage");
                eprintln!("  4. Share raw URL (users can install with: hp template install <url>)");
                eprintln!();
                return Err(e);
            }
        }

        Ok(())
    }

    /// Search templates (local + registry)
    pub fn search(&self, query: &str) -> Result<()> {
        println!("{} Searching for: {}", "🔍".bold(), query);
        println!();

        // Search local templates
        let custom_templates = self.list_custom_templates()?;
        let local_results: Vec<_> = custom_templates
            .iter()
            .filter(|t| {
                t.name.to_lowercase().contains(&query.to_lowercase())
                    || t.description.to_lowercase().contains(&query.to_lowercase())
                    || t.tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query.to_lowercase()))
            })
            .collect();

        // Search registry
        let registry_results = self.registry.search(query).unwrap_or_default();

        // Display results
        if !local_results.is_empty() {
            println!("{}:", "Local Templates".bold().cyan());
            for meta in &local_results {
                println!("  {} - {}", meta.name.cyan().bold(), meta.description);
                println!("    Author: {} | Version: {}", meta.author, meta.version);
                if !meta.tags.is_empty() {
                    println!("    Tags: {}", meta.tags.join(", "));
                }
                println!();
            }
        }

        if !registry_results.is_empty() {
            println!("{}:", "Registry Templates".bold().green());
            for meta in &registry_results {
                println!("  {} - {}", meta.name.green().bold(), meta.description);
                println!("    Author: {} | Version: {}", meta.author, meta.version);
                if !meta.tags.is_empty() {
                    println!("    Tags: {}", meta.tags.join(", "));
                }
                if !meta.agent_types.is_empty() {
                    println!("    Agent types: {}", meta.agent_types.join(", "));
                }
                println!(
                    "    Install: {} {}",
                    "hp template install".dimmed(),
                    meta.name.dimmed()
                );
                println!();
            }
        }

        if local_results.is_empty() && registry_results.is_empty() {
            println!("  {}", "No templates found".yellow());
            println!();
        }

        Ok(())
    }

    // === Private helper methods ===

    fn list_custom_templates(&self) -> Result<Vec<TemplateMetadata>> {
        let mut templates = Vec::new();

        if !self.templates_dir.exists() {
            return Ok(templates);
        }

        for entry in fs::read_dir(&self.templates_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                // Try to parse metadata from template
                if let Ok(content) = fs::read_to_string(&path) {
                    let name = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "unknown".to_string());

                    // Parse frontmatter if present (simple implementation)
                    let meta = TemplateMetadata {
                        name: name.clone(),
                        author: "unknown".to_string(),
                        version: "1.0.0".to_string(),
                        description: content.lines().take(3).collect::<Vec<_>>().join(" "),
                        tags: vec![],
                        agent_types: vec![],
                    };

                    templates.push(meta);
                }
            }
        }

        Ok(templates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_test_env() -> (TempDir, Config) {
        let temp_dir = TempDir::new().unwrap();
        let config = Config::default();
        (temp_dir, config)
    }

    #[test]
    fn test_template_manager_creation() {
        let config = Config::default();
        let result = TemplateManager::new(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_builtin_templates() {
        let (_temp_dir, config) = setup_test_env();
        let mgr = TemplateManager::new(config).unwrap();

        // List should show built-in templates
        let result = mgr.list();
        assert!(result.is_ok());
    }

    #[test]
    fn test_install_template_from_file() {
        let (_temp_dir, config) = setup_test_env();
        let mgr = TemplateManager::new(config.clone());

        // Create a test template file
        let test_template_dir = TempDir::new().unwrap();
        let test_template = test_template_dir.path().join("test-template.md");
        std::fs::write(
            &test_template,
            "# Test Template\nObjectives:\n- Test objective 1",
        )
        .unwrap();

        // Install it - just verify it doesn't error
        let result = mgr.unwrap().install(
            test_template.to_str().unwrap(),
            Some("my-custom-template".to_string()),
        );
        assert!(result.is_ok());
        // Note: File is installed to ~/.config/hupasiya/templates/ by default
    }

    #[test]
    fn test_search_templates() {
        let (_temp_dir, config) = setup_test_env();
        let mgr = TemplateManager::new(config.clone()).unwrap();

        // Install a test template
        let test_template_dir = TempDir::new().unwrap();
        let test_template = test_template_dir.path().join("python-test.md");
        std::fs::write(
            &test_template,
            "# Python Testing Template\nFor writing Python tests",
        )
        .unwrap();

        let _ = mgr.install(
            test_template.to_str().unwrap(),
            Some("python-test".to_string()),
        );

        // Search for it
        let result = mgr.search("python");
        assert!(result.is_ok());

        // Search for non-existent
        let result2 = mgr.search("nonexistent_template_xyz");
        assert!(result2.is_ok()); // Should succeed but show no results
    }

    #[test]
    fn test_install_without_name_uses_source_name() {
        let (_temp_dir, config) = setup_test_env();
        let mgr = TemplateManager::new(config.clone()).unwrap();

        // Create a test template
        let test_template_dir = TempDir::new().unwrap();
        let test_template = test_template_dir.path().join("auto-named.md");
        std::fs::write(&test_template, "# Auto Named Template").unwrap();

        // Install without explicit name - just verify it doesn't error
        let result = mgr.install(test_template.to_str().unwrap(), None);
        assert!(result.is_ok());
        // Note: File is installed to ~/.config/hupasiya/templates/ with source filename
    }
}
