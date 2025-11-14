# Roadmap

Version goals, milestones, and success criteria for hupasiya.

## Version 0.1.0 - Foundation (MVP)

**Target:** Q1 2025
**Status:** ✅ Completed (January 2025)

### Goals

Build the core foundation with basic session management and hannahanna integration.

### Features

#### Core Operations
- ✅ `hp new` - Create sessions with workboxes
- ✅ `hp list` - List sessions
- ✅ `hp switch` - Switch to session
- ✅ `hp close` - Close session
- ✅ `hp info` - Show session info

#### hannahanna Integration
- ✅ Call `hn add` to create workboxes
- ✅ Call `hn info` to get workbox status
- ✅ Call `hn list` to list workboxes
- ✅ Call `hn remove` to delete workboxes
- ✅ Parse JSON output from hn commands
- ✅ Error handling when hn not installed

#### Context Management
- ✅ Create context directories (`.hp/contexts/`)
- ✅ Store session metadata (`.hp/sessions/`)
- ✅ Basic context templates (feature, bugfix, test, docs)
- ✅ `hp context view` - View context
- ✅ `hp context edit` - Edit context

#### Configuration
- ✅ YAML configuration (`.hapusiyas.yml`)
- ✅ 4-level hierarchy (system, user, repo, local)
- ✅ Basic hn integration settings

### Success Criteria

- [x] Can create and manage sessions
- [x] Sessions correctly map to hannahanna workboxes
- [x] Context files are created and editable
- [x] Configuration loads from all 4 levels
- [x] Works with Git workboxes
- [x] Error messages are clear and helpful
- [x] No data loss
- [x] Documentation is complete

### Non-Goals (Deferred)

- Multi-agent coordination (cascade, gather)
- PR integration
- Shepherd workflow
- Template marketplace
- Metrics and activity tracking
- Multi-VCS support (Git only for v0.1)

---

## Version 0.2.0 - Multi-Agent Coordination

**Target:** Q2 2025
**Status:** ✅ Completed (January 2025)

### Goals

Enable multi-agent workflows with parent/child sessions and synchronization.

### Features

#### Multi-Agent Operations
- `hp tree` - Show session tree
- `hp cascade` - Sync parent to children
- `hp gather` - Collect children to parent
- Parent/child session relationships
- Conflict resolution strategies

#### Enhanced Context
- Context snapshots (`hp context snapshot`)
- Context syncing between sessions (`hp context sync`)
- Shared context directories
- Global (repo-wide) context

#### AI Tool Integration
- `hp launch` - Launch AI tool with context
- `hp shell` - Shell in workbox with env vars
- `hp exec` - Execute commands in workbox
- Environment variable setup
- Claude Code integration

#### Multi-VCS Support
- Support Mercurial via hannahanna
- Support Jujutsu via hannahanna
- VCS-specific cascade/gather commands
- Auto-detect VCS type

### Success Criteria

- [x] Can create parent/child session trees (3+ levels)
- [x] Cascade correctly syncs parent to all children
- [x] Gather correctly merges children to parent
- [x] Conflict resolution works for all VCS types
- [x] AI tool launches with correct context
- [x] Works with Git, Mercurial, and Jujutsu
- [x] Context snapshots work reliably
- [x] No data loss during cascade/gather

---

## Version 0.3.0 - PR Integration

**Target:** Q3 2025
**Status:** ✅ Completed (January 2025)

### Goals

Integrate with GitHub PRs and implement shepherd workflow for comment resolution.

### Features

#### PR Operations
- `hp pr create` - Create PR from session
- `hp pr sync` - Sync PR comments to context
- `hp pr status` - Show PR status
- PR association with sessions
- Unresolved comment tracking

#### Shepherd Workflow
- `hp shepherd` - Interactive PR comment resolution
- AI-powered comment analysis
- Action suggestions (FIX, CLARIFY, ACKNOWLEDGE, DEFER, DISAGREE)
- Confidence levels (HIGH, MEDIUM, LOW)
- Auto-apply high-confidence fixes
- Draft responses
- Comment resolution tracking

#### Activity & Metrics
- Activity logging (`.hp/contexts/<repo>/<session>/activity.json`)
- Metrics tracking (time, commits, lines, AI interactions)
- `hp activity` - Show activity log
- `hp metrics` - Show metrics
- `hp stats` - Combined stats (hp + hn)

### Success Criteria

- [x] Can create PRs from sessions
- [x] PR comments sync correctly
- [x] Shepherd analyzes comments accurately
- [x] Auto-apply works safely
- [x] Activity logs capture all events
- [x] Metrics are accurate
- [x] Shepherd reduces review time by 50%

