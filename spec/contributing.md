# Contributing to hupasiya

Thank you for your interest in contributing to hupasiya!

## Development Setup

### Prerequisites

- Rust 1.70 or later
- hannahanna installed (`cargo install hannahanna`)
- Git
- (Optional) One of: Mercurial, Jujutsu for multi-VCS testing

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/yourorg/hupasiya.git
cd hupasiya

# Build
cargo build

# Run tests
cargo test

# Run clippy
cargo clippy

# Format code
cargo fmt

# Install locally for testing
cargo install --path .
```

### Project Structure

```
hupasiya/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── cli/                 # CLI commands
│   │   ├── new.rs
│   │   ├── list.rs
│   │   ├── switch.rs
│   │   ├── close.rs
│   │   └── ...
│   ├── session/             # Session management
│   │   ├── mod.rs
│   │   ├── manager.rs
│   │   └── metadata.rs
│   ├── context/             # Context management
│   │   ├── mod.rs
│   │   ├── manager.rs
│   │   └── template.rs
│   ├── hn/                  # hannahanna integration
│   │   ├── mod.rs
│   │   ├── client.rs
│   │   └── parser.rs
│   ├── orchestration/       # Multi-agent coordination
│   │   ├── mod.rs
│   │   ├── cascade.rs
│   │   └── gather.rs
│   ├── pr/                  # PR integration
│   │   ├── mod.rs
│   │   ├── github.rs
│   │   └── shepherd.rs
│   ├── config/              # Configuration
│   │   ├── mod.rs
│   │   └── loader.rs
│   └── util/                # Utilities
│       └── mod.rs
├── tests/
│   ├── integration/         # Integration tests
│   ├── e2e/                 # End-to-end tests
│   └── common/              # Test utilities
├── spec/                    # Specification documents
├── Cargo.toml
└── README.md
```

## Development Workflow

### 1. Find or Create an Issue

- Check [GitHub Issues](https://github.com/yourorg/hupasiya/issues)
- Comment on an issue to claim it
- For new features, create an issue first to discuss

### 2. Create a Branch

```bash
# Create branch from main
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/bug-description
```

### 3. Make Changes

- Write code following the style guide (below)
- Add tests for new functionality
- Update documentation if needed
- Run tests locally

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### 4. Commit Changes

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```bash
# Format: <type>(<scope>): <description>

# Examples:
git commit -m "feat(session): add session tree visualization"
git commit -m "fix(cascade): handle merge conflicts correctly"
git commit -m "docs(readme): update installation instructions"
git commit -m "test(shepherd): add tests for PR comment parsing"
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Adding or updating tests
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `chore`: Maintenance tasks

### 5. Push and Create PR

```bash
# Push branch
git push -u origin feature/your-feature-name

# Create PR on GitHub
gh pr create --title "Add session tree visualization" --body "..."
```

## Code Style Guide

### Rust Conventions

#### Formatting

Use `rustfmt` (enforced by pre-commit hook):

```bash
cargo fmt
```

#### Linting

Use `clippy` (enforced by pre-commit hook):

```bash
cargo clippy -- -D warnings
```

#### Naming

- **Types/Structs/Enums**: `PascalCase`
- **Functions/Variables**: `snake_case`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`

```rust
// Good
pub struct SessionManager {
    sessions: Vec<Session>,
}

pub fn create_session(name: &str) -> Result<Session> {
    // ...
}

const MAX_SESSIONS: usize = 100;

// Bad
pub struct session_manager { }  // Wrong case
pub fn CreateSession() { }      // Wrong case
```

#### Documentation

All public APIs must have doc comments:

```rust
/// Creates a new session with the given name and type.
///
/// # Arguments
///
/// * `name` - The session name
/// * `agent_type` - Type of AI agent
///
/// # Returns
///
/// Returns `Ok(Session)` on success, or an error if:
/// - Session name already exists
/// - hannahanna command fails
/// - Unable to create context directory
///
/// # Examples
///
/// ```
/// use hupasiya::session::create_session;
/// use hupasiya::AgentType;
///
/// let session = create_session("my-session", AgentType::Feature)?;
/// ```
pub fn create_session(name: &str, agent_type: AgentType) -> Result<Session> {
    // ...
}
```

#### Error Handling

Use `anyhow` for application code:

```rust
use anyhow::{Context, Result, bail};

pub fn load_session(name: &str) -> Result<Session> {
    let path = get_session_path(name)
        .context("Failed to get session path")?;

    if !path.exists() {
        bail!("Session '{}' not found", name);
    }

    let content = fs::read_to_string(&path)
        .context(format!("Failed to read session file: {}", path.display()))?;

    serde_yaml::from_str(&content)
        .context("Failed to parse session metadata")
}
```

Use `thiserror` for library errors (if we extract a library):

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("Session '{0}' not found")]
    NotFound(String),

    #[error("Session '{0}' already exists")]
    AlreadyExists(String),

    #[error("Invalid session name: {0}")]
    InvalidName(String),
}
```

#### Testing

