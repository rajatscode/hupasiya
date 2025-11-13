# Testing Strategy

Comprehensive testing approach for hupasiya.

## Testing Pyramid

```
         ┌─────────┐
         │   E2E   │  10%  - Full workflow tests
         └─────────┘
       ┌─────────────┐
       │ Integration │  30%  - hn integration, file I/O
       └─────────────┘
    ┌────────────────────┐
    │    Unit Tests      │  60%  - Business logic, parsers
    └────────────────────┘
```

## Unit Tests

Test individual functions and modules in isolation.

### Target Coverage: 80%+

### Areas to Cover

#### 1. Session Management

```rust
#[cfg(test)]
mod session_tests {
    use super::*;

    #[test]
    fn test_create_session() {
        let session = Session::new(
            "test-session".to_string(),
            AgentType::Feature,
            "test-wb".to_string(),
            PathBuf::from("/tmp/test"),
            "feature/test".to_string(),
            "main".to_string(),
            "myrepo".to_string(),
            "git".to_string(),
        );

        assert_eq!(session.name, "test-session");
        assert_eq!(session.agent_type, AgentType::Feature);
        assert_eq!(session.status, SessionStatus::Active);
    }

    #[test]
    fn test_add_child_session() {
        let mut parent = create_test_session("parent");
        parent.children.push("child1".to_string());
        parent.children.push("child2".to_string());

        assert_eq!(parent.children.len(), 2);
        assert!(parent.children.contains(&"child1".to_string()));
    }

    #[test]
    fn test_session_locking() {
        let mut session = create_test_session("test");
        assert!(!session.is_locked());

        session.locked_by = Some("alice@laptop".to_string());
        assert!(session.is_locked());
    }
}
```

#### 2. Context Management

```rust
#[cfg(test)]
mod context_tests {
    use super::*;

    #[test]
    fn test_create_context_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let context_path = temp_dir.path().join(".hp/contexts/myrepo/test-session");

        create_context_dir(&context_path).unwrap();

        assert!(context_path.exists());
        assert!(context_path.join("context.md").exists());
    }

    #[test]
    fn test_template_substitution() {
        let template = "Session: {{session_name}}\nType: {{session_type}}";
        let vars = hashmap! {
            "session_name" => "auth-feature",
            "session_type" => "feature",
        };

        let result = substitute_template(template, &vars);
        assert_eq!(result, "Session: auth-feature\nType: feature");
    }

    #[test]
    fn test_snapshot_creation() {
        let session = create_test_session("test");
        let snapshot = create_snapshot(&session, "initial").unwrap();

        assert_eq!(snapshot.name, "initial");
        assert!(snapshot.path.exists());
    }
}
```

#### 3. Configuration

```rust
#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let config_str = r#"
hp:
  default_agent: feature
  hn:
    command: hn
    output_format: json
"#;

        let config: Config = serde_yaml::from_str(config_str).unwrap();
        assert_eq!(config.hp.default_agent, AgentType::Feature);
    }

    #[test]
    fn test_config_hierarchy() {
        // Test that local config overrides repo config
        let system_config = load_system_config();
        let user_config = load_user_config();
        let repo_config = load_repo_config();
        let local_config = load_local_config();

        let merged = merge_configs(vec![
            system_config,
            user_config,
            repo_config,
            local_config,
        ]);

        // Local should win
        assert_eq!(merged.hp.active_profile, "local-override");
    }

    #[test]
    fn test_profile_switching() {
        let mut config = create_test_config();
        config.hp.active_profile = "dev".to_string();

        let profile = get_active_profile(&config).unwrap();
        assert_eq!(profile.hn.unwrap().command, "hn");
    }
}
```

#### 4. Metrics & Activity

```rust
#[cfg(test)]
mod metrics_tests {
    use super::*;

    #[test]
    fn test_metrics_update() {
        let mut metrics = SessionMetrics::default();
        metrics.update_from_git_stats(100, 50, 5);

        assert_eq!(metrics.lines_added, 100);
        assert_eq!(metrics.lines_removed, 50);
        assert_eq!(metrics.files_changed, 5);
    }

    #[test]
    fn test_activity_logging() {
        let mut session = create_test_session("test");
        session.log_activity(
            ActivityType::CommitMade,
            "Initial commit".to_string(),
        );

        assert_eq!(session.activity_log.len(), 1);
        assert_eq!(session.activity_log[0].event_type, ActivityType::CommitMade);
    }
}
```

