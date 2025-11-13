//! CLI commands implementation

use crate::config::Config;
use crate::context::ContextManager;
use crate::error::Result;
use crate::hn_client::{HnClient, WorkboxOptions};
use crate::models::{AgentType, SessionStatus, SnapshotTrigger};
use crate::session::SessionManager;
use colored::Colorize;
use std::io::{self, Write};

/// Execute the 'new' command
pub fn cmd_new(
    name: &str,
    agent_type: &str,
    from_branch: Option<String>,
    no_branch: bool,
    parent: Option<String>,
) -> Result<()> {
    let config = Config::load()?;
    let session_mgr = SessionManager::new(config.clone())?;
    let context_mgr = ContextManager::new(config.clone())?;

    // Parse agent type
    let agent_type = AgentType::from_str(agent_type)
        .map_err(|e| crate::error::Error::InvalidAgentType(e))?;

    // Create workbox options
    let opts = WorkboxOptions {
        from: from_branch,
        no_branch,
        ..Default::default()
    };

    // Create session
    println!("{}", "Creating session...".cyan());
    let mut session = session_mgr.create_session(name, agent_type, opts)?;

    // Link to parent if specified
    if let Some(parent_name) = parent {
        println!("{} {}", "Linking to parent:".cyan(), parent_name);
        session_mgr.link_parent_child(&parent_name, name)?;
        session.parent = Some(parent_name);
    }

    // Initialize context
    println!("{}", "Initializing context...".cyan());
    context_mgr.init_context(&session)?;

    println!();
    println!("{} Session '{}' created successfully!", "✓".green(), name.bold());
    println!("  Workbox: {}", session.workbox_path.display());
    println!("  Branch: {}", session.branch);
    println!("  Context: {}", session.context_dir.display());
    println!();
    println!("Next steps:");
    println!("  hp context edit {}  # Edit session context", name);
    println!("  hp switch {}        # Switch to session", name);

    Ok(())
}

/// Execute the 'list' command
pub fn cmd_list(all: bool, tree: bool, format: Option<String>) -> Result<()> {
    let config = Config::load()?;
    let session_mgr = SessionManager::new(config)?;

    let sessions = if all {
        session_mgr.list_sessions()?
    } else {
        session_mgr.list_sessions_by_status(SessionStatus::Active)?
    };

    if sessions.is_empty() {
        println!("{}", "No sessions found.".yellow());
        println!("Create one with: hp new <session-name>");
        return Ok(());
    }

    if let Some(fmt) = format {
        if fmt == "json" {
            let json = serde_json::to_string_pretty(&sessions)?;
            println!("{}", json);
            return Ok(());
        }
    }

    if tree {
        print_session_tree(&sessions)?;
    } else {
        print_session_table(&sessions)?;
    }

    Ok(())
}

/// Execute the 'info' command
pub fn cmd_info(name: &str, verbose: bool) -> Result<()> {
    let config = Config::load()?;
    let session_mgr = SessionManager::new(config.clone())?;
    let hn_client = HnClient::new()?;

    let session = session_mgr.load_session(name)?;

    println!();
    println!("{}", format!("Session: {}", session.name).bold());
    println!("{}", "=".repeat(40));
    println!("  ID: {}", session.id);
    println!("  Type: {:?}", session.agent_type);
    println!("  Status: {:?}", session.status);
    println!("  Created: {}", session.created.format("%Y-%m-%d %H:%M:%S"));
    println!("  Last Active: {}", session.last_active.format("%Y-%m-%d %H:%M:%S"));
    println!();
    println!("{}", "Workbox:".bold());
    println!("  Name: {}", session.workbox_name);
    println!("  Path: {}", session.workbox_path.display());
    println!("  Branch: {}", session.branch);
    println!("  Base: {}", session.base_branch);
    println!("  VCS: {}", session.vcs_type);
    println!();
    println!("{}", "Context:".bold());
    println!("  Directory: {}", session.context_dir.display());
    println!("  Snapshots: {}", session.context_snapshots.len());

    if let Some(parent) = &session.parent {
        println!();
        println!("{}", "Relationships:".bold());
        println!("  Parent: {}", parent);
    }

    if !session.children.is_empty() {
        if session.parent.is_none() {
            println!();
            println!("{}", "Relationships:".bold());
        }
        println!("  Children: {}", session.children.join(", "));
    }

    if verbose {
        println!();
        println!("{}", "Metrics:".bold());
        println!("  Commits: {}", session.metrics.commits);
        println!("  Lines Added: {}", session.metrics.lines_added);
        println!("  Lines Removed: {}", session.metrics.lines_removed);
        println!("  Files Changed: {}", session.metrics.files_changed);
        println!("  AI Interactions: {}", session.metrics.ai_interactions);

        // Try to get workbox info
        if let Ok(wb_info) = hn_client.get_workbox_info(&session.workbox_name) {
            println!();
            println!("{}", "Workbox Status:".bold());
            println!("  Commit: {}", wb_info.commit);
        }
    }

    println!();

    Ok(())
}

