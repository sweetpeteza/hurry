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

## Git Conventions

### Commit Messages
- Use conventional commit format: `type(scope): description`
- Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`
- Keep first line under 50 characters
- Use imperative mood ("Add feature" not "Added feature")
- Add detailed description for complex changes

### Examples
```
feat: add boxx macro for Box creation
fix: correct pin_box double dereference in tests
docs: update README with new macro examples
refactor: rename box_ to boxx for cleaner API
test: add comprehensive test coverage for all macros
```

### Commit Practices
- Commit frequently with focused changes
- Each commit should be a logical unit of work
- Run tests before committing
- Use `git commit --amend` for fixing the previous commit
- Squash related commits when appropriate
- Never commit secrets, credentials, or sensitive data

### Branch Naming
- Feature branches: `feature/description` or `feat/description`
- Bug fixes: `fix/description` or `bug/description`
- Documentation: `docs/description`
- Use kebab-case for branch names

### Pull Requests
- Create PRs for all changes
- Include clear description of changes
- Reference related issues
- Ensure CI passes before merging
- Use squash merge for clean history