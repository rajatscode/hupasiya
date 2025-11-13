# Commands Reference

Complete CLI command reference for hupasiya.

## Core Operations

### `hp new`

Create a new agent session with workbox.

#### Synopsis

```bash
hp new <session-name> [OPTIONS]
```

#### Examples

```bash
# Basic feature session
hp new auth-feature

# Specify agent type
hp new fix-oauth --type=bugfix

# Create from specific branch
hp new auth-tests --from=feature/auth --type=test

# Create as child session
hp new auth-docs --parent=auth-feature --type=docs

# Use custom template
hp new spike-perf --type=research --template=custom/performance-spike

# Use marketplace template
hp new api-design --template=marketplace/fastapi-api

# Specify VCS explicitly (passed to hn)
hp new auth-feature --vcs=jujutsu

# Pass hannahanna-specific options
hp new auth-feature --hn-option=sparse=services/api/
```

#### Options

- `--type=<agent-type>` - Agent type (feature, bugfix, test, docs, shepherd, review, research, refactor)
- `--from=<branch>` - Base branch (default: current branch)
- `--parent=<session>` - Parent session (creates dependency)
- `--template=<name>` - Context template (local or marketplace/author/name)
- `--no-workbox` - Create session without new workbox (attach to existing)
- `--pr=<number>` - Associate with existing PR
- `--vcs=<git|hg|jj>` - Explicit VCS type (passed to hn add --vcs)
- `--profile=<name>` - Configuration profile to use
- `--hn-option=<key>=<value>` - Pass arbitrary options to hn add (repeatable)

#### What it does

1. Validates session name doesn't exist
2. Calls `hn add <name>` with appropriate options
3. Parses hn output to get workbox path
4. Creates session metadata in `.hp/sessions/`
5. Creates context directory in `.hp/contexts/<repo>/<session>/`
6. Applies context template based on agent type
7. Tracks parent/child relationship if `--parent` specified
8. Initializes activity log and metrics
9. Opens context in editor

---

### `hp list`

List all sessions.

#### Synopsis

```bash
hp list [OPTIONS]
```

#### Examples

```bash
# List all active sessions
hp list

# Show all statuses
hp list --all

# Show as tree (with parent/child relationships)
hp list --tree

# Filter by type
hp list --type=feature

# Filter by status
hp list --status=active

# Filter by VCS type
hp list --vcs=git

# Show PR info
hp list --pr-status

# Show metrics
hp list --metrics

# Show activity
hp list --activity

# Output as JSON
hp list --format=json
```

#### Options

- `--all` - Show all sessions (including paused, archived)
- `--tree` - Show as tree with parent/child relationships
- `--type=<agent-type>` - Filter by agent type
- `--status=<status>` - Filter by status (active, paused, integrated, archived, abandoned)
- `--vcs=<vcs-type>` - Filter by VCS type (git, hg, jj)
- `--pr-status` - Show PR information
- `--metrics` - Show metrics
- `--activity` - Show recent activity
- `--format=<format>` - Output format (table, tree, json)

---

### `hp switch`

Switch to a session (cd into workbox).

#### Synopsis

```bash
hp switch <session-name>
```

#### Examples

```bash
hp switch auth-feature
```

#### What it does

1. Looks up session in `.hp/sessions/`
2. Gets workbox path from session metadata
3. Calls `hn info <workbox-name>` to verify workbox exists
4. Outputs shell commands for wrapper to execute
5. Sets environment variables:
   - `HP_SESSION` - Session name
   - `HP_CONTEXT` - Path to context file
   - `HP_WORKBOX` - Path to workbox
   - `HP_VCS` - VCS type

#### Shell Wrapper Required

Add to `~/.bashrc` or `~/.zshrc`:

```bash
hp() {
    if [[ "$1" == "switch" ]]; then
        local session_info=$(command hp switch "$2" --output=shell)
        eval "$session_info"
    else
        command hp "$@"
    fi
}
```

---

### `hp leave`

Leave current session (return to main repo).

#### Synopsis

```bash
hp leave
```

#### What it does

1. Unsets environment variables (`HP_SESSION`, `HP_CONTEXT`, `HP_WORKBOX`, `HP_VCS`)
2. Returns to original repository directory

