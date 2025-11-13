# Architecture

## System Overview

hupasiya is a standalone Rust binary that orchestrates AI agent sessions by leveraging hannahanna for workbox management. The architecture follows a loose coupling pattern where hupasiya calls `hn` commands as an external tool.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                         User / AI Agent                      │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           ▼
                  ┌────────────────┐
                  │   hp (CLI)     │
                  │   hupasiya     │
                  └────────┬───────┘
                           │
                ┌──────────┴──────────┐
                │                     │
                ▼                     ▼
        ┌───────────────┐     ┌──────────────┐
        │  hn (CLI)     │     │  .hp/        │
        │  hannahanna   │     │  contexts/   │
        └───────┬───────┘     │  sessions/   │
                │             │  templates/  │
                ▼             └──────────────┘
        ┌───────────────┐
        │  .hn-state/   │     hupasiya manages:
        │  workboxes/   │     - Sessions
        │  docker/      │     - Context
        └───────────────┘     - Conversation
                              - PR integration
        hannahanna manages:   - Activity logs
        - Workboxes          - Metrics
        - VCS operations
        - Docker containers
        - Resource sharing
```

## Core Components

### 1. hupasiya Binary (`hp`)

**Responsibility:** Session orchestration, context management, AI coordination

**Implementation:**
- Standalone Rust binary
- No library dependency on hannahanna
- Calls `hn` commands via `std::process::Command`
- Parses `hn` output (JSON when available)

**Key Modules:**
- `session` - Session lifecycle management
- `context` - Context and conversation management
- `orchestration` - Multi-agent coordination (cascade, gather)
- `pr` - PR integration and shepherd workflow
- `hn_client` - Interface to hannahanna CLI
- `templates` - Template system
- `metrics` - Activity and metrics tracking

### 2. hannahanna Integration Layer

**Purpose:** Bridge between hupasiya and hannahanna CLI

**Pattern:**
```rust
use std::process::Command;
use anyhow::{Context, Result, bail};

pub struct HnClient {
    hn_command: String,  // Path to hn executable
}

impl HnClient {
    pub fn new() -> Result<Self> {
        // Find hn in PATH
        let hn_command = which::which("hn")
            .map_err(|_| anyhow!("hannahanna not found"))?
            .to_string_lossy()
            .to_string();

        Ok(Self { hn_command })
    }

    pub fn create_workbox(&self, name: &str, opts: &WorkboxOptions) -> Result<WorkboxInfo> {
        let mut cmd = Command::new(&self.hn_command);
        cmd.arg("add").arg(name);

        if let Some(from) = &opts.from {
            cmd.arg("--from").arg(from);
        }

        if let Some(vcs) = &opts.vcs {
            cmd.arg("--vcs").arg(vcs);
        }

        // Execute and parse output
        let output = cmd.output()
            .context("Failed to execute hn add")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("hn add failed: {}", stderr);
        }

