# Data Model

## Core Entities

### Session

The primary entity representing an AI agent session with its workbox and context.

```rust
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    // === Identity ===
    /// Unique session identifier
    pub id: Uuid,

    /// Human-readable session name (e.g., "auth-feature")
    pub name: String,

    /// Timestamp when session was created
    pub created: DateTime<Utc>,

    /// Timestamp of last activity
    pub last_active: DateTime<Utc>,

    // === Type & Status ===
    /// Type of agent (feature, bugfix, test, etc.)
    pub agent_type: AgentType,

    /// Current status (active, paused, etc.)
    pub status: SessionStatus,

    // === Workbox (managed by hannahanna) ===
    /// Workbox name (used for hn commands)
    pub workbox_name: String,

    /// Cached workbox path (refreshed from hn)
    pub workbox_path: PathBuf,

    /// Branch name (refreshed from hn)
    pub branch: String,

    /// Base branch (e.g., "main")
    pub base_branch: String,

    /// Repository name
    pub repo_name: String,

    /// VCS type: "git", "hg", or "jj"
    pub vcs_type: String,

    // === Relationships (Multi-agent) ===
    /// Parent session name (if child)
    pub parent: Option<String>,

    /// Child session names
    pub children: Vec<String>,

    // === Context (managed by hupasiya) ===
    /// Context directory path
    pub context_dir: PathBuf,

    /// Conversation history
    pub conversation_history: Vec<Message>,

    /// Context snapshots
    pub context_snapshots: Vec<SnapshotInfo>,

    // === PR Integration ===
    /// Associated PR number
    pub pr_number: Option<u64>,

    /// PR URL
    pub pr_url: Option<String>,

    /// PR status
    pub pr_status: Option<PrStatus>,

    /// Unresolved PR review comments
    pub unresolved_comments: Vec<ReviewComment>,

    // === Activity & Metrics ===
    /// Activity log
    pub activity_log: Vec<ActivityEvent>,

    /// Session metrics
    pub metrics: SessionMetrics,

    // === Metadata ===
    /// User-defined tags
    pub tags: Vec<String>,

    /// User notes
    pub notes: String,

    /// Lock info (username@hostname)
    pub locked_by: Option<String>,
}

impl Session {
    /// Create a new session
    pub fn new(
        name: String,
        agent_type: AgentType,
        workbox_name: String,
        workbox_path: PathBuf,
        branch: String,
        base_branch: String,
        repo_name: String,
        vcs_type: String,
    ) -> Self {
        let now = Utc::now();
        let context_dir = PathBuf::from(format!(".hp/contexts/{}/{}", repo_name, name));

        Self {
            id: Uuid::new_v4(),
            name,
            created: now,
            last_active: now,
            agent_type,
            status: SessionStatus::Active,
            workbox_name,
            workbox_path,
            branch,
            base_branch,
            repo_name,
            vcs_type,
            parent: None,
            children: Vec::new(),
            context_dir,
            conversation_history: Vec::new(),
            context_snapshots: Vec::new(),
            pr_number: None,
            pr_url: None,
            pr_status: None,
            unresolved_comments: Vec::new(),
            activity_log: Vec::new(),
            metrics: SessionMetrics::default(),
            tags: Vec::new(),
            notes: String::new(),
            locked_by: None,
        }
    }

    /// Update last active timestamp
    pub fn touch(&mut self) {
        self.last_active = Utc::now();
    }

    /// Add activity event
    pub fn log_activity(&mut self, event_type: ActivityType, details: String) {
        self.activity_log.push(ActivityEvent {
            timestamp: Utc::now(),
            event_type,
            details,
        });
        self.touch();
    }

    /// Check if session is locked
    pub fn is_locked(&self) -> bool {
        self.locked_by.is_some()
    }
}
```

### AgentType

Types of AI agents that can be spawned.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AgentType {
    /// Building new feature
    Feature,

    /// Fixing bugs
    Bugfix,

    /// Code review
    Review,

    /// Investigation/spike
    Research,

    /// Refactoring code
    Refactor,

    /// Writing tests
    Test,

    /// Documentation
    Docs,

    /// PR comment resolution
    Shepherd,

    /// Custom agent type
    Custom(String),
}

impl AgentType {
    /// Get default context template for this agent type
    pub fn default_template(&self) -> &str {
        match self {
            Self::Feature => "feature",
            Self::Bugfix => "bugfix",
            Self::Review => "review",
            Self::Research => "research",
            Self::Refactor => "refactor",
            Self::Test => "test",
            Self::Docs => "docs",
            Self::Shepherd => "shepherd",
            Self::Custom(name) => name,
        }
    }
}
```

### SessionStatus

Current status of a session.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SessionStatus {
    /// Session is actively being worked on
    Active,

    /// Session is paused
    Paused,

    /// Work has been integrated into base branch
    Integrated,

    /// Session has been archived
    Archived,

    /// Session has been abandoned
    Abandoned,
}
```

