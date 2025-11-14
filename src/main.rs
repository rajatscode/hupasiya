use anyhow::Result;
use clap::{Args, Parser, Subcommand};

mod activity;
mod ai_tool;
mod cli;
mod collaboration;
mod completions;
mod config;
mod context;
mod error;
mod hn_client;
mod models;
mod orchestration;
mod pr;
mod profiles;
mod progress;
mod session;
mod shepherd;
mod templates;
mod tutorial;
mod utilities;

/// hupasiya - Multi-agent session orchestrator
#[derive(Parser)]
#[command(name = "hp")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new session
    New {
        /// Session name
        name: String,

        /// Agent type (feature, bugfix, test, docs, etc.)
        #[arg(short = 't', long, default_value = "feature")]
        r#type: String,

        /// Base branch to create from
        #[arg(long)]
        from: Option<String>,

        /// Create on current branch (no new branch)
        #[arg(long)]
        no_branch: bool,

        /// Parent session (creates dependency)
        #[arg(long)]
        parent: Option<String>,
    },

    /// List all sessions
    List {
        /// Show all sessions (including archived)
        #[arg(short, long)]
        all: bool,

        /// Show as tree with parent/child relationships
        #[arg(long)]
        tree: bool,

        /// Output format (table, json)
        #[arg(long)]
        format: Option<String>,
    },

    /// Show session info
    Info {
        /// Session name
        name: String,
    },

    /// Close a session
    Close {
        /// Session name
        name: String,

        /// Remove workbox
        #[arg(long)]
        remove_workbox: bool,

        /// Archive instead of marking as integrated
        #[arg(long)]
        archive: bool,
    },

    /// Switch to a session
    Switch {
        /// Session name
        name: String,

        /// Output shell commands (for wrapper)
        #[arg(long, hide = true)]
        output_shell: bool,
    },

    /// Context management
    Context(ContextCommand),

    /// Cascade parent changes to children
    Cascade {
        /// Parent session name
        parent: String,

        /// Dry run (show what would happen)
        #[arg(long)]
        dry_run: bool,
    },

    /// Gather children back to parent
    Gather {
        /// Parent session name
        parent: String,

        /// Dry run (show what would happen)
        #[arg(long)]
        dry_run: bool,
    },

    /// Show session tree
    Tree {
        /// Root session (or show all roots)
        session: Option<String>,
    },

    /// Launch AI tool with context
    Launch {
        /// Session name (or use HP_SESSION env var)
        session: Option<String>,

        /// Override AI tool command
        #[arg(long)]
        tool: Option<String>,

        /// Configuration profile to use
        #[arg(long)]
        profile: Option<String>,

        /// Extra args to pass to AI tool
        #[arg(last = true)]
        extra_args: Vec<String>,
    },

    /// Launch shell in session workbox
    Shell {
        /// Session name (or use HP_SESSION env var)
        session: Option<String>,

        /// Command to run (instead of interactive shell)
        #[arg(last = true)]
        command: Option<Vec<String>>,
    },

    /// Execute command in session workbox
    Exec {
        /// Session name
        session: String,

        /// Command to execute
        #[arg(required = true)]
        command: Vec<String>,

        /// Execute in all children
        #[arg(long)]
        cascade: bool,

        /// Execute in parent and all children
        #[arg(long)]
        tree: bool,
    },

    /// PR operations
    Pr(PrCommand),

    /// Shepherd - Interactive PR comment resolution
    Shepherd {
        /// Session name
        session: Option<String>,

        /// Batch mode (non-interactive)
        #[arg(long)]
        batch: bool,

        /// Auto-fix in batch mode
        #[arg(long)]
        auto_fix: bool,

        /// Show status only
        #[arg(long)]
        status: bool,
    },

    /// Activity and metrics
    Activity(ActivityCommand),

    /// Template marketplace
    Template(TemplateCommand),

    /// Session collaboration
    Collab(CollabCommand),

    /// Configuration profiles
    Profile(ProfileCommand),

    /// Utilities
    Util(UtilCommand),

    /// Check installation and configuration
    Doctor,

    /// Interactive tutorial - learn all features
    Tutorial {
        /// Skip intro and jump to section selection
        #[arg(long)]
        skip_intro: bool,
    },

    /// Generate shell completion scripts
    Completions {
        /// Shell type (bash, zsh, fish)
        shell: completions::CompletionShell,
    },

    /// Show version information
    Version,
}