/// Execute the 'close' command
pub fn cmd_close(name: &str, remove_workbox: bool, archive: bool) -> Result<()> {
    let config = Config::load()?;
    let session_mgr = SessionManager::new(config)?;

    let session = session_mgr.load_session(name)?;

    // Confirm if there are children
    if !session.children.is_empty() {
        println!("{}", "Warning: This session has children:".yellow());
        for child in &session.children {
            println!("  - {}", child);
        }
        print!("Continue? [y/N] ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }
    }

    let status = if archive {
        SessionStatus::Archived
    } else {
        SessionStatus::Integrated
    };

    session_mgr.close_session(name, status, remove_workbox)?;

    println!("{} Session '{}' closed.", "✓".green(), name.bold());

    if remove_workbox {
        println!("  Workbox removed");
    }

    Ok(())
}

/// Execute the 'switch' command
pub fn cmd_switch(name: &str, output_shell: bool) -> Result<()> {
    let config = Config::load()?;
    let session_mgr = SessionManager::new(config)?;

    let session = session_mgr.load_session(name)?;

    if output_shell {
        // Output shell commands for wrapper to execute
        println!("cd {}", session.workbox_path.display());
        println!("export HP_SESSION={}", session.name);
        println!("export HP_CONTEXT={}", session.context_dir.join("context.md").display());
        println!("export HP_WORKBOX={}", session.workbox_path.display());
        println!("export HP_VCS={}", session.vcs_type);
    } else {
        println!("{}", "Note: Use shell wrapper for 'hp switch' to work properly.".yellow());
        println!();
        println!("Add to ~/.bashrc or ~/.zshrc:");
        println!();
        println!(r#"hp() {{
    if [[ "$1" == "switch" ]]; then
        local session_info=$(command hp switch "$2" --output-shell)
        eval "$session_info"
    else
        command hp "$@"
    fi
}}"#);
        println!();
        println!("Workbox path: {}", session.workbox_path.display());
    }

    Ok(())
}

/// Execute the 'context view' command
pub fn cmd_context_view(session_name: Option<String>) -> Result<()> {
    let config = Config::load()?;
    let session_mgr = SessionManager::new(config.clone())?;
    let context_mgr = ContextManager::new(config)?;

    let name = get_session_name(session_name)?;
    let session = session_mgr.load_session(&name)?;

    let content = context_mgr.read_context(&session)?;
    println!("{}", content);

    Ok(())
}

/// Execute the 'context edit' command
pub fn cmd_context_edit(session_name: Option<String>) -> Result<()> {
    let config = Config::load()?;
    let session_mgr = SessionManager::new(config.clone())?;
    let context_mgr = ContextManager::new(config)?;

    let name = get_session_name(session_name)?;
    let session = session_mgr.load_session(&name)?;

    context_mgr.edit_context(&session)?;

    println!("{} Context updated", "✓".green());

    Ok(())
}

/// Execute the 'context snapshot' command
pub fn cmd_context_snapshot(
    session_name: Option<String>,
    snapshot_name: String,
    description: Option<String>,
) -> Result<()> {
    let config = Config::load()?;
    let session_mgr = SessionManager::new(config.clone())?;
    let context_mgr = ContextManager::new(config)?;

    let name = get_session_name(session_name)?;
    let session = session_mgr.load_session(&name)?;

    let snapshot = context_mgr.create_snapshot(
        &session,
        &snapshot_name,
        SnapshotTrigger::Manual,
        description,
    )?;

    println!("{} Snapshot '{}' created", "✓".green(), snapshot.name.bold());
    println!("  Path: {}", snapshot.path.display());

    Ok(())
}