---

### `hp close`

Close a session.

#### Synopsis

```bash
hp close <session-name> [OPTIONS]
```

#### Examples

```bash
# Close session, keep workbox
hp close auth-feature

# Close and remove workbox (calls hn remove)
hp close auth-feature --remove-workbox

# Close and delete branch
hp close auth-feature --delete-branch

# Archive context before closing
hp close auth-feature --archive

# Full cleanup
hp close auth-feature --remove-workbox --delete-branch --archive
```

#### Options

- `--remove-workbox` - Remove hannahanna workbox (calls `hn remove`)
- `--delete-branch` - Delete VCS branch
- `--archive` - Archive context before closing
- `--force` - Force close even with uncommitted changes

#### What it does

1. Reads session metadata
2. If `--remove-workbox`, calls `hn remove <workbox-name> --force`
3. If `--delete-branch`, deletes branch via VCS command
4. Archives context if `--archive`
5. Removes session metadata
6. Logs activity event

---

### `hp info`

Show session details.

#### Synopsis

```bash
hp info <session-name>
```

#### Examples

```bash
hp info auth-feature
```

#### What it does

1. Reads session metadata
2. Calls `hn info <workbox-name> --format=json`
3. Merges and displays combined information

---

## Context Management

### `hp context view`

View session context.

#### Synopsis

```bash
hp context view [session-name] [OPTIONS]
```

#### Examples

```bash
# View current session context
hp context view

# View specific session
hp context view auth-feature

# View in editor
hp context view --edit

# View snapshot
hp context view --snapshot=2025-01-12_initial

# View global context
hp context view --global
```

#### Options

- `--edit` - Open in editor
- `--snapshot=<name>` - View specific snapshot
- `--global` - View global (repo-wide) context

---

### `hp context edit`

Edit session context.

#### Synopsis

```bash
hp context edit [session-name] [OPTIONS]
```

#### Examples

```bash
# Edit current session context
hp context edit

# Edit specific session
hp context edit auth-feature

# Edit shepherd file
hp context edit --shepherd

# Edit global context
hp context edit --global
```

#### Options

- `--shepherd` - Edit shepherd.md file (PR comments)
- `--global` - Edit global context

---

### `hp context sync`

Sync context from one session to another.

#### Synopsis

```bash
hp context sync <from> <to> [OPTIONS]
```

#### Examples

```bash
# Copy objectives from parent to child
hp context sync auth-feature auth-tests --section=objectives

# Merge all context
hp context sync auth-feature auth-docs --merge
```

#### Options

- `--section=<name>` - Sync specific section only
- `--merge` - Merge contexts instead of replacing

---

### `hp context snapshot`

Create or manage context snapshots.

#### Synopsis

```bash
hp context snapshot [session-name] [snapshot-name] [OPTIONS]
```

#### Examples

```bash
# Snapshot current state
hp context snapshot

# Named snapshot
hp context snapshot auth-feature after-review

# List snapshots
hp context snapshot --list

# Restore snapshot
hp context snapshot --restore=2025-01-12_initial

# Diff snapshots
hp context snapshot --diff=2025-01-12_initial,2025-01-15_after-review
```

#### Options

- `--list` - List all snapshots
- `--restore=<name>` - Restore snapshot
- `--diff=<name1>,<name2>` - Show diff between snapshots

---

## Multi-Agent Orchestration

### `hp tree`

Show session tree.

#### Synopsis

```bash
hp tree [session-name]
```

#### Examples

```bash
# Show all sessions as tree
hp tree

# Show specific session and its family
hp tree auth-feature
```

---

### `hp cascade`

Sync parent changes to children.

#### Synopsis

```bash
hp cascade <session-name> [OPTIONS]
```

#### Examples

```bash
# Cascade to all children
hp cascade auth-feature

# Cascade to specific child
hp cascade auth-feature --to=auth-tests

# Cascade specific files
hp cascade auth-feature --files=src/auth/*

# Dry run
hp cascade auth-feature --dry-run
```

#### Options

- `--to=<child-session>` - Cascade to specific child only
- `--files=<pattern>` - Cascade specific files only
- `--dry-run` - Show what would be cascaded without doing it

