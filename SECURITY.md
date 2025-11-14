# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Currently supported versions:

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

We take the security of hupasiya seriously. If you have discovered a security vulnerability, please report it to us privately.

**Please do NOT report security vulnerabilities through public GitHub issues.**

### How to Report

Send an email to: **security@rajats.site** (or code@rajats.site)

Include the following information:

- Type of vulnerability (e.g., command injection, path traversal, etc.)
- Full paths of source file(s) related to the vulnerability
- Location of the affected source code (tag/branch/commit or direct URL)
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

### What to Expect

- **Acknowledgment**: We'll acknowledge your email within 48 hours
- **Updates**: We'll send you updates about our progress at least every 5 business days
- **Disclosure Timeline**: We aim to disclose vulnerabilities within 90 days of the initial report
- **Credit**: We'll credit you in the security advisory (unless you prefer to remain anonymous)

## Security Considerations

### Command Injection Risks

hupasiya executes external commands including:
- `hn` (hannahanna) commands
- Git commands
- User-configured AI tools (Claude Code, Cursor, etc.)

**Mitigations:**
- All shell commands use proper argument escaping
- User input is validated before being passed to commands
- Configuration files are parsed and validated
- Session names and paths are sanitized

### File System Access

hupasiya reads and writes to:
- `.hp/` directory structure
- `.hapusiyas.yml` and `.hapusiyas.local.yml` configuration files
- Session context files
- Git repositories via hannahanna workboxes

**Mitigations:**
- Path traversal protection (no `..` in session names)
- File permissions are set appropriately
- Sensitive data (tokens, credentials) should be in `.hapusiyas.local.yml` (gitignored)

### GitHub API Credentials

For PR integration features, hupasiya may access:
- GitHub personal access tokens
- GitHub API via octocrab library

**Best Practices:**
- Store tokens in environment variables or gitignored config files
- Use fine-grained personal access tokens with minimal scopes
- Never commit tokens to version control
- Regularly rotate credentials

### Template Security

Template marketplace features may download and execute templates:

**Mitigations:**
- Templates are markdown files with no code execution
- Template sources should be verified before installation
- Use `--dry-run` to preview template effects

## Safe Usage Guidelines

1. **Credentials**: Never commit `.hapusiyas.local.yml` to version control
2. **AI Tools**: Only configure trusted AI tool commands
3. **Templates**: Review templates before installation
4. **Sessions**: Session names should not contain shell metacharacters
5. **Updates**: Keep hupasiya and hannahanna updated to latest versions

## Known Limitations

- Template marketplace registry server is not yet deployed (v1.0)
- Templates from untrusted sources should be manually reviewed
- AI tool integration relies on external commands (security depends on the tool)

## Security Updates

Security advisories will be published in:
- GitHub Security Advisories
- Release notes in CHANGELOG.md
- Repository README.md

Subscribe to repository releases to stay informed.

## Compliance

This project follows responsible disclosure practices and aims to:
- Respond to vulnerability reports promptly
- Release security patches quickly
- Communicate transparently about security issues
- Give credit to security researchers

Thank you for helping keep hupasiya and its users safe!
