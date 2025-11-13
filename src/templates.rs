//! Template marketplace for sharing context templates

use crate::config::Config;
use crate::error::{Error, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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

/// Template marketplace manager
pub struct TemplateManager {
    config: Config,
    templates_dir: PathBuf,
}

impl TemplateManager {
    /// Create new template manager
    pub fn new(config: Config) -> Result<Self> {
        let templates_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("hupasiya")
            .join("templates");
        fs::create_dir_all(&templates_dir)?;

        Ok(Self {
            config,
            templates_dir,
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

        // For now, just copy from local file
        let source_path = PathBuf::from(source);
        if !source_path.exists() {
            return Err(Error::Other(format!(
                "Template source not found: {}",
                source
            )));
        }

        let template_name = name.unwrap_or_else(|| {
            source_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string()
        });

        let dest_path = self.templates_dir.join(format!("{}.md", template_name));
        fs::copy(&source_path, &dest_path)?;

        println!("{} Template installed: {}", "✓".green(), template_name);
        println!("   Path: {}", dest_path.display());
        println!();

        Ok(())
    }

    /// Publish template (stub for future implementation)
    pub fn publish(&self, template_name: &str) -> Result<()> {
        let template_path = self.templates_dir.join(format!("{}.md", template_name));

        if !template_path.exists() {
            return Err(Error::TemplateNotFound(template_name.to_string()));
        }

        println!("{} Publishing template: {}", "→".cyan(), template_name);
        println!();
        println!("{}", "Publishing is not yet implemented.".yellow());
        println!("Future: Templates will be published to a central registry.");
        println!();
        println!("For now, you can share templates by:");
        println!("  1. Copying the file: {}", template_path.display());
        println!("  2. Sharing via git repository");
        println!("  3. Using a shared filesystem");
        println!();

        Ok(())
    }

    /// Search templates (stub)
    pub fn search(&self, query: &str) -> Result<()> {
        println!("{} Searching for: {}", "🔍".bold(), query);
        println!();

        let custom_templates = self.list_custom_templates()?;
        let results: Vec<_> = custom_templates
            .iter()
            .filter(|t| {
                t.name.to_lowercase().contains(&query.to_lowercase())
                    || t.description.to_lowercase().contains(&query.to_lowercase())
                    || t.tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query.to_lowercase()))
            })
            .collect();

        if results.is_empty() {
            println!("  {}", "No templates found".yellow());
        } else {
            for meta in results {
                println!("  {} - {}", meta.name.cyan().bold(), meta.description);
                println!("    Author: {} | Version: {}", meta.author, meta.version);
                if !meta.tags.is_empty() {
                    println!("    Tags: {}", meta.tags.join(", "));
                }
                println!();
            }
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
                    let name = path.file_stem().unwrap().to_string_lossy().to_string();

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

    #[test]
    fn test_template_manager_creation() {
        let config = Config::default();
        let result = TemplateManager::new(config);
        assert!(result.is_ok());
    }
}
