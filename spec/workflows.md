# Workflows

Common workflows and use cases for hupasiya.

## Workflow 1: Feature Development with Tests

Building a feature with separate test session as a child.

```bash
# 1. Create feature session
hp new auth-feature --type=feature

# Editor opens for you to fill in objectives:
# - Implement OAuth 2.0 authentication
# - Support Google and GitHub providers
# - Add token refresh logic

# 2. Launch AI agent
hp launch

# AI works on feature implementation...
# You commit changes as you go

# 3. Create child session for tests
hp new auth-tests --parent=auth-feature --type=test

# 4. Cascade feature code to test session
hp cascade auth-feature

# This merges auth-feature branch into auth-tests branch

# 5. Switch to test session
hp switch auth-tests

# 6. Launch AI in test session
hp launch

# AI writes tests based on implementation...

# 7. After tests written, gather back to feature
hp gather auth-feature

# This merges auth-tests back into auth-feature

# 8. Create PR
hp pr create auth-feature

# 9. Address PR comments with shepherd
hp shepherd
```

---

## Workflow 2: Bug Fix with Root Cause Analysis

Fixing a bug with a research spike first.

```bash
# 1. Create research session to understand the bug
hp new oauth-redirect-investigation --type=research

# Fill in context:
# - OAuth redirect failing in production
# - Need to understand root cause

# 2. Launch AI for investigation
hp launch

# AI investigates codebase, logs, etc.
# You document findings in context

# 3. Create bugfix session based on findings
hp new fix-oauth-redirect --type=bugfix --parent=oauth-redirect-investigation

# Context is inherited from parent

# 4. Launch AI to implement fix
hp launch

# 5. Create PR
hp pr create fix-oauth-redirect

# 6. Close research session (keep for reference)
hp close oauth-redirect-investigation --archive
```

---

## Workflow 3: PR Comment Resolution

Addressing review comments on an existing PR.

```bash
# 1. You have an open PR with review comments
# Sync comments to session
hp pr sync auth-feature

# This fetches unresolved comments and updates context

# 2. Run shepherd to analyze comments
hp shepherd

# Shepherd does:
# - Fetches comments
# - Creates shepherd.md with all comments
# - Launches AI with context
# - AI analyzes each comment

# 3. AI suggests actions:
# Comment 1: FIX (HIGH confidence) - Add error handling
# Comment 2: CLARIFY (MEDIUM) - Ask about edge case
# Comment 3: ACKNOWLEDGE (HIGH) - Good catch, will do

# 4. If --auto-apply was used, high-confidence fixes are applied
# Otherwise, you review and apply manually

# 5. Responses are posted to PR

# 6. Push changes
git push

# 7. Verify comments resolved
hp pr status auth-feature
```

---

## Workflow 4: Multi-Agent Parallel Development

Working on multiple aspects of a feature in parallel.

```bash
# 1. Create parent session for feature
hp new user-dashboard --type=feature

# 2. Create child sessions for different aspects
hp new dashboard-backend --parent=user-dashboard --type=feature
hp new dashboard-frontend --parent=user-dashboard --type=feature
hp new dashboard-tests --parent=user-dashboard --type=test
hp new dashboard-docs --parent=user-dashboard --type=docs

# 3. View the tree
hp tree user-dashboard

# user-dashboard (feature)
# ├── dashboard-backend (feature)
# ├── dashboard-frontend (feature)
# ├── dashboard-tests (test)
# └── dashboard-docs (docs)

# 4. Work on each session independently
hp switch dashboard-backend
hp launch
# Backend implementation...

hp switch dashboard-frontend
hp launch
# Frontend implementation...

# 5. Periodically cascade changes from parent
hp cascade user-dashboard

# This syncs any shared changes (types, interfaces) to all children

# 6. When ready, gather all work back to parent
hp gather user-dashboard

# 7. Create single PR with all work
hp pr create user-dashboard
```

---

## Workflow 5: Session Handoff

Handing off work to another developer.

```bash
# You need to go on vacation mid-feature
# 1. Ensure your work is committed
git commit -m "WIP: OAuth integration"

# 2. Create handoff snapshot
hp context snapshot auth-feature handoff-to-alice

# 3. Handoff session
hp handoff auth-feature --to=alice --message="80% done, need OAuth callback handling"

# This:
# - Creates snapshot
# - Pushes branch
# - Locks session
# - Sends notification to alice

# Alice receives notification and clones the session
# 4. Alice clones your session
hp clone auth-feature --as=auth-feature-alice

# 5. Alice continues work
hp switch auth-feature-alice
hp launch

# 6. When done, Alice creates PR
hp pr create auth-feature-alice

# 7. After merge, Alice closes both sessions
hp close auth-feature-alice --remove-workbox
hp close auth-feature --remove-workbox
```

---

## Workflow 6: Refactoring with Safe Changes

Large refactoring across multiple sessions.

