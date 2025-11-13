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

    #[test]
    fn test_profile_manager_creation() {
        let config = Config::default();
        let result = ProfileManager::new(config);
        assert!(result.is_ok());
    }
}