#### What it does

1. Commits current changes in parent (if any)
2. For each child:
   - Stashes uncommitted changes
   - Merges parent branch into child branch (VCS-appropriate command)
   - Resolves conflicts (auto or prompt)
   - Unstashes changes
3. Updates child contexts with parent objectives
4. Logs activity event

---

### `hp gather`

Collect work from children back to parent.

#### Synopsis

```bash
hp gather <session-name> [OPTIONS]
```

#### Examples

```bash
# Gather all children
hp gather auth-feature

# Gather specific children
hp gather auth-feature --from=auth-tests,auth-docs

# Strategy
hp gather auth-feature --strategy=merge  # or rebase, squash

# Preview
hp gather auth-feature --dry-run
```

#### Options

- `--from=<child1>,<child2>` - Gather specific children only
- `--strategy=<merge|rebase|squash>` - Merge strategy
- `--dry-run` - Show what would be gathered without doing it

#### What it does

1. Validates all children have clean state
2. For each child:
   - Merges child branch into parent branch (VCS-appropriate)
   - Resolves conflicts
   - Updates parent context
3. Optionally closes child sessions
4. Updates parent session status
5. Logs activity event

---

## AI Tool Integration

### `hp launch`

Launch AI tool with context.

#### Synopsis

```bash
hp launch [session-name] [OPTIONS]
```

#### Examples

```bash
# Launch for current session
hp launch

# Launch for specific session
hp launch auth-feature

# Override AI tool
hp launch --tool=cursor

# Pass extra args
hp launch -- --no-telemetry

# Launch with specific profile
hp launch --profile=staging
```

#### Options

- `--tool=<command>` - Override AI tool command
- `--profile=<name>` - Use specific profile
- `-- <args>` - Pass extra arguments to AI tool

#### What it does

1. Sets up environment variables
2. Loads context file
3. Launches AI tool based on config
4. For tools with slash commands (Claude Code), creates global command

---

### `hp shell`

Launch shell in session workbox with env vars.

#### Synopsis

```bash
hp shell [session-name] [OPTIONS]
```

#### Examples

```bash
# Launch shell in current session
hp shell

# Launch in specific session
hp shell auth-feature

# Run command
hp shell auth-feature -- git status
```

#### Options

- `-- <command>` - Run command instead of interactive shell

---

### `hp exec`

Execute command in session workbox.

#### Synopsis

```bash
hp exec <session-name> <command> [OPTIONS]
```

#### Examples

```bash
# Run tests in session
hp exec auth-tests npm test

# Run in all children
hp exec auth-feature --cascade -- npm test

# Run in tree (parent + all children)
hp exec auth-feature --tree -- git pull
```

#### Options

- `--cascade` - Run in all children
- `--tree` - Run in parent and all children

---

## PR Workflow

### `hp pr create`

Create PR from session.

#### Synopsis

```bash
hp pr create [session-name] [OPTIONS]
```

#### Examples

```bash
# Create PR for current session
hp pr create

# Create PR for specific session
hp pr create auth-feature

# With options
hp pr create --draft
hp pr create --reviewers=alice,bob
hp pr create --labels=feature,authentication

# Auto-fill from context
hp pr create --from-context
```

#### Options

- `--draft` - Create as draft PR
- `--reviewers=<user1>,<user2>` - Request reviewers
- `--labels=<label1>,<label2>` - Add labels
- `--from-context` - Use context for PR description

#### What it does

1. Pushes branch to remote
2. Creates PR using GitHub API
3. Uses session context for PR description
4. Links PR number to session
5. Adds configured labels and reviewers

---

### `hp pr sync`

Sync PR feedback to context.

#### Synopsis

```bash
hp pr sync [session-name] [OPTIONS]
```

#### Examples

```bash
# Sync current session
hp pr sync

# Sync specific session
hp pr sync auth-feature

# Auto-create shepherd tasks
hp pr sync --create-shepherd-tasks
```

#### Options

- `--create-shepherd-tasks` - Create shepherd tasks for unresolved comments

#### What it does

1. Fetches PR comments
2. Filters unresolved comments
3. Updates session context with comments
4. Optionally creates shepherd tasks
5. Logs activity event

