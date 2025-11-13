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
    use crate::models::{ActivityEvent, ActivityType, AgentType, Session};
    use tempfile::TempDir;

    fn setup_test_env() -> (TempDir, Config) {
        let temp_dir = TempDir::new().unwrap();
        let mut config = Config::default();
        config.hp.sessions.metadata_dir = temp_dir.path().join("sessions");
        config.hp.sessions.context_dir = temp_dir.path().join("contexts");
        std::fs::create_dir_all(&config.hp.sessions.metadata_dir).unwrap();
        (temp_dir, config)
    }

    fn create_test_session(name: &str, agent_type: AgentType) -> Session {
        Session::new(
            name.to_string(),
            agent_type,
            format!("wb-{}", name),
            std::path::PathBuf::from(format!("/tmp/{}", name)),
            "main".to_string(),
            "main".to_string(),
            "test-repo".to_string(),
            "git".to_string(),
        )
    }

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

    #[test]
    fn test_show_activity_with_activities() {
        let (_temp_dir, config) = setup_test_env();
        let session_mgr = crate::session::SessionManager::new(config.clone());

        match session_mgr {
            Ok(mgr) => {
                // Create a test session with activities
                let mut session = create_test_session("test-activity", AgentType::Feature);

                // Add some activities
                session.log_activity(ActivityType::SessionCreated, "Session created".to_string());
                session.log_activity(ActivityType::AiLaunched, "AI tool launched".to_string());
                session.log_activity(ActivityType::Cascaded, "Changes cascaded".to_string());

                // Save session
                let _ = mgr.save_session(&session);

                // Now test show_activity
                let activity_mgr = ActivityManager::new(config).unwrap();
                let result = activity_mgr.show_activity("test-activity", None);

                // Should succeed (we're not checking console output, just that it doesn't error)
                assert!(result.is_ok());

                // Test with limit
                let result_limited = activity_mgr.show_activity("test-activity", Some(2));
                assert!(result_limited.is_ok());
            }
            Err(crate::error::Error::HnNotFound) => {
                println!("Skipping: hn not installed");
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }

    #[test]
    fn test_show_stats_aggregation() {
        let (_temp_dir, config) = setup_test_env();
        let session_mgr = crate::session::SessionManager::new(config.clone());

        match session_mgr {
            Ok(mgr) => {
                // Create multiple sessions with different metrics
                let mut session1 = create_test_session("session1", AgentType::Feature);
                session1.metrics.ai_interactions = 5;
                session1.metrics.commits = 3;
                session1.metrics.lines_added = 100;

                let mut session2 = create_test_session("session2", AgentType::Bugfix);
                session2.metrics.ai_interactions = 10;
                session2.metrics.commits = 7;
                session2.metrics.lines_added = 250;
                session2.status = crate::models::SessionStatus::Integrated;

                let _ = mgr.save_session(&session1);
                let _ = mgr.save_session(&session2);

                // Test show_stats
                let activity_mgr = ActivityManager::new(config).unwrap();
                let result = activity_mgr.show_stats();

                // Should succeed and aggregate metrics
                assert!(result.is_ok());
            }
            Err(crate::error::Error::HnNotFound) => {
                println!("Skipping: hn not installed");
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }

    #[test]
    fn test_activity_with_nonexistent_session() {
        let (_temp_dir, config) = setup_test_env();

        match ActivityManager::new(config) {
            Ok(activity_mgr) => {
                let result = activity_mgr.show_activity("nonexistent", None);
                // Should error for nonexistent session
                assert!(result.is_err());
            }
            Err(crate::error::Error::HnNotFound) => {
                println!("Skipping: hn not installed");
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }
}
