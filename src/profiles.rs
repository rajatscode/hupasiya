//! Configuration profiles management

use crate::config::Config;
use crate::error::{Error, Result};
use colored::Colorize;

/// Profile manager
pub struct ProfileManager {
    config: Config,
}

impl ProfileManager {
    /// Create new profile manager
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self { config })
    }

    /// List available profiles
    pub fn list(&self) -> Result<()> {
        println!();
        println!("{} Configuration Profiles", "⚙️".bold());
        println!();

        if self.config.hp.profiles.is_empty() {
            println!("  {}", "No custom profiles configured".yellow());
            println!();
            println!("Add profiles in ~/.config/hupasiya/config.toml:");
            println!();
            println!("[hp.profiles.fast]");
            println!("[hp.profiles.fast.ai_tool]");
            println!("command = \"claude-fast\"");
            println!();
        } else {
            for (name, profile) in &self.config.hp.profiles {
                println!("  {}", name.cyan().bold());

                if let Some(ai_config) = &profile.ai_tool {
                    println!("    AI Tool: {}", ai_config.command);
                }

                println!();
            }
        }

        Ok(())
    }

    /// Show profile details
    pub fn show(&self, profile_name: &str) -> Result<()> {
        let profile = self
            .config
            .hp
            .profiles
            .get(profile_name)
            .ok_or_else(|| Error::ProfileNotFound(profile_name.to_string()))?;

        println!();
        println!("{} Profile: {}", "⚙️".bold(), profile_name.bold());
        println!();

        if let Some(ai_config) = &profile.ai_tool {
            println!("{}:", "AI Tool".cyan());
            println!("  Command: {}", ai_config.command);
            println!("  Launch method: {:?}", ai_config.launch_method);
            println!("  Context strategy: {:?}", ai_config.context_strategy);
            if !ai_config.env.is_empty() {
                println!("  Environment variables:");
                for (key, value) in &ai_config.env {
                    println!("    {}: {}", key, value);
                }
            }
            println!();
        }

        Ok(())
    }

    /// Use a profile (set as default) - stub
    pub fn use_profile(&self, profile_name: &str) -> Result<()> {
        if !self.config.hp.profiles.contains_key(profile_name) {
            return Err(Error::ProfileNotFound(profile_name.to_string()));
        }

        println!("{} Using profile: {}", "→".cyan(), profile_name);
        println!();
        println!(
            "{}",
            "Profile switching not yet fully implemented.".yellow()
        );
        println!(
            "To use this profile, pass --profile={} to commands.",
            profile_name
        );
        println!();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AiToolConfig, ContextStrategy, LaunchMethod, ProfileConfig};
    use std::collections::HashMap;

    #[test]
    fn test_profile_manager_creation() {
        let config = Config::default();
        let result = ProfileManager::new(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_profiles() {
        let mut config = Config::default();

        // Add some test profiles
        let mut profiles = HashMap::new();
        profiles.insert(
            "fast".to_string(),
            ProfileConfig {
                hn: None,
                ai_tool: Some(AiToolConfig {
                    command: "claude-code".to_string(),
                    launch_method: LaunchMethod::Exec,
                    context_strategy: ContextStrategy::SlashCommand,
                    extra_args: vec![],
                    env: HashMap::new(),
                }),
                pr: None,
                orchestration: None,
            },
        );
        profiles.insert(
            "deep".to_string(),
            ProfileConfig {
                hn: None,
                ai_tool: Some(AiToolConfig {
                    command: "cursor".to_string(),
                    launch_method: LaunchMethod::Tmux,
                    context_strategy: ContextStrategy::File,
                    extra_args: vec![],
                    env: HashMap::new(),
                }),
                pr: None,
                orchestration: None,
            },
        );
        config.hp.profiles = profiles;

        let mgr = ProfileManager::new(config).unwrap();
        let result = mgr.list();
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_no_profiles() {
        let config = Config::default();
        let mgr = ProfileManager::new(config).unwrap();

        // Should succeed even with no profiles
        let result = mgr.list();
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_profile() {
        let mut config = Config::default();

        // Add a test profile
        let mut profiles = HashMap::new();
        profiles.insert(
            "test-profile".to_string(),
            ProfileConfig {
                hn: None,
                ai_tool: Some(AiToolConfig {
                    command: "test-tool".to_string(),
                    launch_method: LaunchMethod::Exec,
                    context_strategy: ContextStrategy::Env,
                    extra_args: vec![],
                    env: HashMap::from([("TEST_VAR".to_string(), "test_value".to_string())]),
                }),
                pr: None,
                orchestration: None,
            },
        );
        config.hp.profiles = profiles;

        let mgr = ProfileManager::new(config).unwrap();
        let result = mgr.show("test-profile");
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_nonexistent_profile() {
        let config = Config::default();
        let mgr = ProfileManager::new(config).unwrap();

        let result = mgr.show("nonexistent");
        assert!(result.is_err());
    }
}
