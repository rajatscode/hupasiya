# Configuration

Configuration file format and hierarchy for hupasiya.

## Configuration Files

hupasiya uses a 4-level configuration hierarchy, similar to hannahanna:

1. **System**: `/etc/hapusiyas/config.yml`
2. **User**: `~/.config/hp/config.yml`
3. **Repo**: `.hapusiyas.yml` (committed to git)
4. **Local**: `.hapusiyas.local.yml` (gitignored, highest priority)

## Configuration Hierarchy

Higher levels override lower levels:

```
Local (.hapusiyas.local.yml)    ← Highest priority
    ↓
Repo (.hapusiyas.yml)
    ↓
User (~/.config/hp/config.yml)
    ↓
System (/etc/hapusiyas/config.yml)   ← Lowest priority
```

### Merge Strategy

- **Objects**: Deep merge (keys from higher levels override/extend lower levels)
- **Arrays**: Append (arrays from all levels combined)
- **Primitives**: Override (higher level wins)

### Example Merge

**System config**:
```yaml
hp:
  default_agent: feature
  sessions:
    auto_snapshot: false
  ai_tool:
    command: claude-code
```

**User config**:
```yaml
hp:
  sessions:
    auto_snapshot: true  # Override
    metadata_dir: ~/.config/hp/sessions  # Add
```

**Result**:
```yaml
hp:
  default_agent: feature  # From system
  sessions:
    auto_snapshot: true   # From user (overrides system)
    metadata_dir: ~/.config/hp/sessions  # From user
  ai_tool:
    command: claude-code  # From system
```

## Full Configuration Schema

```yaml
# .hapusiyas.yml

hp:
  # Default agent type for new sessions
  default_agent: feature  # feature, bugfix, test, docs, etc.

  # hannahanna CLI integration settings
  hn:
    # Path to hn executable (auto-detected if not specified)
    command: hn

    # Default options to pass to hn commands
    default_options:
      vcs: auto  # auto, git, hg, jj

    # Output format to request from hn
    output_format: json  # json or text

  # Session management settings
  sessions:
    # Where to store session metadata
    metadata_dir: ~/.config/hp/sessions

    # Where to store contexts (relative to repo root)
    context_dir: .hp/contexts

    # Auto-sync with parent on commit
    auto_sync: false

    # Auto-snapshot context before major operations
    auto_snapshot: true

    # When to auto-snapshot
    snapshot_triggers:
      - before_cascade
      - before_gather
      - after_pr_review
      - on_pause

    # Activity log settings
    activity_log:
      enabled: true
      retention_days: 90  # Keep logs for 90 days

    # Metrics collection settings
    metrics:
      enabled: true
      track_tokens: true  # Track AI token usage
      track_time: true    # Track time spent

  # AI tool integration
  ai_tool:
    # Which AI tool to launch
    command: claude-code  # claude-code, cursor, codex, etc.

    # How to launch it
    launch_method: exec  # exec, shell_function, tmux, screen

    # How to pass context to AI tool
    context_strategy: slash_command  # slash_command, flag, env, file

    # Additional arguments to pass
    extra_args: []

    # Environment variables to set
    env:
      HP_SESSION: "{{session_name}}"
      HP_CONTEXT: "{{context_file}}"
      HP_WORKBOX: "{{workbox_path}}"
      HP_VCS: "{{vcs_type}}"
      HP_REPO: "{{repo_name}}"
      HP_BRANCH: "{{branch}}"
      HP_BASE_BRANCH: "{{base_branch}}"

  # Multi-agent orchestration settings
  orchestration:
    # Enable orchestration features
    enabled: true

    # Auto-cascade when parent commits
    cascade_on_commit: false

    # Strategy for gathering children
    gather_strategy: manual  # manual, auto, pr_ready

    # Conflict resolution strategy
    conflict_strategy: prompt  # prompt, parent_wins, child_wins, abort

  # PR integration settings
  pr:
    # GitHub settings
    github:
      org: yourorg
      default_reviewers: []
      labels:
        - agent-created

    # Auto-create PR when session is ready
    auto_create: false

    # Shepherd (PR comment resolution) settings
    shepherd:
      # Auto-apply fixes with high confidence
      auto_apply_safe: false

      # Minimum confidence for auto-apply
      confidence_threshold: high  # high, medium, low

      # Auto-post responses to PR comments
      auto_post_responses: false

      # AI model to use for analysis
      analysis_model: claude-3-5-sonnet

  # Template settings
  templates:
    # Local template paths
    local:
      feature: .hp/templates/local/feature.md
      bugfix: .hp/templates/local/bugfix.md
      test: .hp/templates/local/test.md
      docs: .hp/templates/local/docs.md
      shepherd: .hp/templates/local/shepherd.md
      refactor: .hp/templates/local/refactor.md
      research: .hp/templates/local/research.md
      review: .hp/templates/local/review.md

    # Template marketplace settings
    marketplace:
      enabled: true
      registry_url: https://hp-templates.dev
      auto_update: true
      cache_dir: .hp/cache/templates

  # Configuration profiles
  profiles:
    # Development profile
    dev:
      hn:
        default_options:
          docker: auto-start
      ai_tool:
        command: claude-code
        extra_args: []

    # Staging profile
    staging:
      hn:
        default_options:
          docker: manual
      pr:
        auto_create: true
        github:
          labels:
            - agent-created
            - staging

    # Production profile
    prod:
      hn:
        default_options:
          docker: manual
      orchestration:
        cascade_on_commit: false
      pr:
        auto_create: false
        shepherd:
          auto_apply_safe: false

  # Active profile (defaults to "dev")
  active_profile: dev
```

