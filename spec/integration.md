# Integration with hannahanna

How hupasiya integrates with hannahanna as a separate tool.

## Overview

hupasiya is built as a separate tool that calls `hn` commands via the shell. This loose coupling provides:

- **Independence**: Version and distribute separately
- **Flexibility**: Work with different hn versions
- **Simplicity**: Clear separation of concerns
- **Testing**: Easy to mock hn commands

## Architecture Pattern

Similar to how `gh` (GitHub CLI) calls `git` commands:

```
┌──────────┐
│    gh    │  GitHub CLI
└────┬─────┘
     │ calls
     ▼
┌──────────┐
│   git    │  Git CLI
└──────────┘
```

hupasiya follows the same pattern:

```
┌──────────┐
│    hp    │  hupasiya
└────┬─────┘
     │ calls
     ▼
┌──────────┐
│    hn    │  hannahanna
└──────────┘
```

## Installation Requirements

Both tools must be installed:

```bash
# Install hannahanna first
cargo install hannahanna

# Then install hupasiya
cargo install hupasiya

# Verify both
hn --version  # hannahanna 0.2.0
hp --version  # hupasiya 0.1.0
```

hupasiya checks for hn on startup:

```rust
pub fn check_hn_installed() -> Result<()> {
    which::which("hn").map_err(|_| {
        anyhow!(
            "hannahanna (hn) not found.\n\
             \n\
             Install hannahanna:\n\
             cargo install hannahanna\n\
             \n\
             Then try again."
        )
    })?;
    Ok(())
}
```

## Integration Points

### 1. Workbox Creation

When creating a session, hupasiya calls `hn add`:

```rust
pub fn create_workbox(name: &str, opts: &WorkboxOptions) -> Result<WorkboxInfo> {
    let mut cmd = Command::new("hn");
    cmd.arg("add").arg(name);

    if let Some(from) = &opts.from {
        cmd.arg("--from").arg(from);
    }

    if let Some(vcs) = &opts.vcs {
        cmd.arg("--vcs").arg(vcs);
    }

    if let Some(sparse) = &opts.sparse {
        for path in sparse {
            cmd.arg("--sparse").arg(path);
        }
    }

    let output = cmd.output()
        .context("Failed to execute 'hn add'")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("hn add failed: {}", stderr);
    }

    parse_workbox_info(&output.stdout)
}
```

### 2. Workbox Information

Get workbox status via `hn info`:

```rust
pub fn get_workbox_info(name: &str) -> Result<WorkboxInfo> {
    let output = Command::new("hn")
        .arg("info")
        .arg(name)
        .arg("--format=json")
        .output()
        .context("Failed to execute 'hn info'")?;

    if !output.status.success() {
        bail!("Workbox not found: {}", name);
    }

    serde_json::from_slice(&output.stdout)
        .context("Failed to parse hn info output")
}
```

Expected JSON format from `hn info --format=json`:

```json
{
  "name": "auth-feature",
  "path": "/path/to/repo-wt-auth-feature",
  "branch": "feature/auth",
  "base_branch": "main",
  "vcs_type": "git",
  "commit": "a1b2c3d4",
  "docker_running": true,
  "docker_ports": {
    "app": 3000,
    "db": 5432
  },
  "status": {
    "dirty": false,
    "untracked": 0,
    "modified": 0,
    "staged": 0
  }
}
```

### 3. Listing Workboxes

Get all workboxes via `hn list`:

```rust
pub fn list_workboxes() -> Result<Vec<WorkboxInfo>> {
    let output = Command::new("hn")
        .arg("list")
        .arg("--format=json")
        .output()
        .context("Failed to execute 'hn list'")?;

    if !output.status.success() {
        bail!("Failed to list workboxes");
    }

    serde_json::from_slice(&output.stdout)
        .context("Failed to parse hn list output")
}
```

Expected JSON format from `hn list --format=json`:

