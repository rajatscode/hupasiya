//! Shepherd workflow for PR comment resolution
//!
//! Interactive workflow to address PR review comments with AI assistance.

use crate::config::Config;
use crate::context::ContextManager;
use crate::error::{Error, Result};
use crate::models::{ActivityType, ConfidenceLevel, Session, ShepherdAction};
use crate::pr::PrManager;
use crate::session::SessionManager;
use colored::Colorize;
use dialoguer::{Confirm, Input, Select};
use std::fs;

/// Shepherd workflow manager
pub struct Shepherd {
    config: Config,
    session_mgr: SessionManager,
    context_mgr: ContextManager,
    pr_mgr: PrManager,
}

impl Shepherd {
    /// Create new shepherd workflow manager
    pub fn new(config: Config) -> Result<Self> {
        let session_mgr = SessionManager::new(config.clone())?;
        let context_mgr = ContextManager::new(config.clone())?;
        let pr_mgr = PrManager::new(config.clone())?;

        Ok(Self {
            config,
            session_mgr,
            context_mgr,
            pr_mgr,
        })
    }

    /// Run interactive shepherd workflow
    pub fn run_interactive(&self, session_name: &str) -> Result<()> {
        let mut session = self.session_mgr.load_session(session_name)?;

        // Check if session has PR
        if session.pr_number.is_none() {
            return Err(Error::Other(
                "Session has no associated PR. Create one first with 'hp pr create'".to_string(),
            ));
        }

        // Check if shepherd.md exists
        let shepherd_path = session.context_dir.join("shepherd.md");
        if !shepherd_path.exists() {
            return Err(Error::Other(
                "No shepherd.md found. Run 'hp pr sync --shepherd' first".to_string(),
            ));
        }

        println!();
        println!(
            "{} {} {}",
            "🐕".bold(),
            "Shepherd Workflow".bold(),
            "🐕".bold()
        );
        println!();
        println!("Session: {}", session.name.bold());
        println!("PR: #{}", session.pr_number.unwrap().to_string().bold());
        println!("Unresolved comments: {}", session.unresolved_comments.len());
        println!();

        if session.unresolved_comments.is_empty() {
            println!("{} No unresolved comments!", "✓".green());
            return Ok(());
        }

        let mut resolved_count = 0;
        let total_comments = session.unresolved_comments.len();

        // Process each comment interactively
        for (i, comment) in session.unresolved_comments.clone().iter().enumerate() {
            println!("─────────────────────────────────────────────");
            println!(
                "{} Comment {}/{} (ID: {})",
                "📝".bold(),
                i + 1,
                total_comments,
                comment.id
            );
            println!();
            println!("{}: {}", "Author".cyan(), comment.author);
            println!("{}: {}", "File".cyan(), comment.path);
            if let Some(line) = comment.line {
                println!("{}: {}", "Line".cyan(), line);
            }
            println!();
            println!("{}", "Comment:".yellow().bold());
            println!("{}", comment.body);
            println!();

            // Ask user for action
            let actions = vec![
                "FIX - Make code changes to address the comment",
                "CLARIFY - Ask for clarification",
                "ACKNOWLEDGE - Accept the feedback",
                "DEFER - Address later",
                "DISAGREE - Respectfully disagree",
                "SKIP - Skip for now",
            ];

            let selection = Select::new()
                .with_prompt("What action would you like to take?")
                .items(&actions)
                .default(0)
                .interact()
                .map_err(|e| Error::Other(format!("Selection failed: {}", e)))?;

            let action = match selection {
                0 => ShepherdAction::Fix,
                1 => ShepherdAction::Clarify,
                2 => ShepherdAction::Acknowledge,
                3 => ShepherdAction::Defer,
                4 => ShepherdAction::Disagree,
                5 => continue, // Skip
                _ => continue,
            };

            // Get confidence level
            let confidence_items = vec!["HIGH", "MEDIUM", "LOW"];
            let confidence_selection = Select::new()
                .with_prompt("Confidence level")
                .items(&confidence_items)
                .default(1)
                .interact()
                .map_err(|e| Error::Other(format!("Selection failed: {}", e)))?;

            let confidence = match confidence_selection {
                0 => ConfidenceLevel::High,
                1 => ConfidenceLevel::Medium,
                2 => ConfidenceLevel::Low,
                _ => ConfidenceLevel::Medium,
            };

            // Get response/notes
            let response: String = Input::new()
                .with_prompt("Response/notes (or press Enter to skip)")
                .allow_empty(true)
                .interact_text()
                .map_err(|e| Error::Other(format!("Input failed: {}", e)))?;

            // Update shepherd analysis
            self.record_shepherd_action(
                &mut session,
                comment.id,
                action.clone(),
                confidence,
                response.clone(),
            )?;

            println!();
            println!(
                "{} Action recorded: {:?} ({})",
                "✓".green(),
                action,
                confidence.as_str()
            );

            if !response.is_empty() {
                println!("   Notes: {}", response);
            }

            resolved_count += 1;
            println!();

            // Ask if user wants to continue
            if i < total_comments - 1 {
                let continue_prompt = Confirm::new()
                    .with_prompt("Continue to next comment?")
                    .default(true)
                    .interact()
                    .map_err(|e| Error::Other(format!("Confirmation failed: {}", e)))?;

                if !continue_prompt {
                    break;
                }
                println!();
            }
        }

        // Save session
        session.log_activity(
            ActivityType::ShepherdRun,
            format!("Processed {} comments", resolved_count),
        );
        self.session_mgr.save_session(&session)?;

        println!("─────────────────────────────────────────────");
        println!();
        println!("{} Shepherd workflow complete!", "✓".green());
        println!(
            "   Processed: {}/{}",
            resolved_count.to_string().bold(),
            total_comments
        );
        println!();
        println!("Next steps: Review your changes and update the PR with 'git push'");
        println!();

        Ok(())
    }