## Configuration Sections

### hannahanna Integration (`hp.hn`)

Settings for integrating with hannahanna CLI.

```yaml
hp:
  hn:
    command: hn  # Path to hn executable
    default_options:
      vcs: auto  # Default VCS type
      docker: auto-start  # Docker behavior
    output_format: json  # Prefer JSON output
```

**Options**:
- `command`: Path to `hn` executable (default: `hn`, auto-detected from PATH)
- `default_options`: Key-value pairs passed to `hn` commands (e.g., `--vcs=auto`)
- `output_format`: `json` or `text` (JSON preferred for parsing)

### Session Management (`hp.sessions`)

Settings for session lifecycle and storage.

```yaml
hp:
  sessions:
    metadata_dir: ~/.config/hp/sessions
    context_dir: .hp/contexts
    auto_sync: false
    auto_snapshot: true
    snapshot_triggers:
      - before_cascade
      - before_gather
      - after_pr_review
      - on_pause
    activity_log:
      enabled: true
      retention_days: 90
    metrics:
      enabled: true
      track_tokens: true
      track_time: true
```

**Options**:
- `metadata_dir`: Where to store `.yaml` session metadata files
- `context_dir`: Where to store context files (relative to repo root)
- `auto_sync`: Auto-sync with parent on commit
- `auto_snapshot`: Create snapshots automatically
- `snapshot_triggers`: When to create auto-snapshots
- `activity_log.enabled`: Enable activity logging
- `activity_log.retention_days`: How long to keep logs
- `metrics.enabled`: Enable metrics tracking
- `metrics.track_tokens`: Track AI token usage
- `metrics.track_time`: Track time spent

### AI Tool Integration (`hp.ai_tool`)

Settings for launching and integrating with AI tools.

```yaml
hp:
  ai_tool:
    command: claude-code
    launch_method: exec
    context_strategy: slash_command
    extra_args: []
    env:
      HP_SESSION: "{{session_name}}"
      HP_CONTEXT: "{{context_file}}"
```

**Options**:
- `command`: AI tool command (`claude-code`, `cursor`, `codex`, etc.)
- `launch_method`: How to launch
  - `exec`: Direct execution
  - `shell_function`: Call shell function
  - `tmux`: Launch in tmux pane
  - `screen`: Launch in screen session
- `context_strategy`: How to pass context
  - `slash_command`: Use tool's slash command system
  - `flag`: Pass as command-line flag
  - `env`: Environment variables only
  - `file`: Write to tool's config file
- `extra_args`: Additional arguments
- `env`: Environment variables to set (supports template variables)

