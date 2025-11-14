use clap::{CommandFactory, ValueEnum};
use clap_complete::{generate, Shell};
use std::io;

use crate::error::Result;
use crate::Cli;

/// Generate shell completion scripts
pub fn generate_completions(shell: CompletionShell) -> Result<()> {
    let mut cmd = Cli::command();
    let bin_name = "hp";

    match shell {
        CompletionShell::Bash => {
            generate(Shell::Bash, &mut cmd, bin_name, &mut io::stdout());
        }
        CompletionShell::Zsh => {
            generate(Shell::Zsh, &mut cmd, bin_name, &mut io::stdout());
        }
        CompletionShell::Fish => {
            generate(Shell::Fish, &mut cmd, bin_name, &mut io::stdout());
        }
    }

    Ok(())
}

/// Shell types for completion generation
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CompletionShell {
    /// Bash shell
    Bash,
    /// Zsh shell
    Zsh,
    /// Fish shell
    Fish,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_shell_variants() {
        // Test that all shell variants are valid
        let shells = vec![
            CompletionShell::Bash,
            CompletionShell::Zsh,
            CompletionShell::Fish,
        ];

        for shell in shells {
            // This would panic if generate fails
            let result = generate_completions(shell);
            assert!(result.is_ok());
        }
    }
}
