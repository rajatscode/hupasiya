//! Activity tracking and metrics reporting

use crate::config::Config;
use crate::error::Result;
use crate::models::ActivityType;
use crate::session::SessionManager;
use colored::Colorize;

/// Activity and metrics manager
pub struct ActivityManager {
    session_mgr: SessionManager,
}

impl ActivityManager {
    /// Create new activity manager
    pub fn new(config: Config) -> Result<Self> {
        let session_mgr = SessionManager::new(config)?;
        Ok(Self { session_mgr })
    }

    /// Show activity log for a session
    pub fn show_activity(&self, session_name: &str, limit: Option<usize>) -> Result<()> {
        let session = self.session_mgr.load_session(session_name)?;

        println!();
        println!("{} Activity Log: {}", "📋".bold(), session.name.bold());
        println!();

        let activities: Vec<_> = session
            .activity_log
            .iter()
            .rev()
            .take(limit.unwrap_or(usize::MAX))
            .collect();

        if activities.is_empty() {
            println!("  {}", "No activities recorded".yellow());
        } else {
            for activity in activities {
                let icon = match activity.event_type {
                    ActivityType::SessionCreated => "🆕",
                    ActivityType::ContextEdited => "📝",
                    ActivityType::AiLaunched => "🚀",
                    ActivityType::CommitMade => "💾",
                    ActivityType::PrCreated => "🔗",
                    ActivityType::PrSynced => "🔄",
                    ActivityType::ShepherdRun => "🐕",
                    ActivityType::Cascaded => "⬇️",
                    ActivityType::Gathered => "⬆️",
                    ActivityType::Integrated => "✅",
                    _ => "•",
                };

                println!(
                    "  {} {} - {}",
                    icon,
                    activity
                        .timestamp
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string()
                        .dimmed(),
                    activity.details
                );
            }
        }

        println!();
        Ok(())
    }

    /// Show metrics for a session
    pub fn show_metrics(&self, session_name: &str) -> Result<()> {
        let session = self.session_mgr.load_session(session_name)?;

        println!();
        println!("{} Metrics: {}", "📊".bold(), session.name.bold());
        println!();

        println!("  AI Interactions: {}", session.metrics.ai_interactions);
        println!("  Commits: {}", session.metrics.commits);
        println!("  Lines added: {}", session.metrics.lines_added);
        println!("  Lines removed: {}", session.metrics.lines_removed);
        println!("  Files changed: {}", session.metrics.files_changed);
        println!("  Tokens used: {}", session.metrics.tokens_used);
        println!();

        println!("Time Tracking:");
        let duration = chrono::Utc::now() - session.created;
        println!("  Session age: {} days", duration.num_days());
        println!(
            "  Total time: {} hours",
            session.metrics.total_time_secs / 3600
        );
        println!(
            "  Last active: {}",
            session.last_active.format("%Y-%m-%d %H:%M:%S")
        );
        println!();

        Ok(())
    }

    /// Show aggregated stats across all sessions
    pub fn show_stats(&self) -> Result<()> {
        let sessions = self.session_mgr.list_sessions()?;

        println!();
        println!("{} Global Statistics", "📊".bold());
        println!();

        println!("Total sessions: {}", sessions.len());

        let active_count = sessions
            .iter()
            .filter(|s| matches!(s.status, crate::models::SessionStatus::Active))
            .count();
        println!("  Active: {}", active_count);

        let paused_count = sessions
            .iter()
            .filter(|s| matches!(s.status, crate::models::SessionStatus::Paused))
            .count();
        println!("  Paused: {}", paused_count);

        let integrated_count = sessions
            .iter()
            .filter(|s| matches!(s.status, crate::models::SessionStatus::Integrated))
            .count();
        println!("  Integrated: {}", integrated_count);

        println!();
        println!("Activity:");
        let total_ai_interactions: u32 = sessions.iter().map(|s| s.metrics.ai_interactions).sum();
        let total_commits: u32 = sessions.iter().map(|s| s.metrics.commits).sum();
        let total_lines: u32 = sessions.iter().map(|s| s.metrics.lines_added).sum();

        println!("  Total AI interactions: {}", total_ai_interactions);
        println!("  Total commits: {}", total_commits);
        println!("  Total lines added: {}", total_lines);
        println!();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activity_manager_creation() {
        let config = Config::default();
        match ActivityManager::new(config) {
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
