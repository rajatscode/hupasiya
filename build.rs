use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Only set up hooks in development (not when building as a dependency)
    let is_dependency = env::var("CARGO_FEATURE_VENDORED").is_ok() || env::var("DOCS_RS").is_ok();

    if is_dependency {
        return;
    }

    // Check for hannahanna (hn) binary
    let hn_available = Command::new("hn")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    if !hn_available {
        println!("cargo:warning=");
        println!("cargo:warning=⚠️  hannahanna (hn) not found!");
        println!("cargo:warning=");
        println!("cargo:warning=Attempting to install hannahanna automatically...");
        println!("cargo:warning=");

        // Attempt to install hannahanna
        let install_result = Command::new("cargo")
            .args(["install", "hannahanna", "--quiet"])
            .status();

        match install_result {
            Ok(status) if status.success() => {
                println!("cargo:warning=✅ Successfully installed hannahanna!");
                println!("cargo:warning=");
            }
            Ok(_) => {
                println!("cargo:warning=❌ Failed to install hannahanna automatically.");
                println!("cargo:warning=");
                println!("cargo:warning=Please install it manually with:");
                println!("cargo:warning=  cargo install hannahanna");
                println!("cargo:warning=");
            }
            Err(e) => {
                println!("cargo:warning=❌ Error installing hannahanna: {}", e);
                println!("cargo:warning=");
                println!("cargo:warning=Please install it manually with:");
                println!("cargo:warning=  cargo install hannahanna");
                println!("cargo:warning=");
            }
        }
    }

    // Check if we're in a git repository
    let git_dir = Path::new(".git");
    if !git_dir.exists() {
        println!("cargo:warning=Not in a git repository, skipping hook setup");
        return;
    }

    // Set up git hooks path to use .githooks directory
    println!("cargo:rerun-if-changed=.githooks/");

    let output = Command::new("git")
        .args(["config", "core.hooksPath", ".githooks"])
        .output();

    match output {
        Ok(result) if result.status.success() => {
            println!(
                "cargo:warning=✓ Git hooks configured successfully! Using .githooks directory."
            );
        }
        Ok(result) => {
            let stderr = String::from_utf8_lossy(&result.stderr);
            println!("cargo:warning=Failed to configure git hooks: {}", stderr);
        }
        Err(e) => {
            println!("cargo:warning=Failed to run git config: {}", e);
        }
    }
}
