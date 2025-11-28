# Agent Instructions for hurry

## Terminal Output Guidelines

**IMPORTANT: Always show command output to the user when running terminal commands.**

### Best Practices
- **Always include output** - Users need to see what commands are doing
- **Use concise summaries** - For long output, show key parts (beginning/end)
- **Filter intelligently** - Show errors, warnings, and important status messages
- **Tail long output** - Use `| tail -n 20` for builds/tests to show final results
- **Show progress** - Users should see compilation, test results, and status updates

### Command Output Patterns

**For build commands:**
```bash
cargo build 2>&1 | tail -n 30  # Show last 30 lines including warnings/errors
```

**For test commands:**
```bash
cargo test 2>&1 | tail -n 50  # Show test results summary
```

**For long-running commands:**
```bash
cargo clippy 2>&1 | tail -n 40  # Show linting results
```

**When you need full output:**
```bash
cargo fmt --check  # Short output, show all
git status         # Always show full output
```

### Why This Matters
- Users can track progress and see what's happening
- Errors and warnings are immediately visible
- Builds provide important feedback about compilation
- Test output shows what passed/failed
- Users maintain context about system state

## Quick Commands (Just)

This project uses [just](https://github.com/casey/just) as a command runner to simplify common workflows.

**Most Common Commands:**
- `just check` - Run all pre-commit checks (format, clippy, test)
- `just dev` - Quick development workflow (format + test)
- `just ci` - Run full CI workflow locally
- `just clippy-fix` - Auto-fix clippy warnings
- `just --list` - Show all available commands

**Individual Commands:**
- `just fmt` - Format code
- `just clippy` - Run linter
- `just test` - Run all tests
- `just build` - Build the project
- `just docs` - Build and open documentation

**See `just --list` for the complete list of available commands.**

## Build Commands
- `cargo build` or `just build` - Build the project
- `cargo build --release` or `just build-release` - Build optimized release version

## Test Commands
- `cargo test` or `just test` - Run all tests
- `cargo test <test_name>` - Run a specific test (e.g., `cargo test test_boxx_macro`)
- `just test-verbose` - Run tests with output

## Lint and Format
- `cargo clippy` or `just clippy` - Run linter
- `cargo clippy --fix` or `just clippy-fix` - Automatically apply clippy suggestions
- `cargo fmt` or `just fmt` - Format code

### Using Clippy
- Always run `cargo clippy` to check for code quality issues
- Use `cargo clippy --fix` to automatically apply safe suggestions
- Review clippy warnings and apply fixes when appropriate
- Treat clippy suggestions as code improvement opportunities
- For complex issues, manually review and implement clippy's recommended changes

## Code Style Guidelines

### Naming Conventions
- Macros: snake_case, use alternative names if conflicts with keywords (e.g., `boxx` instead of `box_`)
- Functions: snake_case
- Types: PascalCase

### Documentation
- Use `///` for public API documentation with examples
- Include `# Example` sections in macro docs

### Error Handling
- Use `.unwrap()` in tests only
- Prefer `?` operator for error propagation in library code

### Imports
- Use full paths in macro expansions (e.g., `std::rc::Rc::new`)
- Import types in documentation examples when needed

### Testing
- Place tests in `#[cfg(test)]` modules
- Use descriptive test function names starting with `test_`
- Test both success and failure cases where applicable

### Formatting
- Follow `cargo fmt` output
- Use 4 spaces for indentation (Rust default)

## Git and GitHub Conventions

### Commit Messages (Conventional Commits)

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

**Format:** `type(scope): description`

**Types:**
- `feat` - New feature or functionality
- `fix` - Bug fix
- `docs` - Documentation changes only
- `style` - Code style changes (formatting, missing semicolons, etc.)
- `refactor` - Code changes that neither fix bugs nor add features
- `test` - Adding or updating tests
- `chore` - Maintenance tasks, dependency updates, tooling changes
- `ci` - CI/CD configuration changes
- `perf` - Performance improvements
- `build` - Build system or build process changes

**Guidelines:**
- Keep first line under 50 characters
- Use imperative mood ("Add feature" not "Added feature")
- Don't capitalize first letter of description
- No period at the end of the subject line
- Add detailed description in body for complex changes (separated by blank line)
- Reference issues in footer (e.g., "Fixes #123" or "Closes #456")

**Examples:**
```
feat: add boxx macro for Box creation
fix: correct pin_box double dereference in tests
docs: update README with new macro examples
refactor: rename box_ to boxx for cleaner API
test: add comprehensive test coverage for all macros
chore: update dependencies to latest versions
ci: add caching to GitHub Actions workflow
docs: add comprehensive README files to all major folders

feat(macros): add support for custom derive attributes

This commit adds support for deriving custom attributes on types
that use the shorthand macro, enabling better integration with
other proc-macro crates.

Fixes #42
```

### Commit Practices
- Commit frequently with focused changes
- Each commit should be a logical unit of work
- Run tests before committing: `cargo test`
- Run formatting: `cargo fmt`
- Run linter: `cargo clippy`
- Use `git commit --amend` for fixing the previous commit
- Squash related commits when appropriate
- Never commit secrets, credentials, or sensitive data
- Write meaningful commit messages that explain WHY, not just WHAT

### Branch Naming
- Feature branches: `feature/description` or `feat/description`
- Bug fixes: `fix/description` or `bug/description`
- Documentation: `docs/description`
- Use kebab-case for branch names
- Keep branch names concise but descriptive

**Examples:**
```
feat/shorthand-macro
fix/refcell-borrow-panic
docs/api-documentation
chore/update-dependencies
```

### Pull Requests

**Creating PRs:**
- Create PRs for all changes (even small ones)
- Use descriptive PR titles following conventional commit format
- Include clear description of changes in PR body
- Use PR templates if available
- Reference related issues (e.g., "Closes #123")
- Add labels to categorize the PR
- Request reviews from appropriate maintainers

**PR Description Template:**
```markdown
## Summary
Brief description of what this PR does.

## Changes
- List of specific changes
- Another change
- Yet another change

## Testing
How this was tested, what tests were added/modified.

## Related Issues
Closes #123
Relates to #456
```

**Before Submitting:**
- Ensure all tests pass: `cargo test`
- Ensure code is formatted: `cargo fmt`
- Ensure no clippy warnings: `cargo clippy`
- Ensure CI passes on GitHub
- Review your own changes first

**Merging:**
- Use squash merge for clean history
- Ensure commit message follows conventional commits
- Delete branch after merging

### GitHub-Specific Conventions

**Issue References:**
- Use keywords to auto-close issues: `Fixes #123`, `Closes #123`, `Resolves #123`
- Reference related issues: `Relates to #123`, `See #123`
- Link to issues in commit messages and PR descriptions

**Labels:**
- Use labels to categorize issues and PRs
- Common labels: `bug`, `enhancement`, `documentation`, `good first issue`

**Status Checks:**
- All CI checks must pass before merging
- Address any failing tests or linting issues
- Fix formatting issues with `cargo fmt`