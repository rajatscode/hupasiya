use anyhow::Result;
use clap::{Parser, Subcommand, Args};

mod cli;
mod config;
mod context;
mod error;
mod hn_client;
mod models;
mod orchestration;
mod session;

/// hupasiya - Multi-agent session orchestrator
#[derive(Parser)]
#[command(name = "hp")]
#[command(author, version, about, long_about = None)]
struct Cli {
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

    /// Check installation and configuration
    Doctor,

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

        Commands::Doctor => cli::cmd_doctor(),

        Commands::Version => cli::cmd_version(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