#### 5. Parsers

```rust
#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn test_parse_workbox_path() {
        let output = r#"
Created workbox 'auth-feature'
Path: /home/user/repo-wt-auth-feature
Branch: feature/auth
"#;

        let path = parse_workbox_path(output).unwrap();
        assert_eq!(path, PathBuf::from("/home/user/repo-wt-auth-feature"));
    }

    #[test]
    fn test_parse_git_stats() {
        let output = "5 files changed, 100 insertions(+), 50 deletions(-)";
        let stats = parse_git_stats(output).unwrap();

        assert_eq!(stats.files, 5);
        assert_eq!(stats.added, 100);
        assert_eq!(stats.removed, 50);
    }
}
```

## Integration Tests

Test interaction with external systems (hannahanna, file system, GitHub API).

### Target Coverage: Key integration points

### Areas to Cover

#### 1. hannahanna Integration

```rust
#[cfg(test)]
mod hn_integration_tests {
    use super::*;

    // These tests require hn to be installed
    fn check_hn_available() -> bool {
        which::which("hn").is_ok()
    }

    #[test]
    fn test_hn_add() {
        if !check_hn_available() {
            println!("Skipping: hn not installed");
            return;
        }

        let result = create_workbox("test-integration", &WorkboxOptions {
            from: Some("main".to_string()),
            vcs: Some("git".to_string()),
            sparse: None,
        });

        assert!(result.is_ok());
        let workbox = result.unwrap();
        assert_eq!(workbox.name, "test-integration");

        // Cleanup
        remove_workbox("test-integration", true).unwrap();
    }

    #[test]
    fn test_hn_info() {
        if !check_hn_available() {
            return;
        }

        // Create test workbox
        create_workbox("test-info", &WorkboxOptions::default()).unwrap();

        // Get info
        let info = get_workbox_info("test-info").unwrap();
        assert_eq!(info.name, "test-info");
        assert!(info.path.exists());

        // Cleanup
        remove_workbox("test-info", true).unwrap();
    }

    #[test]
    fn test_hn_exec() {
        if !check_hn_available() {
            return;
        }

        create_workbox("test-exec", &WorkboxOptions::default()).unwrap();

        let output = exec_in_workbox("test-exec", "echo hello").unwrap();
        assert_eq!(output.trim(), "hello");

        remove_workbox("test-exec", true).unwrap();
    }
}
```

#### 2. File System Operations

```rust
#[cfg(test)]
mod fs_integration_tests {
    use super::*;

    #[test]
    fn test_session_persistence() {
        let temp_dir = tempfile::tempdir().unwrap();
        let sessions_dir = temp_dir.path().join(".hp/sessions");
        fs::create_dir_all(&sessions_dir).unwrap();

        let session = create_test_session("test");
        save_session(&session, &sessions_dir).unwrap();

        let loaded = load_session("test", &sessions_dir).unwrap();
        assert_eq!(loaded.name, session.name);
        assert_eq!(loaded.id, session.id);
    }

    #[test]
    fn test_context_file_operations() {
        let temp_dir = tempfile::tempdir().unwrap();
        let context_dir = temp_dir.path().join(".hp/contexts/myrepo/test");
        fs::create_dir_all(&context_dir).unwrap();

        // Create context file
        let context_path = context_dir.join("context.md");
        fs::write(&context_path, "# Test Context\n").unwrap();

        // Read it back
        let content = fs::read_to_string(&context_path).unwrap();
        assert_eq!(content, "# Test Context\n");
    }
}
```

#### 3. GitHub API Integration

```rust
#[cfg(test)]
mod github_integration_tests {
    use super::*;

    // Requires GITHUB_TOKEN env var
    fn check_github_token() -> bool {
        std::env::var("GITHUB_TOKEN").is_ok()
    }

    #[test]
    #[ignore] // Run manually with: cargo test --ignored
    fn test_create_pr() {
        if !check_github_token() {
            return;
        }

        let pr = create_github_pr(
            "test-org",
            "test-repo",
            "Test PR",
            "feature/test",
            "main",
            "Test PR body",
        );

        assert!(pr.is_ok());
        let pr = pr.unwrap();
        assert!(pr.number > 0);
    }

    #[test]
    #[ignore]
    fn test_fetch_pr_comments() {
        if !check_github_token() {
            return;
        }

        let comments = fetch_pr_comments("test-org", "test-repo", 123);
        assert!(comments.is_ok());
    }
}
```

