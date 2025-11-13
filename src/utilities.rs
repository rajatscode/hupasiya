//! Utility commands (monitor, clean, leave)

use crate::config::Config;
use crate::error::Result;
use crate::models::SessionStatus;
use crate::session::SessionManager;
use colored::Colorize;

/// Utilities manager
pub struct UtilitiesManager {
    session_mgr: SessionManager,
}

impl UtilitiesManager {
    /// Create new utilities manager
    pub fn new(config: Config) -> Result<Self> {
        let session_mgr = SessionManager::new(config)?;
        Ok(Self { session_mgr })
    }

    /// Monitor all sessions (dashboard view)
    pub fn monitor(&self, watch: bool) -> Result<()> {
        loop {
            // Clear screen if watching
            if watch {
                print!("\x1B[2J\x1B[1;1H");
            }

            let sessions = self.session_mgr.list_sessions()?;

            println!();
            println!("{} Session Monitor", "📊".bold());
            println!("─────────────────────────────────────────────");
            println!();

            if sessions.is_empty() {
                println!("  {}", "No sessions found".yellow());
            } else {
                println!(
                    "{:20} {:10} {:15} {:8}",
                    "Name", "Status", "Last Active", "PRs"
                );
                println!("─────────────────────────────────────────────");

                for session in &sessions {
                    let status_str = match session.status {
                        SessionStatus::Active => "Active".green(),
                        SessionStatus::Paused => "Paused".yellow(),
                        SessionStatus::Integrated => "Integrated".blue(),
                        SessionStatus::Archived => "Archived".dimmed(),
                        SessionStatus::Abandoned => "Abandoned".red(),
                    };

                    let last_active = if (chrono::Utc::now() - session.last_active).num_hours() < 24
                    {
                        format!(
                            "{}h ago",
                            (chrono::Utc::now() - session.last_active).num_hours()
                        )
                    } else {
                        format!(
                            "{}d ago",
                            (chrono::Utc::now() - session.last_active).num_days()
                        )
                    };

                    let pr_str = if session.pr_number.is_some() {
                        format!("#{}", session.pr_number.unwrap())
                    } else {
                        "-".to_string()
                    };

                    println!(
                        "{:20} {:10} {:15} {:8}",
                        session.name, status_str, last_active, pr_str
                    );
                }
            }

            println!();
            println!("Total: {} sessions", sessions.len());
            println!();

            if watch {
                println!("Press Ctrl+C to exit...");
                std::thread::sleep(std::time::Duration::from_secs(5));
            } else {
                break;
            }
        }

        Ok(())
    }

    /// Clean up old/archived sessions
    pub fn clean(&self, older_than_days: u64, dry_run: bool, force: bool) -> Result<()> {
        let sessions = self.session_mgr.list_sessions()?;
        let threshold = chrono::Utc::now() - chrono::Duration::days(older_than_days as i64);

        let candidates: Vec<_> = sessions
            .iter()
            .filter(|s| {
                matches!(
                    s.status,
                    SessionStatus::Archived | SessionStatus::Integrated
                ) && s.last_active < threshold
            })
            .collect();

        if candidates.is_empty() {
            println!("{} No sessions to clean", "✓".green());
            return Ok(());
        }

        println!(
            "{} Found {} sessions to clean (older than {} days)",
            "🧹".bold(),
            candidates.len(),
            older_than_days
        );
        println!();

        for session in &candidates {
            println!(
                "  {} - last active {}",
                session.name,
                session.last_active.format("%Y-%m-%d")
            );
        }

        println!();

        if dry_run {
            println!("{} Dry run - no changes made", "ℹ".blue());
            return Ok(());
        }

        if !force {
            println!(
                "{}",
                "Use --force to actually delete these sessions".yellow()
            );
            return Ok(());
        }

        let mut deleted = 0;
        for session in candidates {
            match self.session_mgr.delete_session(&session.name) {
                Ok(_) => {
                    println!("  {} Deleted {}", "✓".green(), session.name);
                    deleted += 1;
                }
                Err(e) => {
                    eprintln!("  {} Failed to delete {}: {}", "✗".red(), session.name, e);
                }
            }
        }

        println!();
        println!("{} Cleaned {} sessions", "✓".green(), deleted);

        Ok(())
    }

    /// Leave a session (graceful cleanup)
    pub fn leave(&self, session_name: &str, archive: bool) -> Result<()> {
        let mut session = self.session_mgr.load_session(session_name)?;

        println!("{} Leaving session '{}'", "→".cyan(), session_name);

        if archive {
            session.status = SessionStatus::Archived;
            session.log_activity(
                crate::models::ActivityType::StatusChanged,
                "Archived".to_string(),
            );
            self.session_mgr.save_session(&session)?;
            println!("  {} Session archived", "✓".green());
        } else {
            session.status = SessionStatus::Paused;
            session.log_activity(
                crate::models::ActivityType::StatusChanged,
                "Paused".to_string(),
            );
            self.session_mgr.save_session(&session)?;
            println!("  {} Session paused", "✓".green());
        }

        println!();
        println!("Session '{}' can be resumed later with:", session_name);
        println!("  hp switch {}", session_name);
        println!("  hp launch {}", session_name);
        println!();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utilities_manager_creation() {
        let config = Config::default();
        match UtilitiesManager::new(config) {
            Ok(_) => {
                // Successfully created
            }
            Err(crate::error::Error::HnNotFound) => {
                println!("Skipping: hn not installed");
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }
}