#[derive(Args)]
struct ContextCommand {
    #[command(subcommand)]
    command: ContextSubcommand,
}

#[derive(Subcommand)]
enum ContextSubcommand {
    /// View context
    View {
        /// Session name (or use HP_SESSION env var)
        session: Option<String>,
    },

    /// Edit context
    Edit {
        /// Session name (or use HP_SESSION env var)
        session: Option<String>,
    },

    /// Create a snapshot
    Snapshot {
        /// Snapshot name
        name: String,

        /// Session name (or use HP_SESSION env var)
        #[arg(long)]
        session: Option<String>,

        /// Description
        #[arg(long)]
        description: Option<String>,
    },
}

#[derive(Args)]
struct PrCommand {
    #[command(subcommand)]
    command: PrSubcommand,
}

#[derive(Subcommand)]
enum PrSubcommand {
    /// Create PR from session
    Create {
        /// Session name
        session: String,

        /// Create as draft PR
        #[arg(long)]
        draft: bool,

        /// Reviewers (comma-separated)
        #[arg(long)]
        reviewers: Option<String>,

        /// Labels (comma-separated)
        #[arg(long)]
        labels: Option<String>,

        /// Use context for PR body
        #[arg(long)]
        from_context: bool,
    },

    /// Sync PR comments to context
    Sync {
        /// Session name
        session: String,

        /// Create shepherd tasks
        #[arg(long)]
        shepherd: bool,
    },

    /// Show PR status
    Status {
        /// Session name
        session: String,
    },
}

#[derive(Args)]
struct ActivityCommand {
    #[command(subcommand)]
    command: ActivitySubcommand,
}

#[derive(Subcommand)]
enum ActivitySubcommand {
    /// Show activity log
    Show {
        /// Session name
        session: String,

        /// Limit number of events
        #[arg(long)]
        limit: Option<usize>,
    },

    /// Show session metrics
    Metrics {
        /// Session name
        session: String,
    },

    /// Show global stats
    Stats,
}

#[derive(Args)]
struct TemplateCommand {
    #[command(subcommand)]
    command: TemplateSubcommand,
}

#[derive(Subcommand)]
enum TemplateSubcommand {
    /// List templates
    List,

    /// Search templates
    Search {
        /// Search query
        query: String,
    },

    /// Install template
    Install {
        /// Template source (file path or URL)
        source: String,

        /// Template name
        #[arg(long)]
        name: Option<String>,
    },
}

#[derive(Args)]
struct CollabCommand {
    #[command(subcommand)]
    command: CollabSubcommand,
}

#[derive(Subcommand)]
enum CollabSubcommand {
    /// Hand off session to another developer
    Handoff {
        /// Session name
        session: String,

        /// Target user
        to: String,

        /// Handoff message
        #[arg(long)]
        message: Option<String>,
    },

    /// Clone session
    Clone {
        /// Source session
        source: String,

        /// New session name
        name: String,

        /// Create divergent branch
        #[arg(long)]
        diverge: bool,
    },

    /// Merge sessions
    Merge {
        /// Target session
        target: String,

        /// Source sessions (comma-separated)
        sources: String,

        /// Merge strategy
        #[arg(long, default_value = "default")]
        strategy: String,
    },
}

#[derive(Args)]
struct ProfileCommand {
    #[command(subcommand)]
    command: ProfileSubcommand,
}

#[derive(Subcommand)]
enum ProfileSubcommand {
    /// List profiles
    List,

    /// Show profile details
    Show {
        /// Profile name
        name: String,
    },
}

#[derive(Args)]
struct UtilCommand {
    #[command(subcommand)]
    command: UtilSubcommand,
}

#[derive(Subcommand)]
enum UtilSubcommand {
    /// Monitor sessions
    Monitor {
        /// Watch mode (auto-refresh)
        #[arg(long)]
        watch: bool,
    },

    /// Clean old sessions
    Clean {
        /// Days threshold
        #[arg(long, default_value = "30")]
        older_than: u64,

        /// Dry run
        #[arg(long)]
        dry_run: bool,

        /// Force without confirmation
        #[arg(long)]
        force: bool,
    },

