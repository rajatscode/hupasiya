# hupasiya Specification

**Version:** 1.0
**Command:** `hp`
**Name Origin:** Hupasiya - Hittite hero who slayed the dragon Illuyanka

## Overview

hupasiya (`hp`) is a multi-agent session orchestrator built on top of [hannahanna](https://github.com/rajatscode/hannahanna). It provides coordinated AI agent workflows with isolated workspaces, PR integration, and multi-agent coordination.

### Relationship to hannahanna

- **hannahanna (hn):** Intelligent workbox manager - handles git/hg/jj workspaces, resource sharing, Docker environments, state management
- **hupasiya (hp):** Multi-agent orchestrator - handles AI sessions, context management, PR workflows, agent coordination

### Key Principles

1. **Loose Coupling:** hupasiya calls `hn` commands as an external tool, no tight library coupling
2. **Separation of Concerns:** hn manages workboxes, hp manages sessions and context
3. **Multi-VCS Support:** Works with Git, Mercurial, and Jujutsu through hannahanna
4. **Independent Development:** Can be developed, versioned, and distributed separately from hannahanna

## Specification Documents

### Core Design
- **[Architecture](architecture.md)** - System architecture, loose coupling design, component overview
- **[Data Model](data-model.md)** - Core data structures (Session, Context, Metrics, etc.)
- **[Configuration](configuration.md)** - Config file format, hierarchy, profiles

### Functionality
- **[Commands](commands.md)** - Complete CLI command reference
- **[Workflows](workflows.md)** - Common workflows and use cases
- **[Context Structure](context-structure.md)** - `.hp/` directory structure and context management
- **[Integration](integration.md)** - How hupasiya integrates with hannahanna

### Development
- **[Roadmap](roadmap.md)** - Version goals, milestones, success criteria
- **[Testing](testing.md)** - Testing requirements and strategy
- **[Contributing](contributing.md)** - Development setup, conventions, PR process

## Quick Start

### Installation Requirements

```bash
# Install hannahanna first
cargo install hannahanna

# Then install hupasiya
cargo install hupasiya

# Verify installation
hn --version
hp --version
```

### Basic Usage

```bash
# Create a new session
hp new auth-feature

# Launch AI agent with context
hp launch

# Create child session for tests
hp new auth-tests --parent=auth-feature --type=test

# Sync parent changes to children
hp cascade auth-feature

# Create PR
hp pr create

# Address PR comments
hp shepherd
```

## Terminology

- **Workbox:** An isolated development environment managed by hannahanna (VCS workspace + deps + Docker + state)
- **Session:** An AI agent context managed by hupasiya (objectives + conversation + PR integration + context)
- **Cascade:** Syncing parent session changes to child sessions
- **Gather:** Collecting child session work back to parent
- **Shepherd:** Interactive PR comment resolution workflow

## Philosophy

Each hupasiya session gets a hannahanna workbox. One workspace, one context, one objective.

hannahanna provides the infrastructure (isolated workboxes), hupasiya provides the intelligence (AI agent coordination).

## Status

**Current Version:** 1.0.0
**Status:** Production Ready ✅
**Completed:** All features from v0.1 through v1.0 implemented and tested

See [Roadmap](roadmap.md) for detailed version goals and [CHANGELOG.md](../CHANGELOG.md) for release notes.