Write tests for all non-trivial functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_session() {
        let session = Session::new(
            "test".to_string(),
            AgentType::Feature,
            // ...
        );

        assert_eq!(session.name, "test");
        assert_eq!(session.status, SessionStatus::Active);
    }

    #[test]
    fn test_add_child_session() {
        let mut parent = create_test_session("parent");
        parent.children.push("child".to_string());

        assert_eq!(parent.children.len(), 1);
        assert!(parent.children.contains(&"child".to_string()));
    }

    #[test]
    #[should_panic(expected = "Session not found")]
    fn test_load_nonexistent_session() {
        load_session("nonexistent").unwrap();
    }
}
```

## Testing Guidelines

### Test Categories

1. **Unit Tests** (`src/**/*.rs`):
   - Test individual functions
   - Mock external dependencies
   - Fast (<10s for all unit tests)

2. **Integration Tests** (`tests/integration/**`):
   - Test interaction with hannahanna
   - Test file system operations
   - Require `hn` to be installed

3. **E2E Tests** (`tests/e2e/**`):
   - Test complete workflows
   - Run full commands
   - Slower, run on CI only

### Running Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test integration

# E2E tests (slow)
cargo test --test e2e -- --ignored

# Specific test
cargo test test_session_creation

# With output
cargo test -- --nocapture

# With backtrace
RUST_BACKTRACE=1 cargo test
```

### Writing Good Tests

```rust
#[test]
fn test_something() {
    // Arrange: Set up test data
    let input = create_test_input();

    // Act: Perform the action
    let result = do_something(input);

    // Assert: Check the result
    assert_eq!(result, expected);
}
```

**Good practices**:
- Clear test names: `test_<what>_<condition>_<expected>`
- One assertion per test (when possible)
- Use test utilities in `tests/common/mod.rs`
- Clean up resources (use `tempfile` for temp files)
- Don't depend on test order

## Pull Request Guidelines

### PR Title

Use conventional commit format:

```
feat(session): add session tree visualization
fix(cascade): handle merge conflicts correctly
docs(readme): update installation instructions
```

### PR Description

Include:

1. **Summary**: What does this PR do?
2. **Motivation**: Why is this change needed?
3. **Changes**: What changed?
4. **Testing**: How was this tested?
5. **Screenshots**: (if UI changes)
6. **Breaking Changes**: (if any)

**Template**:

```markdown
## Summary

Adds session tree visualization with `hp tree` command.

## Motivation

Users need a way to visualize parent/child session relationships.

## Changes

- Added `hp tree` command
- Added tree rendering logic
- Added tests for tree visualization
- Updated documentation

## Testing

- Unit tests: `cargo test tree`
- Manual testing: Created parent/child sessions and verified output

## Breaking Changes

None
```

### PR Checklist

Before submitting:

- [ ] Code follows style guide
- [ ] Tests pass locally (`cargo test`)
- [ ] Clippy passes (`cargo clippy`)
- [ ] Format is correct (`cargo fmt`)
- [ ] Documentation is updated
- [ ] CHANGELOG is updated
- [ ] Commit messages follow convention

### Review Process

1. **Automated Checks**: CI must pass
2. **Code Review**: At least one approval required
3. **Discussion**: Address reviewer feedback
4. **Approval**: Maintainer approves and merges

## Git Hooks

The project uses git hooks to enforce code quality:

### Pre-commit Hook

Runs on every commit:
- `cargo fmt` - Format code
- `cargo clippy` - Lint code

### Pre-push Hook

Runs on every push:
- `cargo test` - Run all tests

### Installing Hooks

Hooks are set up automatically when you clone the repo. To manually install:

```bash
# Install hooks
./scripts/install-hooks.sh

# Or manually
cp .githooks/pre-commit .git/hooks/pre-commit
cp .githooks/pre-push .git/hooks/pre-push
chmod +x .git/hooks/pre-commit .git/hooks/pre-push
```

### Skipping Hooks

In emergencies only:

```bash
# Skip pre-commit
git commit --no-verify

# Skip pre-push
git push --no-verify
```

## Release Process

Releases are managed by maintainers.

### Version Bumping

Follow [Semantic Versioning](https://semver.org/):

- **Major** (1.0.0): Breaking changes
- **Minor** (0.1.0): New features, backward compatible
- **Patch** (0.1.1): Bug fixes, backward compatible

### Release Steps

1. Update `Cargo.toml` version
2. Update `CHANGELOG.md`
3. Create git tag: `git tag v0.1.0`
4. Push tag: `git push --tags`
5. CI publishes to crates.io
6. Create GitHub release with notes

## Documentation

### User Documentation

- **README.md**: Quick start, installation
- **spec/**: Detailed specification
- **CHANGELOG.md**: Version history

### Code Documentation

- Doc comments for all public APIs
- Examples in doc comments
- Module-level documentation

### Generating Docs

```bash
# Generate and open docs
cargo doc --open
```

## Communication

### Channels

- **GitHub Issues**: Bug reports, feature requests
- **GitHub Discussions**: General questions, ideas
- **Discord**: (if applicable) Real-time chat
- **Email**: (if applicable) Maintainer contact

### Issue Templates

Use issue templates when creating issues:

- **Bug Report**: For reporting bugs
- **Feature Request**: For suggesting features
- **Question**: For asking questions

### Code of Conduct

Be respectful and professional. See CODE_OF_CONDUCT.md.

## Getting Help

### For Developers

- Read the spec docs in `spec/`
- Check existing issues and PRs
- Ask in GitHub Discussions
- Read the code and tests

### For Users

- Check README and documentation
- Search existing issues
- Create new issue if needed

## License

By contributing, you agree that your contributions will be licensed under the MIT License (or project license).

---

Thank you for contributing to hupasiya! 🎉