```bash
# 1. Create refactor session
hp new refactor-auth-module --type=refactor

# 2. Document refactoring plan in context
hp context edit

# Plan:
# - Extract auth logic to separate module
# - Update all callsites
# - Add type safety

# 3. Launch AI
hp launch

# AI starts refactoring...

# 4. Ensure existing sessions get updates
# List all active sessions that might be affected
hp list --type=feature | grep auth

# 5. For each affected session, create update task
hp new update-login-flow --parent=refactor-auth-module --type=feature

# 6. Cascade refactoring to affected sessions
hp cascade refactor-auth-module

# 7. Each child session adapts to refactored code

# 8. Gather all adaptations back
hp gather refactor-auth-module

# 9. Create PR
hp pr create refactor-auth-module
```

---

## Workflow 7: Multi-VCS Coordination

Working across repositories with different VCS systems.

```bash
# Frontend repo (Git)
cd frontend
hp new oauth-frontend --type=feature --vcs=git

# Fill in context:
# - Implement OAuth UI
# - Call backend API
# - Store tokens

# Backend repo (Jujutsu)
cd ../backend
hp new oauth-backend --type=feature --vcs=jj

# Fill in context:
# - Implement OAuth endpoints
# - Token management
# - API for frontend

# Link contexts for reference
hp context edit oauth-backend
# Add link to frontend context:
# Frontend context: ../frontend/.hp/contexts/frontend/oauth-frontend/context.md

# Both sessions can reference each other's progress

# Work proceeds independently
cd frontend
hp switch oauth-frontend
hp launch

cd ../backend
hp switch oauth-backend
hp launch

# Create PRs independently
cd frontend
hp pr create oauth-frontend

cd ../backend
hp pr create oauth-backend
```

---

## Workflow 8: Shepherd for Large PR

Comprehensive PR comment resolution for a large feature.

```bash
# You have a large PR with 20+ comments
hp pr status feature-oauth

# PR #123: Add OAuth authentication
# Comments: 24 unresolved

# 1. Sync all comments
hp pr sync feature-oauth

# 2. Run shepherd in dry-run mode to see analysis
hp shepherd --dry-run

# Shepherd analyzes all 24 comments and categorizes:
# - 12 FIX (code changes needed)
# - 5 CLARIFY (need more info)
# - 4 ACKNOWLEDGE (will do)
# - 3 DEFER (out of scope)

# 3. Review the analysis
hp context view --shepherd

# 4. Apply shepherd with auto-apply for high confidence
hp shepherd --auto-apply

# This automatically:
# - Applies 8 high-confidence fixes
# - Posts clarification questions for 5 comments
# - Acknowledges 4 comments
# - Requests deferral for 3 comments

# 5. Review 4 remaining medium-confidence fixes manually
hp context view --shepherd

# 6. Apply remaining fixes one by one
hp shepherd apply <comment-id>

# 7. Push all changes
git push

# 8. Verify all comments addressed
hp pr status feature-oauth

# PR #123: Add OAuth authentication
# Comments: 0 unresolved, 24 resolved
```

---

## Workflow 9: Experiment and Merge Back

Trying an experimental approach in a cloned session.

```bash
# You're working on auth-feature but want to try a different approach
# 1. Clone current session
hp clone auth-feature --as=auth-feature-experiment

# 2. Switch to experimental session
hp switch auth-feature-experiment

# 3. Try different approach
hp launch

# AI tries experimental approach...

# 4. If experiment succeeds, merge back
hp merge-sessions auth-feature-experiment auth-feature --strategy=squash

# 5. Close experimental session
hp close auth-feature-experiment --remove-workbox

# 6. If experiment fails, just abandon it
hp close auth-feature-experiment --remove-workbox --delete-branch
```

---

## Workflow 10: Documentation Sprint

Creating comprehensive documentation for a feature.

```bash
# Feature is done, now need docs
# 1. Create docs session as child of feature
hp new auth-docs --parent=auth-feature --type=docs

# 2. Cascade feature code to docs session
hp cascade auth-feature

# This gives the docs session access to all the code

# 3. Switch to docs session
hp switch auth-docs

# 4. Launch AI
hp launch

# Context includes:
# - Feature objectives (inherited from parent)
# - All implemented code
# - Task: Write comprehensive documentation

# AI writes:
# - API documentation
# - User guides
# - Examples
# - Migration guide

# 5. Review and iterate
hp launch

# 6. Gather docs back to feature
hp gather auth-feature

# 7. Update PR with docs
git push

# 8. Close docs session
hp close auth-docs --remove-workbox
```

---

## Workflow 11: Test-Driven Development

Writing tests first, then implementation.

