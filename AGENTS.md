# Agent Instructions for hurry

## Build Commands
- `cargo build` - Build the project
- `cargo build --release` - Build optimized release version

## Test Commands
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run a specific test (e.g., `cargo test test_boxx_macro`)

## Lint and Format
- `cargo clippy` - Run linter
- `cargo fmt` - Format code

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