### Message

A message in the conversation history.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message role
    pub role: Role,

    /// Message content
    pub content: String,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Session ID
    pub session_id: Uuid,

    /// Token count (if available)
    pub tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}
```

### SessionMetrics

Metrics tracked for a session.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionMetrics {
    /// Total active time
    pub total_time: Duration,

    /// Number of commits
    pub commits: u32,

    /// Lines added
    pub lines_added: u32,

    /// Lines removed
    pub lines_removed: u32,

    /// Files changed
    pub files_changed: u32,

    /// Number of AI interactions
    pub ai_interactions: u32,

    /// Total tokens used
    pub tokens_used: u64,
}

impl SessionMetrics {
    /// Update metrics from git diff stats
    pub fn update_from_git_stats(&mut self, added: u32, removed: u32, files: u32) {
        self.lines_added += added;
        self.lines_removed += removed;
        self.files_changed = files;
    }

    /// Record AI interaction
    pub fn record_ai_interaction(&mut self, tokens: u64) {
        self.ai_interactions += 1;
        self.tokens_used += tokens;
    }

    /// Add time
    pub fn add_time(&mut self, duration: Duration) {
        self.total_time = self.total_time + duration;
    }
}
```

### ActivityEvent

An event in the activity log.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEvent {
    /// When the event occurred
    pub timestamp: DateTime<Utc>,

    /// Type of event
    pub event_type: ActivityType,

    /// Event details
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityType {
    SessionCreated,
    ContextEdited,
    AiLaunched,
    CommitMade,
    PrCreated,
    PrCommentReceived,
    ShepherdRun,
    Cascaded,
    Gathered,
    Integrated,
    StatusChanged,
    ParentLinked,
    ChildAdded,
}
```

### SnapshotInfo

Information about a context snapshot.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    /// Snapshot name
    pub name: String,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// File path
    pub path: PathBuf,

    /// Description
    pub description: Option<String>,

    /// Trigger that created this snapshot
    pub trigger: SnapshotTrigger,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotTrigger {
    Manual,
    BeforeCascade,
    BeforeGather,
    AfterPrReview,
    OnPause,
}
```

## PR Integration

### PrStatus

Status of an associated pull request.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PrStatus {
    Open,
    Draft,
    Merged,
    Closed,
}
```

### ReviewComment

A PR review comment.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewComment {
    /// Comment ID
    pub id: u64,

    /// File path
    pub path: String,

    /// Line number
    pub line: Option<u32>,

    /// Comment body
    pub body: String,

    /// Author
    pub author: String,

    /// Timestamp
    pub created_at: DateTime<Utc>,

    /// Is resolved
    pub resolved: bool,

    /// Diff hunk
    pub diff_hunk: Option<String>,
}
```

### ShepherdAnalysis

AI analysis of a PR comment.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShepherdAnalysis {
    /// Comment ID
    pub comment_id: u64,

    /// Recommended action
    pub action: ShepherdAction,

    /// Confidence level
    pub confidence: ConfidenceLevel,

    /// Summary
    pub summary: String,

    /// Assessment
    pub assessment: String,

    /// Code changes (if action is Fix)
    pub changes: Option<String>,

    /// Response to post
    pub response: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ShepherdAction {
    /// Fix the issue with code changes
    Fix,

    /// Clarify requirements
    Clarify,

    /// Acknowledge without changes
    Acknowledge,

    /// Defer to later
    Defer,

    /// Disagree with comment
    Disagree,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfidenceLevel {
    High,
    Medium,
    Low,
}
```

## Workbox Integration

### WorkboxInfo

Information about a workbox (from hannahanna).

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkboxInfo {
    /// Workbox name
    pub name: String,

    /// Path to workbox
    pub path: PathBuf,

    /// Current branch
    pub branch: String,

    /// Base branch
    pub base_branch: String,

    /// VCS type ("git", "hg", or "jj")
    pub vcs_type: String,

    /// Current commit
    pub commit: String,

    /// Docker running status
    pub docker_running: bool,

    /// Docker ports (if running)
    pub docker_ports: Option<HashMap<String, u16>>,

    /// Working directory status
    pub status: WorkboxStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkboxStatus {
    /// Has uncommitted changes
    pub dirty: bool,

    /// Untracked files count
    pub untracked: u32,

    /// Modified files count
    pub modified: u32,

    /// Staged files count
    pub staged: u32,
}
```

## Templates

### Template

A context template.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    /// Template name
    pub name: String,

    /// Template content
    pub content: String,

    /// Template variables
    pub variables: Vec<String>,

    /// Template metadata
    pub metadata: TemplateMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    /// Author
    pub author: String,

    /// Version
    pub version: String,

    /// Description
    pub description: String,

    /// Agent types this template is for
    pub agent_types: Vec<AgentType>,

    /// Tags
    pub tags: Vec<String>,
}
```

## Configuration

### Config

Application configuration.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// hupasiya config
    pub hp: HpConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HpConfig {
    /// Default agent type
    pub default_agent: AgentType,

    /// hannahanna CLI settings
    pub hn: HnConfig,

    /// Session management
    pub sessions: SessionConfig,

    /// AI tool integration
    pub ai_tool: AiToolConfig,

    /// Multi-agent orchestration
    pub orchestration: OrchestrationConfig,

    /// PR integration
    pub pr: PrConfig,

    /// Templates
    pub templates: TemplateConfig,

    /// Configuration profiles
    pub profiles: HashMap<String, ProfileConfig>,

    /// Active profile
    pub active_profile: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HnConfig {
    /// Path to hn executable (auto-detected if not specified)
    #[serde(default = "default_hn_command")]
    pub command: String,

    /// Default options to pass to hn commands
    #[serde(default)]
    pub default_options: HashMap<String, String>,

    /// Output format to request from hn
    #[serde(default = "default_output_format")]
    pub output_format: String,
}

fn default_hn_command() -> String {
    "hn".to_string()
}

fn default_output_format() -> String {
    "json".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Where to store session metadata
    pub metadata_dir: PathBuf,

    /// Where to store contexts
    pub context_dir: PathBuf,

    /// Auto-sync with parent on commit
    #[serde(default)]
    pub auto_sync: bool,

    /// Auto-snapshot context before major operations
    #[serde(default)]
    pub auto_snapshot: bool,

    /// Snapshot triggers
    #[serde(default)]
    pub snapshot_triggers: Vec<SnapshotTrigger>,

    /// Activity log settings
    pub activity_log: ActivityLogConfig,

    /// Metrics settings
    pub metrics: MetricsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityLogConfig {
    pub enabled: bool,
    pub retention_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub track_tokens: bool,
    pub track_time: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiToolConfig {
    /// AI tool command
    pub command: String,

    /// Launch method
    pub launch_method: LaunchMethod,

    /// Context strategy
    pub context_strategy: ContextStrategy,

    /// Extra args
    #[serde(default)]
    pub extra_args: Vec<String>,

    /// Environment variables
    #[serde(default)]
    pub env: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LaunchMethod {
    Exec,
    ShellFunction,
    Tmux,
    Screen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextStrategy {
    SlashCommand,
    Flag,
    Env,
    File,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationConfig {
    pub enabled: bool,
    pub cascade_on_commit: bool,
    pub gather_strategy: GatherStrategy,
    pub conflict_strategy: ConflictStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GatherStrategy {
    Manual,
    Auto,
    PrReady,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictStrategy {
    Prompt,
    ParentWins,
    ChildWins,
    Abort,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrConfig {
    pub github: GitHubConfig,
    pub auto_create: bool,
    pub shepherd: ShepherdConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    pub org: String,
    pub default_reviewers: Vec<String>,
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShepherdConfig {
    pub auto_apply_safe: bool,
    pub confidence_threshold: ConfidenceLevel,
    pub auto_post_responses: bool,
    pub analysis_model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub local: HashMap<String, PathBuf>,
    pub marketplace: MarketplaceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceConfig {
    pub enabled: bool,
    pub registry_url: String,
    pub auto_update: bool,
    pub cache_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub hn: Option<HnConfig>,
    pub ai_tool: Option<AiToolConfig>,
    pub pr: Option<PrConfig>,
    pub orchestration: Option<OrchestrationConfig>,
}
```

## Serialization Format

All session metadata is stored as YAML:

```yaml
# .hp/sessions/myrepo-auth-feature.yaml
id: "550e8400-e29b-41d4-a716-446655440000"
name: "auth-feature"
created: "2025-01-12T14:30:00Z"
last_active: "2025-01-15T09:15:00Z"
agent_type: "feature"
status: "active"
workbox_name: "auth-feature"
workbox_path: "/path/to/repo-wt-auth-feature"
branch: "feature/auth"
base_branch: "main"
repo_name: "myrepo"
vcs_type: "git"
parent: null
children:
  - "auth-tests"
  - "auth-docs"
context_dir: ".hp/contexts/myrepo/auth-feature"
pr_number: 123
pr_url: "https://github.com/org/repo/pull/123"
pr_status: "open"
tags:
  - "oauth"
  - "authentication"
notes: "Implementing OAuth 2.0 flow"
locked_by: null
```

Conversation history is stored as JSON for efficiency:

```json
{
  "messages": [
    {
      "role": "user",
      "content": "Implement OAuth login",
      "timestamp": "2025-01-12T14:35:00Z",
      "session_id": "550e8400-e29b-41d4-a716-446655440000",
      "tokens": null
    },
    {
      "role": "assistant",
      "content": "I'll implement OAuth login...",
      "timestamp": "2025-01-12T14:35:15Z",
      "session_id": "550e8400-e29b-41d4-a716-446655440000",
      "tokens": 1234
    }
  ]
}
```