/// Execute the 'doctor' command
pub fn cmd_doctor() -> Result<()> {
    println!("{}", "Running system checks...".bold());
    println!();

    let mut all_ok = true;

    // Check hannahanna
    print!("Checking hannahanna (hn)... ");
    io::stdout().flush()?;
    match HnClient::check_installed() {
        Ok(_) => {
            println!("{}", "OK".green());

            // Try to get version
            if let Ok(output) = std::process::Command::new("hn").arg("--version").output() {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout);
                    println!("  Version: {}", version.trim());
                }
            }
        }
        Err(e) => {
            println!("{}", "FAIL".red());
            println!("  Error: {}", e);
            all_ok = false;
        }
    }

    // Check git
    print!("Checking git... ");
    io::stdout().flush()?;
    match std::process::Command::new("git").arg("--version").output() {
        Ok(output) if output.status.success() => {
            println!("{}", "OK".green());
            let version = String::from_utf8_lossy(&output.stdout);
            println!("  {}", version.trim());
        }
        _ => {
            println!("{}", "FAIL".red());
            all_ok = false;
        }
    }

    // Check configuration
    print!("Checking configuration... ");
    io::stdout().flush()?;
    match Config::load() {
        Ok(config) => {
            println!("{}", "OK".green());
            println!("  Default agent: {:?}", config.hp.default_agent);
            println!("  Sessions dir: {}", config.hp.sessions.metadata_dir.display());
            println!("  Contexts dir: {}", config.hp.sessions.context_dir.display());
        }
        Err(e) => {
            println!("{}", "FAIL".red());
            println!("  Error: {}", e);
            all_ok = false;
        }
    }

    // Check directories
    print!("Checking directories... ");
    io::stdout().flush()?;
    let dirs_ok = std::path::Path::new(".hp").exists()
        || std::fs::create_dir_all(".hp/sessions").is_ok();

    if dirs_ok {
        println!("{}", "OK".green());
    } else {
        println!("{}", "FAIL".red());
        all_ok = false;
    }

    println!();

    if all_ok {
        println!("{} All checks passed!", "✓".green());
    } else {
        println!("{} Some checks failed.", "✗".red());
        println!();
        println!("Install hannahanna: cargo install hannahanna");
    }

    Ok(())
}

/// Execute the 'version' command
pub fn cmd_version() -> Result<()> {
    println!("hupasiya (hp) v{}", env!("CARGO_PKG_VERSION"));
    println!("Multi-agent session orchestrator");
    println!();
    println!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));
    println!("License: {}", env!("CARGO_PKG_LICENSE"));

    Ok(())
}

// Helper functions

fn print_session_table(sessions: &[crate::models::Session]) -> Result<()> {
    println!();
    println!(
        "{:<20} {:<12} {:<10} {:<20} {:<15}",
        "NAME", "TYPE", "STATUS", "BRANCH", "LAST ACTIVE"
    );
    println!("{}", "=".repeat(85));

    for session in sessions {
        let type_str = format!("{:?}", session.agent_type);
        let status_str = format!("{:?}", session.status);
        let last_active = session.last_active.format("%Y-%m-%d %H:%M").to_string();

        println!(
            "{:<20} {:<12} {:<10} {:<20} {:<15}",
            session.name, type_str, status_str, session.branch, last_active
        );
    }

    println!();
    println!("Total: {} sessions", sessions.len());
    println!();

    Ok(())
}

fn print_session_tree(sessions: &[crate::models::Session]) -> Result<()> {
    // Find root sessions (no parent)
    let roots: Vec<_> = sessions.iter().filter(|s| s.parent.is_none()).collect();

    println!();

    for root in roots {
        print_session_node(root, sessions, 0);
    }

    println!();

    Ok(())
}

fn print_session_node(session: &crate::models::Session, all_sessions: &[crate::models::Session], depth: usize) {
    let indent = "  ".repeat(depth);
    let prefix = if depth == 0 { "" } else { "└─ " };

    let status_icon = match session.status {
        SessionStatus::Active => "●".green(),
        SessionStatus::Paused => "◐".yellow(),
        _ => "○".normal(),
    };

    println!(
        "{}{}{} {} ({:?})",
        indent, prefix, status_icon, session.name.bold(), session.agent_type
    );

    // Print children
    for child_name in &session.children {
        if let Some(child) = all_sessions.iter().find(|s| s.name == *child_name) {
            print_session_node(child, all_sessions, depth + 1);
        }
    }
}

fn get_session_name(session_name: Option<String>) -> Result<String> {
    if let Some(name) = session_name {
        return Ok(name);
    }

    // Try to get from environment
    if let Ok(name) = std::env::var("HP_SESSION") {
        return Ok(name);
    }

    Err(crate::error::Error::Other(
        "No session specified. Use --session=<name> or set HP_SESSION".to_string(),
    ))
}