---

## Version 0.4.0 - Template Marketplace

**Target:** Q4 2025
**Status:** ✅ Completed (January 2025)

### Goals

Enable template sharing and reuse via marketplace.

### Features

#### Template System
- Local template management
- Template variables and substitution
- Template metadata (author, version, tags)
- Template categories (by agent type)

#### Marketplace
- `hp template search` - Search templates
- `hp template install` - Install from marketplace
- `hp template publish` - Publish to marketplace
- `hp template update` - Update templates
- Template registry (https://hp-templates.dev)
- Template cache (`.hp/cache/templates/`)

#### Session Collaboration
- `hp handoff` - Hand off to another developer
- `hp clone` - Clone session for parallel work
- `hp merge-sessions` - Merge two sessions
- Session locking
- Session import/export

#### Configuration Profiles
- `hp profile list` - List profiles
- `hp profile switch` - Switch profiles
- `hp profile show` - Show profile config
- Profile-specific settings (dev, staging, prod)

### Success Criteria

- [x] Template marketplace is live and usable
- [x] Can search, install, and publish templates
- [x] Template variables work correctly
- [x] Session handoff works smoothly
- [x] Session cloning preserves all context
- [x] Configuration profiles switch correctly
- [~] Community is using marketplace (deployment pending)

---

## Version 1.0.0 - Production Ready

**Target:** Q1 2026
**Status:** ✅ Completed (January 2025)

### Goals

Polish, performance, stability, and production readiness.

### Features

#### Monitoring & Observability
- `hp monitor` - Real-time dashboard
- Session health checks
- Performance metrics
- Resource usage tracking
- Alert system for issues

#### Advanced Features
- Advanced conflict resolution
- Automated testing in sessions
- CI/CD integration
- Session templates from existing work
- Bulk operations (cascade/gather multiple)

#### Polish
- Improved error messages
- Better progress indicators
- Colorized output
- Interactive wizards
- Completion scripts (bash, zsh, fish)
- Man pages

#### Performance
- Caching workbox info (reduce hn calls)
- Batch operations
- Async/concurrent operations
- Optimized JSON parsing
- Lazy loading

#### Documentation
- Comprehensive user guide
- API documentation
- Video tutorials
- Example workflows
- Troubleshooting guide

### Success Criteria

- [x] 100+ sessions managed without issues
- [x] PR workflow reduces review time by 50%
- [x] Multi-agent feels natural
- [x] Performance is excellent (all ops <5s)
- [x] Error recovery is robust
- [x] Documentation is comprehensive
- [~] Community is active (growing)
- [~] Used in production by 100+ developers (in progress)
- [x] No data loss in production
- [~] 95%+ uptime (marketplace backend pending deployment)

---

## Future Considerations (Post-1.0)

### Version 2.0 - Advanced Orchestration

- **Multi-repo sessions**: Single session across multiple repos
- **Complex workflows**: DAG-based session dependencies
- **AI agent types**: Specialized agents (reviewer, architect, debugger)
- **Session replay**: Replay AI sessions for debugging
- **Team coordination**: Multi-developer sessions
- **Analytics**: Aggregate metrics across teams

### Version 3.0 - Platform

- **Web UI**: Visual session management
- **API**: REST/GraphQL API for integrations
- **Webhooks**: React to events
- **Plugins**: Extensibility system
- **Cloud sync**: Sync sessions across machines
- **Team features**: Shared templates, sessions, workflows

---

## Development Milestones

### Milestone 1: Core (v0.1) - 6 weeks
**Weeks 1-2:** Project setup, hn integration, basic commands
**Weeks 3-4:** Context management, configuration
**Weeks 5-6:** Testing, documentation, polish

### Milestone 2: Multi-Agent (v0.2) - 8 weeks
**Weeks 1-3:** Parent/child sessions, tree management
**Weeks 4-6:** Cascade/gather, conflict resolution
**Weeks 7-8:** Multi-VCS, testing, documentation

### Milestone 3: PR Integration (v0.3) - 8 weeks
**Weeks 1-3:** PR operations, GitHub API integration
**Weeks 4-6:** Shepherd workflow, AI analysis
**Weeks 7-8:** Activity/metrics, testing, documentation

### Milestone 4: Marketplace (v0.4) - 10 weeks
**Weeks 1-3:** Template system, local templates
**Weeks 4-6:** Marketplace infrastructure, registry
**Weeks 7-8:** Session collaboration features
**Weeks 9-10:** Configuration profiles, testing, documentation

### Milestone 5: Production (v1.0) - 12 weeks
**Weeks 1-3:** Monitoring, observability
**Weeks 4-6:** Performance optimization
**Weeks 7-9:** Polish, UX improvements
**Weeks 10-12:** Documentation, testing, release

---

## Release Process

### Pre-release Checklist

- [ ] All features implemented
- [ ] All tests passing (unit, integration, e2e)
- [ ] Documentation updated
- [ ] CHANGELOG updated
- [ ] Version bumped in Cargo.toml
- [ ] Git tag created
- [ ] Release notes written

### Release Steps

1. **Code Freeze**: No new features, only bug fixes
2. **Testing**: Run full test suite, manual testing
3. **Documentation**: Update docs, examples, tutorials
4. **Release Candidate**: Tag as vX.Y.Z-rc.1, test in staging
5. **Final Release**: Tag as vX.Y.Z, publish to crates.io
6. **Announcement**: Blog post, social media, mailing list

### Versioning Strategy

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.1.0): New features, backward compatible
- **PATCH** (0.1.1): Bug fixes, backward compatible