        // Parse workbox info from output
        self.parse_workbox_info(&output.stdout)
    }

    pub fn get_workbox_info(&self, name: &str) -> Result<WorkboxInfo> {
        let output = Command::new(&self.hn_command)
            .arg("info")
            .arg(name)
            .arg("--format=json")
            .output()
            .context("Failed to execute hn info")?;

        if !output.status.success() {
            bail!("Workbox not found: {}", name);
        }

        serde_json::from_slice(&output.stdout)
            .context("Failed to parse hn info output")
    }

    pub fn list_workboxes(&self) -> Result<Vec<WorkboxInfo>> {
        let output = Command::new(&self.hn_command)
            .arg("list")
            .arg("--format=json")
            .output()
            .context("Failed to execute hn list")?;

        serde_json::from_slice(&output.stdout)
            .context("Failed to parse hn list output")
    }

    pub fn exec_in_workbox(&self, name: &str, command: &str) -> Result<String> {
        let output = Command::new(&self.hn_command)
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

    pub fn remove_workbox(&self, name: &str, force: bool) -> Result<()> {
        let mut cmd = Command::new(&self.hn_command);
        cmd.arg("remove").arg(name);

        if force {
            cmd.arg("--force");
        }

        let status = cmd.status()
            .context("Failed to execute hn remove")?;

        if !status.success() {
            bail!("Failed to remove workbox");
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkboxInfo {
    pub name: String,
    pub path: PathBuf,
    pub branch: String,
    pub base_branch: String,
    pub vcs_type: String,  // "git", "hg", or "jj"
    pub commit: String,
    pub docker_running: bool,
    pub docker_ports: Option<HashMap<String, u16>>,
}
```

### 3. Session Manager

**Responsibility:** Session lifecycle and metadata management

**Storage:**
- Session metadata: `.hp/sessions/<repo>-<session>.yaml`
- Context files: `.hp/contexts/<repo>/<session>/`

**Operations:**
- Create session (with workbox via hn)
- Update session metadata
- Track parent/child relationships
- Manage session status (active, paused, integrated, archived)

### 4. Context Manager

**Responsibility:** Context files, conversation history, snapshots

**Structure:**
```
.hp/contexts/<repo>/<session>/
├── context.md              # Main context
├── conversation.json       # AI chat history
├── shepherd.md            # PR comments (shepherd sessions)
├── activity.json          # Activity log
├── metrics.json           # Session metrics
├── snapshots/             # Context snapshots
│   ├── 2025-01-12_initial.md
│   └── 2025-01-15_after_review.md
└── .lock                  # Lock file
```

### 5. Orchestration Engine

**Responsibility:** Multi-agent coordination

**Key Operations:**

**Cascade (Parent → Children):**
```rust
pub fn cascade(parent: &Session, children: &[Session]) -> Result<()> {
    for child in children {
        // Get current state via hn
        let child_wb = hn_client.get_workbox_info(&child.workbox_name)?;

        // Merge parent branch into child using hn exec
        let merge_cmd = match child_wb.vcs_type.as_str() {
            "git" => format!("git merge {}", parent.branch),
            "hg" => format!("hg merge {}", parent.branch),
            "jj" => format!("jj rebase -d {}", parent.branch),
            _ => bail!("Unknown VCS type"),
        };

        hn_client.exec_in_workbox(&child.workbox_name, &merge_cmd)?;

        // Update child context
        update_child_context(child, parent)?;
    }
    Ok(())
}
```

**Gather (Children → Parent):**
```rust
pub fn gather(parent: &Session, children: &[Session]) -> Result<()> {
    for child in children {
        // Merge child into parent
        let merge_cmd = match parent.vcs_type.as_str() {
            "git" => format!("git merge {}", child.branch),
            "hg" => format!("hg merge {}", child.branch),
            "jj" => format!("jj rebase -s {} -d {}", child.branch, parent.branch),
            _ => bail!("Unknown VCS type"),
        };

        hn_client.exec_in_workbox(&parent.workbox_name, &merge_cmd)?;

        // Update parent context
        merge_child_context(parent, child)?;
    }
    Ok(())
}
```

### 6. PR Integration

**Responsibility:** GitHub/GitLab PR management and shepherd workflow

**Components:**
- PR creation and syncing
- Comment fetching and parsing
- Shepherd analysis (AI-powered comment resolution)
- Response drafting and posting

### 7. Template System

**Responsibility:** Context templates for different agent types

**Sources:**
- Local templates: `.hp/templates/local/`
- Marketplace templates: `.hp/templates/marketplace/`
- Template cache: `.hp/cache/templates/`

## Data Flow

### Session Creation Flow

```
User: hp new auth-feature
    │
    ▼
┌─────────────────────────────────┐
│ 1. Parse command & options      │
│ 2. Validate session name        │
└──────────────┬──────────────────┘
               │
               ▼
┌─────────────────────────────────┐
│ 3. Call: hn add auth-feature    │
│    --from=main --vcs=git        │
└──────────────┬──────────────────┘
               │
               ▼
        ┌──────────────┐
        │ hannahanna   │
        │ creates      │
        │ workbox      │
        └──────┬───────┘
               │
               ▼
┌─────────────────────────────────┐
│ 4. Parse hn output              │
│ 5. Extract workbox path         │
└──────────────┬──────────────────┘
               │
               ▼
┌─────────────────────────────────┐
│ 6. Create session metadata      │
│    .hp/sessions/repo-auth.yaml  │
└──────────────┬──────────────────┘
               │
               ▼
┌─────────────────────────────────┐
│ 7. Create context directory     │
│    .hp/contexts/repo/auth/      │
└──────────────┬──────────────────┘
               │
               ▼
┌─────────────────────────────────┐
│ 8. Apply context template       │
│ 9. Initialize activity log      │
│10. Open context in editor       │
└─────────────────────────────────┘
```

### Multi-Agent Cascade Flow

```
User: hp cascade parent-session
    │
    ▼
┌─────────────────────────────────┐
│ 1. Load parent session          │
│ 2. Load all child sessions      │
└──────────────┬──────────────────┘
               │
               ▼
    ┌──────────────────────┐
    │ For each child:      │
    └──────────┬───────────┘
               │
               ▼
┌─────────────────────────────────┐
│ 3. hn info child-workbox        │
│    Get current state            │
└──────────────┬──────────────────┘
               │
               ▼
┌─────────────────────────────────┐
│ 4. hn exec child-workbox --     │
│    git merge parent-branch      │
└──────────────┬──────────────────┘
               │
               ▼
┌─────────────────────────────────┐
│ 5. Handle conflicts (if any)    │
└──────────────┬──────────────────┘
               │
               ▼
┌─────────────────────────────────┐
│ 6. Update child context         │
│    Sync parent objectives       │
└──────────────┬──────────────────┘
               │
               ▼
┌─────────────────────────────────┐
│ 7. Log cascade event            │
└─────────────────────────────────┘
```

## Error Handling Strategy

### hannahanna Not Found

```rust
pub fn check_hn_installed() -> Result<()> {
    which::which("hn").map_err(|_| {
        anyhow!(
            "hannahanna (hn) not found.\n\
             \n\
             Install hannahanna:\n\
             cargo install hannahanna\n\
             \n\
             Or via package manager:\n\
             brew install hannahanna"
        )
    })?;
    Ok(())
}
```

### hannahanna Command Failed

When `hn` commands fail, provide context:
- What hupasiya was trying to do
- The exact hn command that failed
- The error output from hn
- Suggestions for resolution

### Workbox Missing

If workbox is deleted outside hupasiya:
- Detect via `hn info` failure
- Mark session as having missing workbox
- Provide recovery options:
  - Close session
  - Recreate workbox and reattach
  - Archive session

## Benefits of Loose Coupling

### Independence
- Version hupasiya and hannahanna separately
- Update one without breaking the other
- Different release cycles

### Development
- Develop in separate repos
- No shared Rust workspace
- Easier to maintain
- Clear boundaries

### Distribution
- Users install independently
- Can use different package managers
- Simpler dependency management

### Testing
- Test hupasiya with different hn versions
- Mock hn commands for unit tests
- Integration tests use real hn binary

### Flexibility
- Users can use different hn versions
- Could support alternative workbox managers
- Easier to add backends

## Configuration Integration

hupasiya reads its own config but can reference hannahanna config:

```yaml
# .hapusiyas.yml
hp:
  # hupasiya-specific config
  default_agent: feature

  # hannahanna CLI settings
  hn:
    command: hn  # Path to hn executable
    default_options:
      vcs: auto
    output_format: json

# Config hierarchy (4 levels, like hannahanna)
# System:  /etc/hapusiyas/config.yml
# User:    ~/.config/hp/config.yml
# Repo:    .hapusiyas.yml (committed)
# Local:   .hapusiyas.local.yml (gitignored)
```

## Performance Considerations

### Caching
- Cache workbox info to reduce `hn info` calls
- Refresh on demand or on timeout (30s)
- Invalidate on known state changes

### Batch Operations
- Batch multiple `hn exec` calls when possible
- Use `hn list` instead of multiple `hn info` calls

### Async Operations
- Use tokio for concurrent operations
- Launch AI tools asynchronously
- Parallel cascade/gather when no conflicts

## Security Considerations

### Sensitive Data
- Never store secrets in context files
- Git hooks to prevent committing `.hp/contexts/*/conversation.json`
- Sanitize command output before logging

### Command Injection
- Validate all inputs passed to `hn` commands
- Use proper command building (no string interpolation)
- Sanitize filenames and branch names

### File Permissions
- `.hp/contexts/` should be user-readable only
- Lock files use file system locks
- Respect umask settings