**Template Variables**:
- `{{session_name}}`: Session name
- `{{context_file}}`: Path to context.md
- `{{workbox_path}}`: Path to workbox
- `{{vcs_type}}`: VCS type (git, hg, jj)
- `{{repo_name}}`: Repository name
- `{{branch}}`: Branch name
- `{{base_branch}}`: Base branch name

### Orchestration (`hp.orchestration`)

Settings for multi-agent coordination.

```yaml
hp:
  orchestration:
    enabled: true
    cascade_on_commit: false
    gather_strategy: manual
    conflict_strategy: prompt
```

**Options**:
- `enabled`: Enable orchestration features
- `cascade_on_commit`: Auto-cascade to children when parent commits
- `gather_strategy`: When to gather children
  - `manual`: Only on explicit `hp gather` command
  - `auto`: Automatically when children are ready
  - `pr_ready`: When creating PR
- `conflict_strategy`: How to handle merge conflicts
  - `prompt`: Ask user interactively
  - `parent_wins`: Prefer parent changes
  - `child_wins`: Prefer child changes
  - `abort`: Abort on conflict

### PR Integration (`hp.pr`)

Settings for GitHub/GitLab PR integration.

```yaml
hp:
  pr:
    github:
      org: yourorg
      default_reviewers: [alice, bob]
      labels: [agent-created, feature]
    auto_create: false
    shepherd:
      auto_apply_safe: false
      confidence_threshold: high
      auto_post_responses: false
      analysis_model: claude-3-5-sonnet
```

**Options**:
- `github.org`: GitHub organization
- `github.default_reviewers`: Default PR reviewers
- `github.labels`: Default PR labels
- `auto_create`: Auto-create PR when session ready
- `shepherd.auto_apply_safe`: Auto-apply high-confidence fixes
- `shepherd.confidence_threshold`: Minimum confidence (high, medium, low)
- `shepherd.auto_post_responses`: Auto-post AI-generated responses
- `shepherd.analysis_model`: AI model for shepherd analysis

### Templates (`hp.templates`)

Settings for context templates.

```yaml
hp:
  templates:
    local:
      feature: .hp/templates/local/feature.md
      bugfix: .hp/templates/local/bugfix.md
    marketplace:
      enabled: true
      registry_url: https://hp-templates.dev
      auto_update: true
      cache_dir: .hp/cache/templates
```

**Options**:
- `local.<type>`: Path to local template for each agent type
- `marketplace.enabled`: Enable template marketplace
- `marketplace.registry_url`: URL of template registry
- `marketplace.auto_update`: Auto-update installed templates
- `marketplace.cache_dir`: Where to cache downloaded templates

### Profiles (`hp.profiles`)

Named configuration profiles for different environments.

```yaml
hp:
  profiles:
    dev:
      ai_tool:
        command: claude-code
    prod:
      orchestration:
        cascade_on_commit: false
  active_profile: dev
```

**Usage**:
```bash
# Switch profile
hp profile switch prod

# Use specific profile for command
hp new session --profile=staging
```

## Environment Variables

Override config via environment variables:

```bash
# hannahanna command
export HP_HN_COMMAND=/usr/local/bin/hn

# AI tool
export HP_AI_TOOL=cursor

# Active profile
export HP_PROFILE=staging

# Metadata directory
export HP_METADATA_DIR=~/.local/share/hp/sessions
```

Priority: Environment variables > Local config > Repo config > User config > System config

## Example Configurations

### Minimal Config (Repo)

```yaml
# .hapusiyas.yml
hp:
  default_agent: feature
  pr:
    github:
      org: myorg
```

Everything else uses defaults.

### Developer Config (User)

```yaml
# ~/.config/hp/config.yml
hp:
  ai_tool:
    command: claude-code
    extra_args:
      - --no-telemetry

  sessions:
    metadata_dir: ~/.config/hp/sessions
    auto_snapshot: true

  profiles:
    dev:
      hn:
        default_options:
          docker: auto-start
```

### Team Config (Repo)