```json
[
  {
    "name": "auth-feature",
    "path": "/path/to/repo-wt-auth-feature",
    "branch": "feature/auth",
    "base_branch": "main",
    "vcs_type": "git",
    "commit": "a1b2c3d4",
    "docker_running": true,
    "docker_ports": { "app": 3000 }
  },
  {
    "name": "bugfix-oauth",
    "path": "/path/to/repo-wt-bugfix-oauth",
    "branch": "fix/oauth",
    "base_branch": "main",
    "vcs_type": "git",
    "commit": "e5f6g7h8",
    "docker_running": false,
    "docker_ports": null
  }
]
```

### 4. Executing Commands in Workbox

Run commands inside workbox via `hn exec`:

```rust
pub fn exec_in_workbox(name: &str, command: &str) -> Result<String> {
    let output = Command::new("hn")
        .arg("exec")
        .arg(name)
        .arg("--")
        .arg("sh")
        .arg("-c")
        .arg(command)
        .output()
        .context("Failed to execute command in workbox")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Command failed: {}", stderr);
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
```

Example usage:

```rust
// Get git status
let status = exec_in_workbox("auth-feature", "git status --short")?;

// Merge parent branch
let merge_cmd = "git merge main";
exec_in_workbox("auth-feature", merge_cmd)?;

// Run tests
exec_in_workbox("auth-tests", "npm test")?;
```

### 5. Removing Workbox

Remove workbox via `hn remove`:

```rust
pub fn remove_workbox(name: &str, force: bool) -> Result<()> {
    let mut cmd = Command::new("hn");
    cmd.arg("remove").arg(name);

    if force {
        cmd.arg("--force");
    }

    let status = cmd.status()
        .context("Failed to execute 'hn remove'")?;

    if !status.success() {
        bail!("Failed to remove workbox");
    }

    Ok(())
}
```

### 6. Docker Operations

Docker operations via hn commands:

```rust
// Start docker for workbox
pub fn start_docker(name: &str) -> Result<()> {
    Command::new("hn")
        .arg("docker")
        .arg("start")
        .arg(name)
        .status()?;
    Ok(())
}

// Get docker ports
pub fn get_docker_ports(name: &str) -> Result<HashMap<String, u16>> {
    let info = get_workbox_info(name)?;
    Ok(info.docker_ports.unwrap_or_default())
}

// Docker logs
pub fn docker_logs(name: &str) -> Result<String> {
    exec_in_workbox(name, "hn docker logs")
}
```

## Multi-VCS Support

hupasiya leverages hannahanna's multi-VCS support:

```rust
pub fn cascade_to_child(parent: &Session, child: &Session) -> Result<()> {
    // Get merge command based on VCS type
    let merge_cmd = match child.vcs_type.as_str() {
        "git" => format!("git merge {}", parent.branch),
        "hg" => format!("hg merge {}", parent.branch),
        "jj" => format!("jj rebase -d {}", parent.branch),
        _ => bail!("Unknown VCS type: {}", child.vcs_type),
    };

    // Execute via hn
    exec_in_workbox(&child.workbox_name, &merge_cmd)?;

    Ok(())
}
```

hannahanna handles VCS-specific details, hupasiya just needs to know which VCS is being used.

## Output Parsing

### Text Output Parsing

When `--format=json` is not available:

```rust
pub fn parse_workbox_path(output: &str) -> Result<PathBuf> {
    // Parse text output like:
    // Created workbox 'auth-feature'
    // Path: /path/to/repo-wt-auth-feature
    // Branch: feature/auth

    for line in output.lines() {
        if line.starts_with("Path:") {
            let path = line.strip_prefix("Path:").unwrap().trim();
            return Ok(PathBuf::from(path));
        }
    }

    bail!("Could not parse workbox path from hn output");
}
```

### JSON Output Parsing

Preferred method when available:

