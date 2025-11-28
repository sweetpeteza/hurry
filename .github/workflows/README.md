# GitHub Actions Workflows

This directory contains the CI/CD pipeline configuration for the `hurry` project.

## Overview

The project uses GitHub Actions for automated testing, linting, and releasing. All workflows are defined as YAML files in this directory.

## Workflows

### ci.yml

The Continuous Integration workflow runs on every push and pull request to ensure code quality.

**Triggers:**
- Push to any branch
- Pull request to any branch

**Jobs:**
- **test** - Runs the test suite across multiple Rust versions
- **fmt** - Checks code formatting with `rustfmt`
- **clippy** - Runs the Clippy linter for code quality

**Rust Versions Tested:**
- stable - Latest stable Rust release
- beta - Beta channel (testing upcoming features)
- nightly - Nightly channel (experimental features)

**Test Matrix:**
The CI runs on multiple operating systems:
- Linux (ubuntu-latest)
- macOS (macos-latest)
- Windows (windows-latest)

**Commands Executed:**
```bash
cargo test --all-features    # Run all tests
cargo fmt -- --check         # Check formatting
cargo clippy -- -D warnings  # Lint with warnings as errors
```

### release.yml

The Release workflow automates the process of publishing new versions to crates.io.

**Triggers:**
- Manual dispatch (workflow_dispatch)
- Git tags matching `v*.*.*` (e.g., `v0.1.0`)

**Jobs:**
1. **Build and Test** - Verifies the crate builds successfully
2. **Publish** - Publishes to crates.io using the `CARGO_TOKEN` secret

**Prerequisites:**
- `CARGO_TOKEN` secret must be configured in repository settings
- Git tag must follow semantic versioning (v + semver)

**Publishing Order:**
If publishing multiple crates, dependencies are published first:
1. `hurry-macros` (if applicable)
2. `hurry` (main crate)

## Secrets Required

### CARGO_TOKEN
Token for publishing to crates.io.

**How to set up:**
1. Generate a token at https://crates.io/settings/tokens
2. Add it to repository secrets:
   - Go to repository Settings
   - Navigate to Secrets and variables > Actions
   - Add new repository secret named `CARGO_TOKEN`

## Local Testing

Before pushing, you can run the same checks locally:

```bash
# Run tests
cargo test --all-features

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy --all-features -- -D warnings

# Build release
cargo build --release
```

## Workflow Status

You can view the status of all workflows:
- In the "Actions" tab of the GitHub repository
- As status badges in the README (if configured)

## Customizing Workflows

### Modifying Test Matrix

To add or remove Rust versions, edit the `matrix` section in `ci.yml`:

```yaml
strategy:
  matrix:
    rust: [stable, beta, nightly]
    os: [ubuntu-latest, macos-latest, windows-latest]
```

### Adding New Jobs

To add a new job to CI:

1. Add a new job section in `ci.yml`:
```yaml
new_job:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v3
    - uses: dtolnay/rust-toolchain@stable
    - run: cargo your-command
```

2. Test locally first
3. Commit and push to trigger the workflow

### Caching

The workflows use GitHub Actions caching to speed up builds:
- Cargo registry cache
- Cargo build cache
- Target directory cache

This significantly reduces CI run times for subsequent builds.

## Troubleshooting

### CI Failing on Formatting

Run `cargo fmt` locally and commit the changes:
```bash
cargo fmt
git add .
git commit -m "fix: apply rustfmt formatting"
```

### CI Failing on Clippy

Fix the warnings or allow specific lints:
```bash
cargo clippy --fix
# or add #[allow(clippy::lint_name)] to code
```

### Release Workflow Failing

Common issues:
- Missing or invalid `CARGO_TOKEN`
- Version already published to crates.io
- Cargo.toml version doesn't match git tag
- Tests failing

## Best Practices

1. **Always run tests locally** before pushing
2. **Keep workflows fast** - use caching and parallel jobs
3. **Pin action versions** - use specific versions (e.g., `@v3`) not `@latest`
4. **Test on multiple platforms** - catch OS-specific issues early
5. **Use matrix builds** - test across Rust versions efficiently

## Related Documentation

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [rust-toolchain Actions](https://github.com/dtolnay/rust-toolchain)
- [Publishing to crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)