    /// Leave session gracefully
    Leave {
        /// Session name
        session: String,

        /// Archive instead of pause
        #[arg(long)]
        archive: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::New {
            name,
            r#type,
            from,
            no_branch,
            parent,
        } => cli::cmd_new(&name, &r#type, from, no_branch, parent),

        Commands::List { all, tree, format } => cli::cmd_list(all, tree, format),

        Commands::Info { name } => cli::cmd_info(&name, cli.verbose),

        Commands::Close {
            name,
            remove_workbox,
            archive,
        } => cli::cmd_close(&name, remove_workbox, archive),

        Commands::Switch { name, output_shell } => cli::cmd_switch(&name, output_shell),

        Commands::Context(ctx) => match ctx.command {
            ContextSubcommand::View { session } => cli::cmd_context_view(session),
            ContextSubcommand::Edit { session } => cli::cmd_context_edit(session),
            ContextSubcommand::Snapshot {
                name,
                session,
                description,
            } => cli::cmd_context_snapshot(session, name, description),
        },

        Commands::Cascade { parent, dry_run } => cli::cmd_cascade(&parent, dry_run),

        Commands::Gather { parent, dry_run } => cli::cmd_gather(&parent, dry_run),

        Commands::Tree { session } => cli::cmd_tree(session),

        Commands::Launch {
            session,
            tool,
            profile,
            extra_args,
        } => cli::cmd_launch(session, tool, profile, extra_args),

        Commands::Shell { session, command } => cli::cmd_shell(session, command),

        Commands::Exec {
            session,
            command,
            cascade,
            tree,
        } => cli::cmd_exec(session, command, cascade, tree),

        Commands::Pr(pr) => match pr.command {
            PrSubcommand::Create {
                session,
                draft,
                reviewers,
                labels,
                from_context,
            } => {
                let reviewers_vec =
                    reviewers.map(|r| r.split(',').map(|s| s.trim().to_string()).collect());
                let labels_vec =
                    labels.map(|l| l.split(',').map(|s| s.trim().to_string()).collect());
                cli::cmd_pr_create(&session, draft, reviewers_vec, labels_vec, from_context)
            }
            PrSubcommand::Sync { session, shepherd } => cli::cmd_pr_sync(&session, shepherd),
            PrSubcommand::Status { session } => cli::cmd_pr_status(&session),
        },

        Commands::Shepherd {
            session,
            batch,
            auto_fix,
            status,
        } => cli::cmd_shepherd(session, batch, auto_fix, status),

        Commands::Activity(activity) => match activity.command {
            ActivitySubcommand::Show { session, limit } => cli::cmd_activity(&session, limit),
            ActivitySubcommand::Metrics { session } => cli::cmd_metrics(&session),
            ActivitySubcommand::Stats => cli::cmd_stats(),
        },

        Commands::Template(template) => match template.command {
            TemplateSubcommand::List => cli::cmd_template_list(),
            TemplateSubcommand::Search { query } => cli::cmd_template_search(&query),
            TemplateSubcommand::Install { source, name } => {
                cli::cmd_template_install(&source, name)
            }
        },

        Commands::Collab(collab) => match collab.command {
            CollabSubcommand::Handoff {
                session,
                to,
                message,
            } => cli::cmd_handoff(&session, &to, message),
            CollabSubcommand::Clone {
                source,
                name,
                diverge,
            } => cli::cmd_clone(&source, &name, diverge),
            CollabSubcommand::Merge {
                target,
                sources,
                strategy,
            } => {
                let sources_vec: Vec<String> =
                    sources.split(',').map(|s| s.trim().to_string()).collect();
                cli::cmd_merge(&target, sources_vec, &strategy)
            }
        },

        Commands::Profile(profile) => match profile.command {
            ProfileSubcommand::List => cli::cmd_profile_list(),
            ProfileSubcommand::Show { name } => cli::cmd_profile_show(&name),
        },

        Commands::Util(util) => match util.command {
            UtilSubcommand::Monitor { watch } => cli::cmd_monitor(watch),
            UtilSubcommand::Clean {
                older_than,
                dry_run,
                force,
            } => cli::cmd_clean(older_than, dry_run, force),
            UtilSubcommand::Leave { session, archive } => cli::cmd_leave(&session, archive),
        },

        Commands::Doctor => cli::cmd_doctor(),

        Commands::Tutorial { skip_intro } => cli::cmd_tutorial(skip_intro),

        Commands::Completions { shell } => completions::generate_completions(shell),

        Commands::Version => cli::cmd_version(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