```rust
#[derive(Debug, Deserialize)]
pub struct HnAddOutput {
    pub name: String,
    pub path: PathBuf,
    pub branch: String,
    pub vcs_type: String,
}

pub fn parse_workbox_info_json(output: &[u8]) -> Result<WorkboxInfo> {
    serde_json::from_slice(output)
        .context("Failed to parse hn output as JSON")
}
```

## Error Handling

### hn Not Found

```rust
if which::which("hn").is_err() {
    eprintln!("Error: hannahanna (hn) not found");
    eprintln!();
    eprintln!("hupasiya requires hannahanna to be installed.");
    eprintln!();
    eprintln!("Install hannahanna:");
    eprintln!("  cargo install hannahanna");
    eprintln!();
    eprintln!("Or via package manager:");
    eprintln!("  brew install hannahanna");
    std::process::exit(1);
}
```

### hn Command Failed

```rust
match cmd.status() {
    Ok(status) if status.success() => Ok(()),
    Ok(status) => {
        eprintln!("Error: hannahanna command failed");
        eprintln!();
        eprintln!("Command: hn {}", args.join(" "));
        eprintln!("Exit code: {}", status.code().unwrap_or(-1));
        eprintln!();
        eprintln!("This might mean:");
        eprintln!("  - Workbox already exists");
        eprintln!("  - Invalid branch name");
        eprintln!("  - VCS error");
        eprintln!();
        eprintln!("Run 'hn {}' directly to see full error", args.join(" "));
        Err(anyhow!("hn command failed"))
    }
    Err(e) => {
        eprintln!("Error: Failed to execute hannahanna");
        eprintln!();
        eprintln!("Make sure hannahanna is installed:");
        eprintln!("  cargo install hannahanna");
        Err(anyhow!("Failed to execute hn: {}", e))
    }
}
```

### Workbox Missing

```rust
pub fn verify_workbox_exists(session: &Session) -> Result<()> {
    match get_workbox_info(&session.workbox_name) {
        Ok(_) => Ok(()),
        Err(_) => {
            eprintln!("Error: Workbox missing for session '{}'", session.name);
            eprintln!();
            eprintln!("Session exists but workbox was removed.");
            eprintln!();
            eprintln!("This can happen if:");
            eprintln!("  - You ran 'hn remove' directly");
            eprintln!("  - Workbox was deleted manually");
            eprintln!("  - Filesystem was cleaned");
            eprintln!();
            eprintln!("Fix:");
            eprintln!("  1. Close session: hp close {}", session.name);
            eprintln!("  2. Recreate: hp new {} --from={}", session.name, session.base_branch);
            eprintln!();
            eprintln!("Or recreate workbox and reattach:");
            eprintln!("  1. Create workbox: hn add {}", session.workbox_name);
            eprintln!("  2. Reattach session: hp reattach {}", session.name);

            Err(anyhow!("Workbox missing"))
        }
    }
}
```

## Configuration Integration

hupasiya can reference hannahanna config but doesn't parse it directly:

```yaml
# .hapusiyas.yml
hp:
  # hannahanna CLI settings
  hn:
    command: hn  # Path to hn executable
    default_options:
      vcs: auto  # Passed to hn commands
    output_format: json  # Request JSON from hn

  # hupasiya-specific settings
  sessions:
    metadata_dir: ~/.config/hp/sessions
    context_dir: .hp/contexts
```

hupasiya passes options to hn but doesn't interpret them:

```rust
// Read from config
let vcs_option = config.hp.hn.default_options.get("vcs");

// Pass to hn
if let Some(vcs) = vcs_option {
    cmd.arg("--vcs").arg(vcs);
}
```

## Version Compatibility

### Semantic Versioning

hupasiya follows semver for hn compatibility:

- **Major version**: Breaking changes in hn integration (e.g., hn 2.0 changes output format)
- **Minor version**: New features using existing hn commands
- **Patch version**: Bug fixes, no hn integration changes