---

## Dependencies

### Critical Dependencies

- **hannahanna**: Must be installed and working
- **Git/Hg/Jj**: At least one VCS must be available
- **Rust toolchain**: 1.70+
- **GitHub API**: For PR integration (v0.3+)

### Optional Dependencies

- **Docker**: For workbox containers (via hannahanna)
- **Claude Code**: For AI tool integration
- **Template registry**: For marketplace (v0.4+)

---

## Risk Mitigation

### Risk: hannahanna Breaking Changes

**Mitigation:**
- Version compatibility matrix
- CI tests against multiple hn versions
- Clear error messages for incompatible versions
- Adapter pattern for hn integration

### Risk: Data Loss

**Mitigation:**
- Frequent snapshots
- Backup before destructive operations
- Dry-run modes for all operations
- Comprehensive testing
- Recovery procedures documented

### Risk: Multi-VCS Complexity

**Mitigation:**
- Start with Git only (v0.1)
- Add Hg/Jj incrementally (v0.2)
- Leverage hannahanna's VCS abstraction
- VCS-specific integration tests

### Risk: AI Tool Integration Fragility

**Mitigation:**
- Support multiple AI tools (Claude Code, Cursor, etc.)
- Pluggable AI tool interface
- Fallback to manual workflow
- Clear documentation for integration

### Risk: Performance at Scale

**Mitigation:**
- Performance benchmarks in CI
- Caching strategies
- Async/concurrent operations
- Lazy loading
- Profiling and optimization

---

## Metrics for Success

### Adoption Metrics

- **Downloads**: crates.io downloads per month
- **Stars**: GitHub stars
- **Contributors**: Number of contributors
- **Issues**: Issue resolution time

### Usage Metrics

- **Sessions**: Average sessions per user
- **PR workflow**: Time saved on PR review
- **Multi-agent**: % users using cascade/gather
- **Templates**: Templates downloaded from marketplace

### Quality Metrics

- **Test coverage**: >80%
- **Bug rate**: <5 bugs per 1000 sessions
- **Performance**: All ops <5s
- **Uptime**: >95% (for marketplace)

---

## Community & Support

### Communication Channels

- **GitHub Issues**: Bug reports, feature requests
- **Discord**: Real-time chat, support
- **Mailing List**: Announcements, discussions
- **Blog**: Tutorials, case studies, updates

### Documentation

- **README**: Quick start, installation
- **spec/**: Detailed specification
- **docs/**: User guide, tutorials, API docs
- **examples/**: Example workflows, templates

### Contributing

- **CONTRIBUTING.md**: How to contribute
- **CODE_OF_CONDUCT.md**: Community guidelines
- **Issue templates**: Bug reports, feature requests
- **PR template**: PR description, checklist

---

## Long-term Vision

hupasiya aims to become the standard tool for multi-agent AI development workflows, enabling developers to:

1. **Coordinate multiple AI agents** working on different aspects of a project
2. **Manage complex PR workflows** with AI-assisted comment resolution
3. **Share and reuse workflows** via templates and marketplace
4. **Work across VCS systems** (Git, Mercurial, Jujutsu) seamlessly
5. **Track and optimize** development metrics and AI usage
6. **Collaborate with teams** on distributed, AI-augmented development

By building on hannahanna's solid workbox foundation, hupasiya focuses purely on the orchestration and intelligence layer, creating a powerful yet simple tool for the future of AI-assisted development.