## End-to-End Tests

Test complete workflows from user perspective.

### Target: Critical workflows

### Workflows to Test

#### 1. Feature Development Workflow

```rust
#[test]
fn test_feature_workflow() {
    if !check_hn_available() {
        return;
    }

    // 1. Create feature session
    let result = run_hp_command(vec!["new", "e2e-feature", "--type=feature"]);
    assert!(result.is_ok());

    // 2. Verify session created
    let sessions = list_sessions().unwrap();
    assert!(sessions.iter().any(|s| s.name == "e2e-feature"));

    // 3. Edit context
    let context_path = get_context_path("e2e-feature").unwrap();
    assert!(context_path.exists());

    // 4. Create child test session
    let result = run_hp_command(vec![
        "new", "e2e-feature-tests",
        "--parent=e2e-feature",
        "--type=test"
    ]);
    assert!(result.is_ok());

    // 5. Cascade
    let result = run_hp_command(vec!["cascade", "e2e-feature"]);
    assert!(result.is_ok());

    // 6. Gather
    let result = run_hp_command(vec!["gather", "e2e-feature"]);
    assert!(result.is_ok());

    // Cleanup
    run_hp_command(vec!["close", "e2e-feature-tests", "--remove-workbox"]).unwrap();
    run_hp_command(vec!["close", "e2e-feature", "--remove-workbox"]).unwrap();
}
```

#### 2. PR Shepherd Workflow

```rust
#[test]
#[ignore] // Requires GitHub API
fn test_shepherd_workflow() {
    if !check_hn_available() || !check_github_token() {
        return;
    }

    // 1. Create session with PR
    run_hp_command(vec!["new", "e2e-pr", "--type=feature"]).unwrap();

    // 2. Create PR
    run_hp_command(vec!["pr", "create", "e2e-pr"]).unwrap();

    // 3. Sync PR comments
    run_hp_command(vec!["pr", "sync", "e2e-pr"]).unwrap();

    // 4. Run shepherd (dry-run)
    let result = run_hp_command(vec!["shepherd", "--dry-run"]);
    assert!(result.is_ok());

    // 5. Verify shepherd.md created
    let shepherd_path = get_shepherd_path("e2e-pr").unwrap();
    assert!(shepherd_path.exists());

    // Cleanup
    run_hp_command(vec!["close", "e2e-pr", "--remove-workbox"]).unwrap();
}
```

#### 3. Multi-Agent Workflow

```rust
#[test]
fn test_multi_agent_workflow() {
    if !check_hn_available() {
        return;
    }

    // Create parent
    run_hp_command(vec!["new", "parent"]).unwrap();

    // Create children
    run_hp_command(vec!["new", "child1", "--parent=parent"]).unwrap();
    run_hp_command(vec!["new", "child2", "--parent=parent"]).unwrap();

    // Verify tree
    let tree = get_session_tree("parent").unwrap();
    assert_eq!(tree.children.len(), 2);

    // Cascade to all
    run_hp_command(vec!["cascade", "parent"]).unwrap();

    // Gather from all
    run_hp_command(vec!["gather", "parent"]).unwrap();

    // Cleanup
    run_hp_command(vec!["close", "child1", "--remove-workbox"]).unwrap();
    run_hp_command(vec!["close", "child2", "--remove-workbox"]).unwrap();
    run_hp_command(vec!["close", "parent", "--remove-workbox"]).unwrap();
}
```

## Performance Tests

Test performance at scale.

### Benchmarks

```rust
#[bench]
fn bench_list_sessions(b: &mut Bencher) {
    // Create 100 sessions
    for i in 0..100 {
        create_test_session(&format!("session-{}", i));
    }

    b.iter(|| {
        list_sessions().unwrap()
    });
}

#[bench]
fn bench_cascade(b: &mut Bencher) {
    let parent = create_test_session("parent");
    for i in 0..10 {
        create_child_session("parent", &format!("child-{}", i));
    }

    b.iter(|| {
        cascade(&parent).unwrap()
    });
}

#[bench]
fn bench_parse_json_output(b: &mut Bencher) {
    let json = r#"{"name":"test","path":"/tmp/test","branch":"main"}"#;

    b.iter(|| {
        serde_json::from_str::<WorkboxInfo>(json).unwrap()
    });
}
```

