# Contributing to hupasiya

Thank you for your interest in contributing to hupasiya! We welcome contributions from the community.

## Quick Start

1. **Fork** the repository
2. **Clone** your fork: `git clone https://github.com/YOUR_USERNAME/hupasiya.git`
3. **Create a branch**: `git checkout -b feature/my-feature`
4. **Make changes** and add tests
5. **Run checks**: `cargo test && cargo clippy && cargo fmt`
6. **Commit**: `git commit -m "feat: add my feature"`
7. **Push**: `git push origin feature/my-feature`
8. **Create a Pull Request**

## Code Quality

Before submitting a PR, ensure:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test

# Build
cargo build --release
```

## Commit Message Format

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Test changes
- `refactor:` - Code refactoring
- `chore:` - Maintenance tasks
- `perf:` - Performance improvements

Examples:
```
feat: add session cloning command
fix: resolve crash when hn not installed
docs: update installation instructions
test: add tests for PR sync functionality
```

## Development Setup

```bash
# Install dependencies
cargo build

# Install git hooks (optional but recommended)
./scripts/install-hooks.sh

# Install hannahanna (required dependency)
cargo install hannahanna

# Run tests
cargo test

# Run in development
cargo run -- <command>
```

## Project Structure

```
hupasiya/
├── src/              # Source code
│   ├── main.rs       # CLI entry point
│   ├── cli/          # CLI commands
│   ├── session/      # Session management
│   ├── context/      # Context management
│   ├── hn_client/    # hannahanna integration
│   ├── orchestration/# Multi-agent coordination
│   ├── pr/           # PR integration
│   └── config/       # Configuration
├── tests/            # Integration tests
├── spec/             # Specification documents
└── .githooks/        # Git hooks
```

## Testing

- Write unit tests for new functionality
- Add integration tests for complex features
- Ensure all tests pass before submitting PR
- Aim for >80% code coverage

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

## Documentation

- Document all public APIs with rustdoc comments
- Update README.md if adding user-facing features
- Update spec/ documentation for architectural changes
- Add examples for new commands

## Code Style

- Use `rustfmt` for formatting (enforced by pre-commit hook)
- Use `clippy` for linting (enforced by CI)
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Keep functions small and focused
- Add comments for complex logic

## Pull Request Process

1. Update documentation if needed
2. Add tests for new functionality
3. Ensure all CI checks pass
4. Request review from maintainers
5. Address review feedback
6. Squash commits if requested

## Detailed Contributing Guide

For comprehensive contributing guidelines, including:
- Architecture overview
- Module-specific guidelines
- PR review criteria
- Release process
- and more...

**See the detailed guide: [spec/contributing.md](spec/contributing.md)**

## Code of Conduct

Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md).

## Security

For security vulnerability reporting, see [SECURITY.md](SECURITY.md).

## Need Help?

- Read the [documentation](spec/README.md)
- Check [existing issues](https://github.com/rajatscode/hupasiya/issues)
- Ask questions in [discussions](https://github.com/rajatscode/hupasiya/discussions)
- Join our community chat (coming soon)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to hupasiya! 🎉