```bash
# 1. Create test session first
hp new auth-tests --type=test

# Fill in context:
# - Write tests for OAuth authentication
# - Test login flow
# - Test token refresh
# - Test error handling

# 2. Launch AI to write tests
hp launch

# AI writes comprehensive tests (that fail)

# 3. Create feature session as sibling (not child)
hp new auth-feature --type=feature --from=main

# 4. Link to test session in context
hp context edit auth-feature
# Add: Tests in ../auth-tests

# 5. Launch AI to implement feature
hp launch

# Context includes:
# - Feature objectives
# - Link to tests
# - Task: Make tests pass

# 6. Periodically sync test updates
# If tests change:
cd auth-tests
git push

cd ../auth-feature
git merge auth-tests-branch

# 7. When all tests pass, merge both
hp close auth-tests --remove-workbox
hp pr create auth-feature
```

---

## Workflow 12: Monitoring and Metrics

Tracking progress across multiple sessions.

```bash
# 1. Launch monitoring dashboard
hp monitor

# Dashboard shows:
# - All active sessions
# - Time spent
# - Commits
# - Recent activity

# 2. Check metrics for specific session
hp metrics auth-feature

# Session Metrics: auth-feature
# Time: 12h 35m
# Commits: 24
# Changes: +2,345/-567 lines
# AI: 89 interactions, 234k tokens

# 3. Compare sessions
hp metrics --compare=auth-feature,auth-tests,auth-docs

# 4. View trends
hp metrics auth-feature --trend --period=7d

# 5. Export for reporting
hp metrics auth-feature --export=report.json

# 6. Check activity log
hp activity auth-feature

# Recent Activity:
# 2025-01-15 14:30  Commit: "Add token refresh"
# 2025-01-15 13:45  AI launched
# 2025-01-15 12:15  Cascaded to auth-tests
# 2025-01-15 11:30  PR comment received
```

---

## Workflow 13: Clean Up and Archive

Cleaning up completed sessions.

```bash
# 1. List all integrated sessions
hp list --status=integrated

# 2. Archive old sessions
hp close auth-feature --archive --remove-workbox

# This:
# - Archives context to .hp/archives/
# - Removes workbox
# - Keeps session metadata for history

# 3. Clean up stale sessions (missing workboxes)
hp clean --dry-run

# Shows:
# - old-feature-1 (workbox missing)
# - experiment-2 (workbox missing)

hp clean

# Removes session metadata for missing workboxes

# 4. Clean template cache
hp clean --cache

# 5. Clean old activity logs (>90 days)
hp clean --logs --older-than=90d
```

---

## Workflow 14: Emergency Hotfix

Quick hotfix with minimal ceremony.

```bash
# Production is down, need quick fix
# 1. Create bugfix session from main
hp new hotfix-auth-crash --type=bugfix --from=main

# 2. Quick context fill
hp context edit
# Context: Auth service crashing on null token

# 3. Launch AI
hp launch

# AI quickly identifies and fixes issue

# 4. Create PR immediately
hp pr create hotfix-auth-crash --labels=hotfix,urgent

# 5. Get quick review and merge
# ...

# 6. Clean up
hp close hotfix-auth-crash --remove-workbox --delete-branch
```

---

## Workflow 15: Configuration Profiles

Using different profiles for different environments.

```bash
# 1. Check available profiles
hp profile list

# Available Profiles:
# * dev (active)
#   - Docker auto-start: enabled
#   - AI tool: claude-code
#
#   staging
#   - Docker auto-start: disabled
#   - PR auto-create: enabled
#
#   prod
#   - Orchestration: manual only

# 2. Create session with dev profile (default)
hp new feature-x

# Docker starts automatically, uses claude-code

# 3. Switch to staging profile
hp profile switch staging

# 4. Create session with staging profile
hp new feature-y

# Docker doesn't auto-start, PR will be created automatically

# 5. Check current profile
hp profile show

# Active Profile: staging
# - Docker auto-start: disabled
# - PR auto-create: enabled
# - AI tool: claude-code
```

---

## Best Practices

### Context Management

- **Keep context focused**: One objective per session
- **Inherit from parent**: Use parent/child for related work
- **Snapshot frequently**: Before major changes (cascade, gather, PR review)
- **Link shared context**: Use `.hp/contexts/<repo>/shared/` for common patterns

### Multi-Agent Coordination

- **Use tree structure**: Parent for feature, children for aspects (backend, frontend, tests, docs)
- **Cascade regularly**: Keep children in sync with parent changes
- **Gather atomically**: Gather all children at once when feature is ready
- **Avoid deep trees**: Keep hierarchy shallow (2-3 levels max)

### PR Workflow

- **Sync before shepherd**: Always `hp pr sync` before `hp shepherd`
- **Review dry-run first**: Use `--dry-run` to see shepherd analysis before applying
- **Auto-apply with caution**: Only use `--auto-apply` for well-tested contexts
- **Track resolution**: Use `hp pr status` to track progress

### Session Lifecycle

- **Close when done**: Don't leave sessions active indefinitely
- **Archive important work**: Use `--archive` for sessions you want to reference later
- **Clean regularly**: Run `hp clean` to remove stale sessions
- **Export metrics**: Use `hp metrics --export` for reporting