### Performance Requirements

- `hp list`: <100ms for 50 sessions
- `hp new`: <5s including workbox creation
- `hp cascade` to 5 children: <30s
- `hp gather` from 5 children: <60s
- `hp shepherd` analysis: <5s per comment
- `hp info`: <500ms

## Test Infrastructure

### Test Utilities

```rust
// tests/common/mod.rs

pub fn create_test_session(name: &str) -> Session {
    Session::new(
        name.to_string(),
        AgentType::Feature,
        name.to_string(),
        PathBuf::from(format!("/tmp/{}", name)),
        format!("feature/{}", name),
        "main".to_string(),
        "test-repo".to_string(),
        "git".to_string(),
    )
}

pub fn create_test_config() -> Config {
    // Load from test fixtures
    let config_str = include_str!("fixtures/test_config.yml");
    serde_yaml::from_str(config_str).unwrap()
}

pub fn run_hp_command(args: Vec<&str>) -> Result<String> {
    let output = Command::new(env!("CARGO_BIN_EXE_hp"))
        .args(args)
        .output()
        .context("Failed to run hp command")?;

    if !output.status.success() {
        bail!("Command failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn check_hn_available() -> bool {
    which::which("hn").is_ok()
}

pub fn check_github_token() -> bool {
    std::env::var("GITHUB_TOKEN").is_ok()
}
```

### Test Fixtures

```
tests/
├── fixtures/
│   ├── test_config.yml
│   ├── test_session.yaml
│   ├── test_template.md
│   └── test_pr_comments.json
├── integration/
│   ├── hn_tests.rs
│   ├── fs_tests.rs
│   └── github_tests.rs
├── e2e/
│   ├── feature_workflow.rs
│   ├── shepherd_workflow.rs
│   └── multi_agent_workflow.rs
└── common/
    └── mod.rs
```

## Continuous Integration

### GitHub Actions Workflow

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest]
        rust: [stable, nightly]

    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: ${{ matrix.rust }}
          override: true

      - name: Install hannahanna
        run: cargo install hannahanna

      - name: Run unit tests
        run: cargo test --lib

      - name: Run integration tests
        run: cargo test --test '*'

      - name: Run clippy
        run: cargo clippy -- -D warnings

      - name: Check formatting
        run: cargo fmt -- --check

      - name: Build
        run: cargo build --release

  coverage:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3

      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin

      - name: Generate coverage
        run: cargo tarpaulin --out Xml

      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          file: ./cobertura.xml

  e2e:
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'

    steps:
      - uses: actions/checkout@v3

      - name: Install hannahanna
        run: cargo install hannahanna

      - name: Run E2E tests
        run: cargo test --test e2e -- --ignored
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Test Coverage Goals

### v0.1.0
- Unit tests: 70%+
- Integration tests: Key hn integration
- E2E tests: Basic workflow

### v0.2.0
- Unit tests: 75%+
- Integration tests: Multi-VCS, cascade/gather
- E2E tests: Multi-agent workflows

### v0.3.0
- Unit tests: 80%+
- Integration tests: GitHub API, shepherd
- E2E tests: PR workflows

### v1.0.0
- Unit tests: 80%+
- Integration tests: All integration points
- E2E tests: All critical workflows
- Performance tests: All benchmarks meet requirements

## Testing Best Practices

1. **Isolation**: Tests should not depend on each other
2. **Cleanup**: Always clean up resources (workboxes, temp files)
3. **Mocking**: Mock external dependencies (hn, GitHub API) for unit tests
4. **Real Integration**: Use real hn for integration tests
5. **Fast Feedback**: Unit tests should run in <10s
6. **Deterministic**: Tests should be repeatable and deterministic
7. **Clear Assertions**: Use descriptive assertion messages
8. **Test Data**: Use realistic test data
9. **Error Cases**: Test both success and failure paths
10. **Documentation**: Document why tests exist and what they verify