---

### `hp pr status`

Show PR status.

#### Synopsis

```bash
hp pr status [session-name]
```

#### Examples

```bash
hp pr status
hp pr status auth-feature
```

---

## Shepherd (PR Comment Resolution)

### `hp shepherd`

Interactive PR comment resolution.

#### Synopsis

```bash
hp shepherd [OPTIONS]
```

#### Examples

```bash
# Auto-detect PR from current session
hp shepherd

# Specific PR
hp shepherd --pr=123

# Specific session
hp shepherd --session=auth-feature

# Auto-apply safe fixes
hp shepherd --auto-apply

# Dry run (show analysis only)
hp shepherd --dry-run

# Address specific comment
hp shepherd --comment=789012345
```

#### Options

- `--pr=<number>` - Specific PR number
- `--session=<name>` - Specific session
- `--auto-apply` - Auto-apply high-confidence fixes
- `--dry-run` - Show analysis without applying
- `--comment=<id>` - Address specific comment only

#### What it does

1. Fetches unresolved PR review comments
2. Creates shepherd context in `.hp/contexts/<session>/shepherd.md`
3. Launches AI tool with shepherd context
4. AI analyzes each comment and suggests action (FIX, CLARIFY, ACKNOWLEDGE, DEFER, DISAGREE)
5. Applies fixes if `--auto-apply` and high confidence
6. Posts responses to PR comments
7. Marks comments as resolved (if fix applied)
8. Logs activity event

---

### `hp shepherd analyze`

Analyze specific comment without applying.

#### Synopsis

```bash
hp shepherd analyze <comment-id>
```

---

### `hp shepherd apply`

Apply previously analyzed fix.

#### Synopsis

```bash
hp shepherd apply <comment-id>
```

---

## Session Collaboration

### `hp handoff`

Hand off session to another developer.

#### Synopsis

```bash
hp handoff <session-name> --to=<user> [OPTIONS]
```

#### Examples

```bash
hp handoff auth-feature --to=alice

# With message
hp handoff auth-feature --to=alice --message="Please finish OAuth integration"
```

#### Options

- `--to=<user>` - User to hand off to (required)
- `--message=<msg>` - Handoff message

#### What it does

1. Creates context snapshot
2. Pushes branch to remote
3. Locks session
4. Sends notification to user (if configured)
5. Exports session metadata for import

---

### `hp clone`

Clone session for parallel work.

#### Synopsis

```bash
hp clone <session-name> --as=<new-name> [OPTIONS]
```

#### Examples

```bash
# Clone for experimentation
hp clone auth-feature --as=auth-feature-experiment

# Clone with different type
hp clone auth-feature --as=auth-refactor --type=refactor
```

#### Options

- `--as=<new-name>` - New session name (required)
- `--type=<agent-type>` - Override agent type

---

### `hp merge-sessions`

Merge two sessions.

#### Synopsis

```bash
hp merge-sessions <session1> <session2> [OPTIONS]
```

#### Examples

```bash
# Merge experimental branch back
hp merge-sessions auth-feature-experiment auth-feature

# Strategy
hp merge-sessions s1 s2 --strategy=squash
```

#### Options

- `--strategy=<merge|squash>` - Merge strategy

---

## Activity & Metrics

### `hp activity`

Show activity log.

#### Synopsis

```bash
hp activity [session-name] [OPTIONS]
```

#### Examples

```bash
# Show current session activity
hp activity

# Show specific session
hp activity auth-feature

# Last N events
hp activity --limit=10

# Since time
hp activity --since=1d

# Filter by type
hp activity --type=commit

# Export to JSON
hp activity --export=activity.json
```

#### Options

- `--limit=<n>` - Show last N events
- `--since=<duration>` - Show events since (1h, 1d, 1w)
- `--type=<event-type>` - Filter by event type
- `--export=<file>` - Export to file

---

### `hp metrics`

Show session metrics.

#### Synopsis

```bash
hp metrics [session-name] [OPTIONS]
```

#### Examples

```bash
# Show current session metrics
hp metrics

# Show specific session
hp metrics auth-feature

# Compare sessions
hp metrics --compare=auth-feature,auth-tests

# Show trends
hp metrics --trend --period=7d

# Export
hp metrics --export=metrics.json
```