    /// Run shepherd workflow in batch mode
    pub fn run_batch(&self, session_name: &str, auto_fix: bool) -> Result<()> {
        let mut session = self.session_mgr.load_session(session_name)?;

        if session.unresolved_comments.is_empty() {
            println!("{} No unresolved comments!", "✓".green());
            return Ok(());
        }

        println!(
            "{} Processing {} comments in batch mode...",
            "→".cyan(),
            session.unresolved_comments.len()
        );

        let mut fixed_count = 0;
        let mut deferred_count = 0;

        let comments_clone = session.unresolved_comments.clone();
        for comment in &comments_clone {
            // Simple heuristic: if comment contains certain keywords, mark as FIX
            let should_fix = auto_fix
                && (comment.body.to_lowercase().contains("typo")
                    || comment.body.to_lowercase().contains("naming")
                    || comment.body.to_lowercase().contains("formatting"));

            if should_fix {
                self.record_shepherd_action(
                    &mut session,
                    comment.id,
                    ShepherdAction::Fix,
                    ConfidenceLevel::Medium,
                    "Auto-marked for fixing".to_string(),
                )?;
                fixed_count += 1;
            } else {
                self.record_shepherd_action(
                    &mut session,
                    comment.id,
                    ShepherdAction::Defer,
                    ConfidenceLevel::Low,
                    "Needs manual review".to_string(),
                )?;
                deferred_count += 1;
            }
        }

        session.log_activity(
            ActivityType::ShepherdRun,
            format!(
                "Batch processed: {} fixed, {} deferred",
                fixed_count, deferred_count
            ),
        );
        self.session_mgr.save_session(&session)?;

        println!(
            "{} Batch processing complete: {} marked for fix, {} deferred",
            "✓".green(),
            fixed_count,
            deferred_count
        );

        Ok(())
    }

    /// Show shepherd status
    pub fn show_status(&self, session_name: &str) -> Result<()> {
        let session = self.session_mgr.load_session(session_name)?;

        println!();
        println!("{} Shepherd Status", "🐕".bold());
        println!();
        println!("Session: {}", session.name);
        println!("Total comments: {}", session.unresolved_comments.len());

        // Count actions
        let mut action_counts = std::collections::HashMap::new();
        for comment in &session.unresolved_comments {
            if comment.resolved {
                let action_key = "resolved";
                *action_counts.entry(action_key).or_insert(0) += 1;
            } else {
                *action_counts.entry("pending").or_insert(0) += 1;
            }
        }

        println!();
        println!("Action breakdown:");
        for (action, count) in action_counts {
            println!("  {}: {}", action, count);
        }
        println!();

        Ok(())
    }

    // === Private helper methods ===

    fn record_shepherd_action(
        &self,
        session: &mut Session,
        comment_id: u64,
        action: ShepherdAction,
        confidence: ConfidenceLevel,
        response: String,
    ) -> Result<()> {
        // Update the comment with shepherd analysis
        let mut found = false;
        for comment in session.unresolved_comments.iter_mut() {
            if comment.id == comment_id {
                // Mark as resolved based on action
                comment.resolved = matches!(
                    action,
                    ShepherdAction::Fix | ShepherdAction::Acknowledge | ShepherdAction::Disagree
                );
                found = true;
                break;
            }
        }
        if !found {
            return Ok(());
        }

        // Record in context
        let analysis_file = session.context_dir.join("shepherd_analysis.md");
        let mut content = if analysis_file.exists() {
            fs::read_to_string(&analysis_file)?
        } else {
            format!("# Shepherd Analysis\n\nSession: {}\n\n", session.name)
        };

        content.push_str(&format!("\n## Comment ID: {}\n", comment_id));
        content.push_str(&format!("Action: {:?}\n", action));
        content.push_str(&format!("Confidence: {}\n", confidence.as_str()));
        if !response.is_empty() {
            content.push_str(&format!("Response: {}\n", response));
        }
        content.push_str(&format!("Timestamp: {}\n", chrono::Utc::now()));

        fs::write(&analysis_file, content)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shepherd_creation() {
        let config = Config::default();
        match Shepherd::new(config) {
            Ok(_) => {
                // Successfully created shepherd
            }
            Err(Error::HnNotFound) => {
                println!("Skipping: hn not installed");
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }
}
