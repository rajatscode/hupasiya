# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-01-13

### Added - v1.0 Production Release
All features from v0.1 through v1.0 implemented and tested.

#### Core Features (v0.1)
- Complete session management (new, list, info, close, switch)
- hannahanna integration for workbox management
- Context management with snapshots and templates
- 4-level configuration hierarchy
- Doctor command for system health checks

#### Multi-Agent Orchestration (v0.2)
- `cascade`: Sync parent changes to all children
- `gather`: Collect children back to parent
- `tree`: Visualize session hierarchies
- AI tool integration (launch, shell, exec)
- Multiple launch methods (exec, tmux, screen, shell-function)

#### PR & Shepherd Workflows (v0.3)
- Full GitHub API integration via octocrab
- `pr create`: Create PRs with reviewers/labels/draft support
- `pr sync`: Fetch unresolved comments, generate shepherd.md
- `pr status`: Display PR state and metrics
- Interactive shepherd workflow for comment resolution
- Batch shepherd mode for automated triage
- Activity logging and metrics tracking

#### Collaboration & Marketplace (v0.4)
- `handoff`: Transfer sessions between developers
- `clone`: Duplicate sessions for parallel work
- `merge-sessions`: Consolidate work from multiple sessions
- Template marketplace (list, install, search, publish)
- Configuration profiles for different workflows

#### Utilities & Polish (v1.0)
- `monitor`: Live dashboard of all sessions
- `clean`: Remove old/archived sessions
- `leave`: Gracefully exit with optional archival
- Activity and metrics commands
- Shell completion generation ready
- Comprehensive error messages
- 56 passing tests

### Technical
- Async GitHub API with proper error handling
- Interactive CLI with dialoguer
- Colored terminal output
- Cross-platform support
- Type-safe configuration
- Extensive test coverage

## [Unreleased]

### Planned for v1.1
- Shell completion scripts generation
- Enhanced template metadata parsing
- Marketplace backend integration
- Performance optimizations

## [0.1.0] - 2025-01-10

### Initial Implementation
- Project structure and specifications
- Git hooks for quality
- Basic CLI framework

---

**Note**: This project is in early development. The first release (v0.1.0) is planned for Q1 2025.

[unreleased]: https://github.com/yourorg/hupasiya/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourorg/hupasiya/releases/tag/v0.1.0