#### Options

- `--compare=<s1>,<s2>` - Compare multiple sessions
- `--trend` - Show trend graph
- `--period=<duration>` - Period for trends (7d, 30d)
- `--export=<file>` - Export to file

---

### `hp monitor`

Real-time monitoring dashboard.

#### Synopsis

```bash
hp monitor [OPTIONS]
```

#### Examples

```bash
# Monitor all sessions
hp monitor

# Monitor specific session
hp monitor auth-feature

# Refresh interval
hp monitor --refresh=5s

# Dashboard view
hp monitor --dashboard
```

#### Options

- `--refresh=<duration>` - Refresh interval (default: 2s)
- `--dashboard` - Full dashboard view

---

### `hp stats`

Show combined stats from hupasiya and hannahanna.

#### Synopsis

```bash
hp stats <session-name>
```

#### Examples

```bash
hp stats auth-feature
```

---

## Template Marketplace

### `hp template search`

Search template marketplace.

#### Synopsis

```bash
hp template search <query> [OPTIONS]
```

#### Examples

```bash
# Search templates
hp template search fastapi

# Filter by category
hp template search --category=api

# Filter by author
hp template search --author=yourorg
```

#### Options

- `--category=<category>` - Filter by category
- `--author=<author>` - Filter by author

---

### `hp template install`

Install template from marketplace.

#### Synopsis

```bash
hp template install <name> [OPTIONS]
```

#### Examples

```bash
# Install template
hp template install fastapi-api

# Install specific version
hp template install fastapi-api@1.2.0

# Install from URL
hp template install https://hp-templates.dev/yourorg/custom-template
```

#### Options

- `@<version>` - Specific version
- `--url=<url>` - Install from URL

---

### `hp template list`

List installed templates.

#### Synopsis

```bash
hp template list [OPTIONS]
```

#### Examples

```bash
# List all templates
hp template list

# Show details
hp template list --verbose
```

#### Options

- `--verbose` - Show detailed information

---

### `hp template publish`

Publish template to marketplace.

#### Synopsis

```bash
hp template publish <path> [OPTIONS]
```

#### Examples

```bash
# Publish template
hp template publish .hp/templates/local/my-template

# With metadata
hp template publish .hp/templates/local/my-template --author=yourname --category=api
```

#### Options

- `--author=<author>` - Author name
- `--category=<category>` - Category

---

### `hp template update`

Update templates.

#### Synopsis

```bash
hp template update [name]
```

#### Examples

```bash
# Update all templates
hp template update

# Update specific template
hp template update fastapi-api
```

---

## Configuration Profiles

### `hp profile list`

List available profiles.

#### Synopsis

```bash
hp profile list
```

---

### `hp profile switch`

Switch active profile.

#### Synopsis

```bash
hp profile switch <name>
```

#### Examples

```bash
hp profile switch staging
```

---

### `hp profile show`

Show profile configuration.

#### Synopsis

```bash
hp profile show [name]
```

#### Examples

```bash
# Show active profile
hp profile show

# Show specific profile
hp profile show prod
```

---

## Utility Commands

### `hp version`

Show version information.

#### Synopsis

```bash
hp version
```

---

### `hp doctor`

Check installation and configuration.

#### Synopsis

```bash
hp doctor
```

Checks:
- hannahanna installation
- Configuration validity
- Directory permissions
- Git/Hg/Jj availability

---

### `hp clean`

Clean up stale sessions and cache.

#### Synopsis

```bash
hp clean [OPTIONS]
```

#### Examples

```bash
# Clean stale sessions (missing workboxes)
hp clean

# Clean template cache
hp clean --cache

# Clean old activity logs
hp clean --logs --older-than=90d
```

#### Options

- `--cache` - Clean template cache
- `--logs` - Clean old activity logs
- `--older-than=<duration>` - Only clean items older than duration
- `--dry-run` - Show what would be cleaned

---

## Global Options

All commands support:

- `-h, --help` - Show help
- `-v, --verbose` - Verbose output
- `--quiet` - Suppress output
- `--config=<file>` - Use specific config file
- `--no-color` - Disable colored output
