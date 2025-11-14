//! Interactive tutorial for hupasiya
//!
//! Comprehensive walkthrough of all hupasiya features.

use crate::config::Config;
use crate::error::Result;
use colored::Colorize;
use dialoguer::{Confirm, Select};
use std::io::{self, Write};

/// Tutorial manager
pub struct Tutorial {
    #[allow(dead_code)]
    config: Config,
}

impl Tutorial {
    /// Create new tutorial manager
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self { config })
    }

    /// Run interactive tutorial
    pub fn run(&self, skip_intro: bool) -> Result<()> {
        if !skip_intro {
            self.show_intro()?;
        }

        // Tutorial sections
        let sections = vec![
            "1. Session Management (new, list, info)",
            "2. Context Management (edit, snapshots, templates)",
            "3. Multi-Agent Coordination (cascade, gather, tree)",
            "4. AI Tool Integration (launch)",
            "5. PR Workflows (create, sync, status)",
            "6. Shepherd - PR Comment Resolution",
            "7. Activity & Metrics",
            "8. Session Collaboration (handoff, clone, merge)",
            "9. Template Marketplace",
            "10. Configuration Profiles",
            "11. Monitoring & Utilities",
            "Complete Tutorial - All Sections",
        ];

        let selection = Select::new()
            .with_prompt("Choose a section to learn about (or run complete tutorial)")
            .items(&sections)
            .default(11)
            .interact()
            .map_err(|e| crate::error::Error::Other(format!("Selection failed: {}", e)))?;

        match selection {
            0 => self.tutorial_session_management()?,
            1 => self.tutorial_context_management()?,
            2 => self.tutorial_multi_agent()?,
            3 => self.tutorial_ai_tool()?,
            4 => self.tutorial_pr_workflows()?,
            5 => self.tutorial_shepherd()?,
            6 => self.tutorial_activity_metrics()?,
            7 => self.tutorial_collaboration()?,
            8 => self.tutorial_templates()?,
            9 => self.tutorial_profiles()?,
            10 => self.tutorial_monitoring()?,
            11 => self.run_complete_tutorial()?,
            _ => {}
        }

        self.show_outro()?;

        Ok(())
    }

    // === Tutorial sections ===

    fn tutorial_session_management(&self) -> Result<()> {
        self.section_header("Session Management");

        self.explain(
            "Sessions are isolated AI agent contexts. Each session has its own:\n\
             - Workbox (git workspace via hannahanna)\n\
             - Context files (objectives, notes, PR comments)\n\
             - Activity log and metrics\n\
             - Optional parent/child relationships",
        );

        self.pause()?;

        self.step(
            "Creating a session",
            "hp new my-feature --type=feature",
            "Creates a new session named 'my-feature' of type 'feature'.\n\
             This will:\n\
             - Create a new git branch\n\
             - Set up a hannahanna workbox\n\
             - Initialize context directory with template\n\
             - Mark session as active",
        );

        self.prompt_try_it("hp new tutorial-session --type=feature")?;

        self.step(
            "Listing sessions",
            "hp list",
            "Shows all sessions with their status, type, and last activity.\n\
             Use --status=active to filter by status.",
        );

        self.prompt_try_it("hp list")?;

        self.step(
            "Getting session info",
            "hp info tutorial-session",
            "Displays detailed information about a session:\n\
             - Workbox path and branch\n\
             - Context directory\n\
             - Parent/child relationships\n\
             - PR integration status\n\
             - Activity metrics",
        );

        self.prompt_try_it("hp info tutorial-session")?;

        self.step(
            "Switching sessions",
            "hp switch tutorial-session",
            "Switches your shell to work in the session's workbox.\n\
             Note: Requires shell integration (see docs).",
        );

        self.step(
            "Closing a session",
            "hp close tutorial-session",
            "Marks session as completed and cleans up resources.\n\
             Use --archive to archive instead of delete.",
        );

        Ok(())
    }

    fn tutorial_context_management(&self) -> Result<()> {
        self.section_header("Context Management");

        self.explain(
            "Context files provide rich information to AI agents:\n\
             - objectives.md: What you want to accomplish\n\
             - notes.md: Implementation notes and decisions\n\
             - shepherd.md: PR comments to address\n\
             - snapshots/: Versioned context backups",
        );

        self.pause()?;

        self.step(
            "Editing context",
            "hp context edit tutorial-session",
            "Opens the session's objectives.md in your $EDITOR.\n\
             Write what you want the AI agent to do.",
        );

        self.step(
            "Creating a snapshot",
            "hp context snapshot tutorial-session before-refactor",
            "Saves a versioned backup of the current context.\n\
             Useful before major changes.",
        );

        self.step(
            "Listing snapshots",
            "hp context list tutorial-session",
            "Shows all saved context snapshots with timestamps.",
        );

        self.step(
            "Restoring a snapshot",
            "hp context restore tutorial-session before-refactor",
            "Restores context from a saved snapshot.\n\
             Current context is backed up first.",
        );

        self.step(
            "Using templates",
            "hp new review-session --type=review",
            "Session types use different context templates:\n\
             - feature: Feature development template\n\
             - bugfix: Bug investigation template\n\
             - test: Test writing template\n\
             - review: Code review template\n\
             - docs: Documentation template",
        );

        Ok(())
    }

    fn tutorial_multi_agent(&self) -> Result<()> {
        self.section_header("Multi-Agent Coordination");

        self.explain(
            "Parent/child sessions enable parallel work:\n\
             - Parent session: Main feature work\n\
             - Child sessions: Tests, docs, refactoring\n\
             - Cascade: Sync parent changes to children\n\
             - Gather: Collect child work back to parent",
        );

        self.pause()?;

        self.step(
            "Creating a child session",
            "hp new tutorial-tests --parent=tutorial-session --type=test",
            "Creates a child session linked to a parent.\n\
             Child starts from parent's current state.",
        );

        self.prompt_try_it("hp new tutorial-tests --parent=tutorial-session --type=test")?;

        self.step(
            "Viewing session tree",
            "hp tree tutorial-session",
            "Displays the session hierarchy:\n\
             tutorial-session\n\
             └── tutorial-tests",
        );

        self.prompt_try_it("hp tree tutorial-session")?;

        self.step(
            "Cascading changes",
            "hp cascade tutorial-session",
            "Syncs parent commits to all children.\n\
             Uses 'git merge parent-branch' by default.\n\
             Options: --strategy=rebase, --force",
        );

        self.step(
            "Gathering changes",
            "hp gather tutorial-session",
            "Merges all child branches back to parent.\n\
             Options: --squash (combine commits)",
        );

        Ok(())
    }

    fn tutorial_ai_tool(&self) -> Result<()> {
        self.section_header("AI Tool Integration");

        self.explain(
            "hupasiya integrates with various AI coding tools:\n\
             - Claude Code\n\
             - Cursor\n\
             - OpenAI Codex\n\
             - Any CLI-based AI tool\n\n\
             The 'launch' command starts the AI with full context.",
        );

        self.pause()?;

        self.step(
            "Launching AI agent",
            "hp launch",
            "Launches the configured AI tool in the current session.\n\
             Automatically provides:\n\
             - Session context from objectives.md\n\
             - PR comments if in shepherd mode\n\
             - Custom slash commands",
        );

        self.step(
            "Configuring AI tool",
            "Edit .hapusiyas.yml:\n\n\
             hp:\n  \
               ai_tool:\n    \
                 command: claude-code\n    \
                 launch_method: exec  # or tmux, screen, shell-function\n    \
                 context_strategy: slash_command  # or env, file\n    \
                 env:\n      \
                   CLAUDE_API_KEY: your-key",
            "Customize which AI tool to use and how to launch it.",
        );

        self.step(
            "Launch methods",
            "exec: Direct execution (simplest)\n\
             tmux: Launch in tmux session (persistent)\n\
             screen: Launch in screen session (portable)\n\
             shell-function: Call shell function (flexible)",
            "Choose based on your workflow preferences.",
        );

        Ok(())
    }

    fn tutorial_pr_workflows(&self) -> Result<()> {
        self.section_header("PR Workflows");

        self.explain(
            "hupasiya integrates with GitHub PRs:\n\
             - Create PRs from sessions\n\
             - Sync PR comments to context\n\
             - Track PR status and metrics\n\
             - Automated comment resolution (shepherd)",
        );

        self.pause()?;

        self.step(
            "Creating a PR",
            "hp pr create tutorial-session",
            "Creates a GitHub PR for the session's branch.\n\
             Options:\n\
             - --draft: Create as draft PR\n\
             - --reviewers=alice,bob: Request reviews\n\
             - --labels=feature,ai-generated: Add labels\n\
             - --from-context: Use objectives.md for PR body",
        );

        self.step(
            "Syncing PR comments",
            "hp pr sync tutorial-session",
            "Fetches all unresolved PR comments and saves to context.\n\
             Use --shepherd to also generate shepherd.md for AI analysis.",
        );

        self.step(
            "Checking PR status",
            "hp pr status tutorial-session",
            "Displays:\n\
             - PR state (open, draft, merged)\n\
             - Unresolved comment count\n\
             - CI status\n\
             - Review status\n\
             - Mergeable state",
        );

        Ok(())
    }

    fn tutorial_shepherd(&self) -> Result<()> {
        self.section_header("Shepherd - PR Comment Resolution");

        self.explain(
            "Shepherd is an interactive workflow for addressing PR comments:\n\
             1. Fetches unresolved comments\n\
             2. Presents each comment to you\n\
             3. You choose an action (FIX, CLARIFY, ACKNOWLEDGE, etc.)\n\
             4. Records decisions and confidence levels\n\
             5. AI helps implement fixes based on your decisions",
        );

        self.pause()?;

        self.step(
            "Running shepherd",
            "hp shepherd",
            "Starts interactive workflow:\n\
             - Shows each comment one by one\n\
             - Prompts for action:\n\
               * FIX: Make code changes\n\
               * CLARIFY: Ask for more info\n\
               * ACKNOWLEDGE: Accept feedback\n\
               * DEFER: Address later\n\
               * DISAGREE: Respectfully disagree\n\
             - Records confidence (HIGH/MEDIUM/LOW)\n\
             - Saves analysis to shepherd_analysis.md",
        );

        self.step(
            "Batch mode",
            "hp shepherd --batch --auto-fix",
            "Processes comments automatically:\n\
             - Simple issues (typos, naming) marked for FIX\n\
             - Complex issues marked for manual review\n\
             - Useful for large PRs with many comments",
        );

        self.step(
            "Shepherd status",
            "hp shepherd --status",
            "Shows breakdown of comment actions:\n\
             - How many resolved vs pending\n\
             - Action distribution (FIX, CLARIFY, etc.)",
        );

        Ok(())
    }

    fn tutorial_activity_metrics(&self) -> Result<()> {
        self.section_header("Activity & Metrics");

        self.explain(
            "Track what's happening in your sessions:\n\
             - Activity log: Timeline of events\n\
             - Metrics: Quantitative measurements\n\
             - Stats: Aggregated statistics across sessions",
        );

        self.pause()?;

        self.step(
            "Viewing activity log",
            "hp activity tutorial-session",
            "Shows chronological log of session events:\n\
             - Session created/switched/closed\n\
             - AI launches\n\
             - Cascade/gather operations\n\
             - PR created/synced\n\
             - Shepherd runs\n\n\
             Options: --limit=20 to show recent N events",
        );

        self.step(
            "Session metrics",
            "hp metrics tutorial-session",
            "Displays session statistics:\n\
             - AI interactions count\n\
             - Commits made\n\
             - Lines added/removed\n\
             - Context snapshots created\n\
             - Cascade/gather operations",
        );

        self.step(
            "Global stats",
            "hp stats",
            "Aggregated statistics across all sessions:\n\
             - Total sessions by status\n\
             - Total AI interactions\n\
             - Most active sessions\n\
             - Session type distribution",
        );

        Ok(())
    }

    fn tutorial_collaboration(&self) -> Result<()> {
        self.section_header("Session Collaboration");

        self.explain(
            "Transfer work between developers or create parallel workflows:\n\
             - Handoff: Transfer session to another developer\n\
             - Clone: Duplicate session for parallel work\n\
             - Merge: Consolidate multiple sessions",
        );

        self.pause()?;

        self.step(
            "Handing off a session",
            "hp handoff tutorial-session alice",
            "Prepares session for transfer to another developer:\n\
             - Generates handoff notes\n\
             - Pauses session\n\
             - Saves current state\n\
             - Alice can then import the session",
        );

        self.step(
            "Cloning a session",
            "hp clone tutorial-session tutorial-v2 --diverge",
            "Creates a copy of the session:\n\
             - New workbox with same state\n\
             - Copied context files\n\
             - --diverge: Create new branch\n\
             - Without --diverge: Share same branch (risky!)",
        );

        self.step(
            "Merging sessions",
            "hp merge-sessions tutorial-session tutorial-v2,tutorial-tests",
            "Consolidates work from multiple sessions:\n\
             - Merges branches into target\n\
             - Options:\n\
               * --strategy=squash: Combine all commits\n\
               * --strategy=no-ff: Preserve history\n\
             - Combines context files",
        );

        Ok(())
    }

    fn tutorial_templates(&self) -> Result<()> {
        self.section_header("Template Marketplace");

        self.explain(
            "Templates provide starting context for different session types:\n\
             - Built-in templates: feature, bugfix, test, review, etc.\n\
             - Custom templates: Create your own\n\
             - Marketplace: Share and discover templates (stub in v1.0)",
        );

        self.pause()?;

        self.step(
            "Listing templates",
            "hp template list",
            "Shows available templates:\n\
             - Built-in templates with descriptions\n\
             - Custom templates in ~/.config/hp/templates/\n\
             - Tags for easy discovery",
        );

        self.step(
            "Installing a template",
            "hp template install /path/to/template.md custom-feature",
            "Copies a template file to your templates directory.\n\
             Can install from:\n\
             - Local file path\n\
             - URL (future: marketplace)",
        );

        self.step(
            "Searching templates",
            "hp template search python",
            "Finds templates matching query in:\n\
             - Name\n\
             - Description\n\
             - Tags",
        );

        self.step(
            "Using a template",
            "hp new my-session --template=custom-feature",
            "Creates session with specific template instead of default.",
        );

        Ok(())
    }

    fn tutorial_profiles(&self) -> Result<()> {
        self.section_header("Configuration Profiles");

        self.explain(
            "Profiles let you switch between different configurations:\n\
             - Different AI tools (Claude vs Cursor)\n\
             - Different launch methods\n\
             - Different default settings\n\n\
             Define profiles in ~/.config/hupasiya/config.toml",
        );

        self.pause()?;

        self.step(
            "Listing profiles",
            "hp profile list",
            "Shows configured profiles and their settings.",
        );

        self.step(
            "Viewing profile details",
            "hp profile show fast",
            "Displays full configuration for a profile:\n\
             - AI tool command\n\
             - Launch method\n\
             - Environment variables\n\
             - Other settings",
        );

        self.step(
            "Using a profile",
            "hp new my-session --profile=fast",
            "Creates session with profile settings.\n\
             Or: hp launch --profile=fast",
        );

        self.step(
            "Example profile config",
            "# In ~/.config/hupasiya/config.toml\n\n\
             [hp.profiles.fast]\n\
             [hp.profiles.fast.ai_tool]\n\
             command = \"claude-code\"\n\
             launch_method = \"exec\"\n\n\
             [hp.profiles.deep]\n\
             [hp.profiles.deep.ai_tool]\n\
             command = \"cursor\"\n\
             launch_method = \"tmux\"",
            "Create multiple profiles for different workflows.",
        );

        Ok(())
    }

    fn tutorial_monitoring(&self) -> Result<()> {
        self.section_header("Monitoring & Utilities");

        self.explain(
            "Tools for maintaining your sessions:\n\
             - Monitor: Live dashboard of all sessions\n\
             - Clean: Remove old sessions\n\
             - Leave: Gracefully exit sessions\n\
             - Doctor: Health checks",
        );

        self.pause()?;

        self.step(
            "Monitoring dashboard",
            "hp monitor",
            "Displays table of all sessions:\n\
             - Name, status, type\n\
             - Branch and last activity\n\
             - Metrics summary\n\n\
             Use --watch to auto-refresh every 5 seconds.",
        );

        self.step(
            "Cleaning old sessions",
            "hp clean --older-than-days=30",
            "Removes archived/integrated sessions older than N days.\n\
             Options:\n\
             - --dry-run: Preview what would be deleted\n\
             - --force: Skip confirmation",
        );

        self.step(
            "Leaving a session",
            "hp leave tutorial-session",
            "Gracefully exits a session:\n\
             - Saves current state\n\
             - Creates snapshot\n\
             - Returns to previous session or default branch",
        );

        self.step(
            "Health checks",
            "hp doctor",
            "Verifies system setup:\n\
             - hannahanna installed and working\n\
             - Git configured\n\
             - Config files valid\n\
             - Session directories accessible",
        );

        Ok(())
    }

    fn run_complete_tutorial(&self) -> Result<()> {
        println!();
        println!("{}", "═══════════════════════════════════════".cyan());
        println!("{}", "  Complete Interactive Tutorial".bold().cyan());
        println!("{}", "═══════════════════════════════════════".cyan());
        println!();

        println!("This tutorial will walk you through ALL hupasiya features.");
        println!("You'll create a real session and try each command.");
        println!();

        let continue_prompt = Confirm::new()
            .with_prompt("Ready to begin?")
            .default(true)
            .interact()
            .map_err(|e| crate::error::Error::Other(format!("Confirmation failed: {}", e)))?;

        if !continue_prompt {
            return Ok(());
        }

        // Run all sections
        self.tutorial_session_management()?;
        self.tutorial_context_management()?;
        self.tutorial_multi_agent()?;
        self.tutorial_ai_tool()?;
        self.tutorial_pr_workflows()?;
        self.tutorial_shepherd()?;
        self.tutorial_activity_metrics()?;
        self.tutorial_collaboration()?;
        self.tutorial_templates()?;
        self.tutorial_profiles()?;
        self.tutorial_monitoring()?;

        println!();
        println!("{}", "🎉 Tutorial Complete! 🎉".green().bold());
        println!();
        println!("You've learned all the major features of hupasiya!");
        println!();
        println!("Next steps:");
        println!("  1. Clean up tutorial sessions: hp clean --dry-run");
        println!("  2. Read the docs: spec/README.md");
        println!("  3. Join discussions: https://github.com/yourorg/hupasiya/discussions");
        println!();

        Ok(())
    }

    // === Helper methods ===

    fn show_intro(&self) -> Result<()> {
        println!();
        println!(
            "{}",
            "╔════════════════════════════════════════════════╗".cyan()
        );
        println!(
            "{}",
            "║                                                ║".cyan()
        );
        println!(
            "{}",
            "║        Welcome to hupasiya Tutorial! 🎓        ║"
                .cyan()
                .bold()
        );
        println!(
            "{}",
            "║                                                ║".cyan()
        );
        println!(
            "{}",
            "╚════════════════════════════════════════════════╝".cyan()
        );
        println!();
        println!("hupasiya is a multi-agent session orchestrator for AI-assisted development.");
        println!();
        println!("This tutorial will teach you:");
        println!("  • Session management and workflows");
        println!("  • Multi-agent coordination");
        println!("  • PR integration and comment resolution");
        println!("  • Collaboration features");
        println!("  • And much more!");
        println!();
        println!("{}", "Let's get started!".green().bold());
        println!();

        self.pause()?;

        Ok(())
    }

    fn show_outro(&self) -> Result<()> {
        println!();
        println!("{}", "═══════════════════════════════════════".cyan());
        println!();
        println!("{}", "Thank you for completing the tutorial!".bold());
        println!();
        println!("More resources:");
        println!("  • Documentation: spec/README.md");
        println!("  • Commands reference: hp --help");
        println!("  • Examples: spec/workflows.md");
        println!();
        println!("Questions? File an issue or start a discussion on GitHub!");
        println!();

        Ok(())
    }

    fn section_header(&self, title: &str) {
        println!();
        println!("{}", "═══════════════════════════════════════".cyan());
        println!("{}", format!("  {}", title).bold().cyan());
        println!("{}", "═══════════════════════════════════════".cyan());
        println!();
    }

    fn explain(&self, text: &str) {
        println!("{}", text.white());
        println!();
    }

    fn step(&self, title: &str, command: &str, explanation: &str) {
        println!("{} {}", "▸".yellow().bold(), title.bold());
        println!();
        println!("  Command: {}", command.green());
        println!();
        println!("{}", self.indent(explanation, 2));
        println!();
    }

    fn prompt_try_it(&self, command: &str) -> Result<()> {
        println!("{}", "Try it yourself!".yellow().bold());
        println!();
        println!("  Run: {}", command.green().bold());
        println!();

        let completed = Confirm::new()
            .with_prompt("Have you tried this command? (y/n)")
            .default(false)
            .interact()
            .map_err(|e| crate::error::Error::Other(format!("Confirmation failed: {}", e)))?;

        if completed {
            println!("{} Great! Let's continue.", "✓".green());
        } else {
            println!("{}", "No problem! You can try it later.".yellow());
        }
        println!();

        Ok(())
    }

    fn pause(&self) -> Result<()> {
        print!("{}", "Press Enter to continue...".dimmed());
        io::stdout()
            .flush()
            .map_err(|e| crate::error::Error::Other(format!("Failed to flush stdout: {}", e)))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| crate::error::Error::Other(format!("Failed to read input: {}", e)))?;

        Ok(())
    }

    fn indent(&self, text: &str, spaces: usize) -> String {
        let indent = " ".repeat(spaces);
        text.lines()
            .map(|line| format!("{}{}", indent, line))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tutorial_creation() {
        let config = Config::default();
        let result = Tutorial::new(config);
        assert!(result.is_ok());
    }
}