```yaml
# .hapusiyas.yml (committed)
hp:
  default_agent: feature

  pr:
    github:
      org: myteam
      default_reviewers:
        - alice
        - bob
      labels:
        - agent-created
        - needs-review

  templates:
    local:
      feature: .hp/templates/local/feature.md
      bugfix: .hp/templates/local/bugfix.md

  orchestration:
    cascade_on_commit: false
    gather_strategy: manual

  sessions:
    activity_log:
      enabled: true
      retention_days: 90
```

### Personal Overrides (Local)

```yaml
# .hapusiyas.local.yml (gitignored)
hp:
  ai_tool:
    command: cursor  # Override team's claude-code

  pr:
    github:
      default_reviewers: []  # I'll add reviewers manually

  active_profile: dev
```

## Configuration Management Commands

### View Config

```bash
# Show merged config
hp config show

# Show specific section
hp config show hp.ai_tool

# Show raw config (specific file)
hp config show --file=.hapusiyas.yml
```

### Edit Config

```bash
# Edit repo config
hp config edit

# Edit user config
hp config edit --user

# Edit local config
hp config edit --local
```

### Validate Config

```bash
# Validate all config files
hp config validate

# Validate specific file
hp config validate --file=.hapusiyas.yml
```

### Export Config

```bash
# Export merged config
hp config export > my-config.yml

# Export as JSON
hp config export --format=json > my-config.json
```

## Config File Location

hupasiya searches for config files in this order:

1. `HP_CONFIG` environment variable (if set)
2. `.hapusiyas.local.yml` (current directory)
3. `.hapusiyas.yml` (current directory)
4. `~/.config/hp/config.yml`
5. `/etc/hapusiyas/config.yml`

All found files are merged using the hierarchy rules.

## Configuration Best Practices

### Repo Config (.hapusiyas.yml)

**Commit these**:
- Team-wide settings (PR settings, reviewers, labels)
- Local template paths
- Orchestration preferences
- Default agent type

**Example**:
```yaml
hp:
  default_agent: feature
  pr:
    github:
      org: myteam
      labels: [agent-created]
  templates:
    local:
      feature: .hp/templates/local/feature.md
```

### User Config (~/.config/hp/config.yml)

**Personal preferences**:
- AI tool choice
- Session storage location
- Auto-snapshot settings
- Profile definitions

**Example**:
```yaml
hp:
  ai_tool:
    command: claude-code
  sessions:
    metadata_dir: ~/.config/hp/sessions
    auto_snapshot: true
```

### Local Config (.hapusiyas.local.yml)

**Personal overrides** (gitignored):
- Experimental settings
- Temporary overrides
- Local-only configurations

**Example**:
```yaml
hp:
  ai_tool:
    command: cursor  # Trying cursor this week
  active_profile: dev
```

### System Config (/etc/hapusiyas/config.yml)

**Organization-wide defaults**:
- Standard AI tool
- Common template registry
- Compliance settings

**Example**:
```yaml
hp:
  templates:
    marketplace:
      registry_url: https://templates.mycompany.com
  sessions:
    activity_log:
      enabled: true
      retention_days: 365  # Compliance requirement
```

## Troubleshooting

### Config Not Loading

```bash
# Check which configs are loaded
hp config show --debug

# Shows:
# Loading: /etc/hapusiyas/config.yml ✓
# Loading: ~/.config/hp/config.yml ✓
# Loading: .hapusiyas.yml ✗ (not found)
# Loading: .hapusiyas.local.yml ✗ (not found)
```

### Config Validation Errors

```bash
# Validate config
hp config validate

# Shows:
# Error in .hapusiyas.yml:
#   hp.default_agent: invalid value "featur" (did you mean "feature"?)
#   hp.pr.shepherd.confidence_threshold: must be one of: high, medium, low
```

### Config Merge Issues

```bash
# Show how config is merged
hp config show --show-source

# Shows:
# hp:
#   default_agent: feature  # from: .hapusiyas.yml
#   ai_tool:
#     command: cursor       # from: .hapusiyas.local.yml (overrides user config)
```

## Schema Validation

hupasiya validates configuration against a JSON schema. Invalid configs are rejected with clear error messages.

See `schema/config.schema.json` in the repository for the full schema definition.
