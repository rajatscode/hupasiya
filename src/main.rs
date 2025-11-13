use anyhow::Result;
use clap::{Parser, Subcommand};

/// hupasiya - Multi-agent session orchestrator
#[derive(Parser)]
#[command(name = "hp")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new session
    New {
        /// Session name
        name: String,

        /// Agent type (feature, bugfix, test, docs, etc.)
        #[arg(short, long, default_value = "feature")]
        r#type: String,
    },

    /// List all sessions
    List {
        /// Show all sessions (including archived)
        #[arg(short, long)]
        all: bool,
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
    },

    /// Show version information
    Version,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, r#type } => {
            println!("Creating session '{}' with type '{}'", name, r#type);
            println!("(Not yet implemented - this is a placeholder)");
            Ok(())
        }
        Commands::List { all } => {
            println!("Listing sessions (all: {})", all);
            println!("(Not yet implemented - this is a placeholder)");
            Ok(())
        }
        Commands::Info { name } => {
            println!("Showing info for session '{}'", name);
            println!("(Not yet implemented - this is a placeholder)");
            Ok(())
        }
        Commands::Close {
            name,
            remove_workbox,
        } => {
            println!(
                "Closing session '{}' (remove_workbox: {})",
                name, remove_workbox
            );
            println!("(Not yet implemented - this is a placeholder)");
            Ok(())
        }
        Commands::Version => {
            println!("hupasiya (hp) v{}", env!("CARGO_PKG_VERSION"));
            println!("Multi-agent session orchestrator");
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        // Placeholder test to ensure the project compiles
        assert_eq!(2 + 2, 4);
    }
}