### Compatibility Matrix

| hupasiya | hannahanna | Status |
|----------|------------|--------|
| 0.1.x    | 0.2.x      | ✅ Supported |
| 0.1.x    | 0.1.x      | ⚠️ Limited (no JSON output) |
| 0.1.x    | 1.0.x      | ✅ Supported |
| 1.0.x    | 0.2.x      | ❌ Not supported |

### Version Check

```rust
pub fn check_hn_version() -> Result<String> {
    let output = Command::new("hn")
        .arg("--version")
        .output()
        .context("Failed to get hn version")?;

    let version = String::from_utf8_lossy(&output.stdout);
    let version = version.trim();

    // Extract version number
    let version_num = version.split_whitespace()
        .last()
        .ok_or_else(|| anyhow!("Could not parse hn version"))?;

    Ok(version_num.to_string())
}

pub fn verify_hn_compatibility() -> Result<()> {
    let version = check_hn_version()?;
    let semver = semver::Version::parse(&version)
        .context("Could not parse hn version")?;

    // Require hn >= 0.2.0
    if semver < semver::Version::new(0, 2, 0) {
        bail!(
            "hannahanna version {} is too old. Please upgrade:\n\
             cargo install hannahanna --force",
            version
        );
    }

    Ok(())
}
```

## Testing Strategy

### Unit Tests with Mocks

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Mock hn commands for testing
    fn mock_hn_add(name: &str) -> Result<WorkboxInfo> {
        Ok(WorkboxInfo {
            name: name.to_string(),
            path: PathBuf::from(format!("/tmp/{}", name)),
            branch: format!("feature/{}", name),
            base_branch: "main".to_string(),
            vcs_type: "git".to_string(),
            commit: "abc123".to_string(),
            docker_running: false,
            docker_ports: None,
        })
    }

    #[test]
    fn test_create_session() {
        // Use mock instead of real hn command
        let workbox = mock_hn_add("test-session").unwrap();
        assert_eq!(workbox.name, "test-session");
    }
}
```

### Integration Tests with Real hn

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    #[ignore] // Run only when hn is available
    fn test_real_hn_integration() {
        // Check if hn is installed
        if which::which("hn").is_err() {
            eprintln!("Skipping: hn not found");
            return;
        }

        // Test real hn commands
        let result = Command::new("hn")
            .arg("--version")
            .output();

        assert!(result.is_ok());
    }
}
```

## Benefits of Integration Approach

### 1. Independence
- hupasiya and hannahanna can be updated separately
- No library version conflicts
- Clear API boundary

### 2. Simplicity
- No complex Rust workspace setup
- Easy to understand integration points
- Standard command-line interface

### 3. Flexibility
- Users can use different versions of each tool
- Could support alternative workbox managers
- Easy to add new features without coordinating releases

### 4. Testing
- Can mock hn commands for unit tests
- Integration tests use real hn binary
- Clear separation of test scopes

### 5. Distribution
- Install via different package managers
- No bundling required
- Simpler dependency management

## Future Enhancements

### 1. hn Plugin API

If hannahanna adds plugin support, hupasiya could become a plugin:

```rust
// Future: hupasiya as hn plugin
// hn plugin install hupasiya
// hn hp new auth-feature
```

### 2. Shared Library

For performance, could share a common library:

```toml
[dependencies]
hannahanna-core = "0.2"  # Shared types and utilities
```

But still maintain CLI separation:
- `hn` binary uses `hannahanna-core`
- `hp` binary uses `hannahanna-core`
- No direct dependency between binaries

### 3. Event System

If hannahanna adds event hooks:

```yaml
# .hannahanna.yml
hooks:
  post_add:
    - hp session-created $WORKBOX_NAME
  pre_remove:
    - hp session-cleanup $WORKBOX_NAME
```

hupasiya could respond to hn events automatically.
