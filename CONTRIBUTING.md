# Contributing to hurry

Thank you for your interest in contributing to hurry! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

Be respectful, constructive, and professional in all interactions.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/hurry.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test your changes
6. Submit a pull request

## Development Setup

### Prerequisites

- Rust 1.56 or later
- Cargo

### Building the Project

```bash
cargo build
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with all features
cargo test --all-features

# Run tests for hurry-macros
cd hurry-macros && cargo test
```

## Code Style Guidelines

### Naming Conventions

- **Macros**: snake_case, use alternative names if conflicts with keywords (e.g., `boxx` instead of `box_`)
- **Functions**: snake_case
- **Types**: PascalCase

### Documentation

- Use `///` for public API documentation with examples
- Include `# Example` sections in macro docs
- All public APIs must be documented

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
- Ensure all new features have corresponding tests

### Formatting

- Run `cargo fmt` before committing
- Use 4 spaces for indentation (Rust default)
- Follow the output of `cargo fmt`

## Making Changes

### Before You Submit

1. **Run the linter**: `cargo clippy -- -D warnings`
2. **Format your code**: `cargo fmt`
3. **Run all tests**: `cargo test --all-features`
4. **Build documentation**: `cargo doc --no-deps`
5. **Update CHANGELOG.md**: Add your changes under the `[Unreleased]` section

### Commit Message Format

Use conventional commit format:

```
type(scope): description

[optional body]
```

**Types:**
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Formatting, missing semicolons, etc.
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `test`: Adding missing tests
- `chore`: Changes to build process or auxiliary tools

**Examples:**
```
feat: add boxx macro for Box creation
fix: correct pin_box double dereference in tests
docs: update README with new macro examples
refactor: rename box_ to boxx for cleaner API
test: add comprehensive test coverage for all macros
```

**Guidelines:**
- Keep first line under 50 characters
- Use imperative mood ("Add feature" not "Added feature")
- Add detailed description for complex changes

## Pull Request Process

1. **Update Documentation**: Ensure README.md and doc comments reflect your changes
2. **Update CHANGELOG.md**: Add your changes under `[Unreleased]`
3. **Pass CI Checks**: Ensure all CI checks pass (tests, clippy, fmt, docs)
4. **Clear Description**: Provide a clear description of what your PR does and why
5. **Link Issues**: Reference any related issues in your PR description

### PR Checklist

- [ ] Code follows project style guidelines
- [ ] All tests pass locally
- [ ] New tests added for new features
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Commit messages follow conventional format
- [ ] No compiler warnings
- [ ] `cargo clippy` passes with no warnings
- [ ] `cargo fmt` has been run

## Adding New Macros

When adding new declarative macros to `src/lib.rs`:

1. Add the macro implementation
2. Add documentation with examples
3. Add unit tests in the `tests` module
4. Update README.md with the new macro
5. Update CHANGELOG.md

Example:

```rust
/// Macro for creating a `NewType<T>` from a value
///
/// # Example
/// ```
/// use hurry::new_type;
/// let x = new_type!(5);
/// assert_eq!(*x, 5);
/// ```
#[macro_export]
macro_rules! new_type {
    ($val:expr) => {
        NewType::new($val)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_type_macro() {
        let x = new_type!(42);
        assert_eq!(*x, 42);
    }
}
```

## Adding Procedural Macros

When adding procedural macros to `hurry-macros`:

1. Implement in `hurry-macros/src/lib.rs`
2. Add comprehensive documentation
3. Add tests in `hurry-macros/tests/`
4. Update README.md
5. Update CHANGELOG.md

## Questions?

Feel free to open an issue for:
- Bug reports
- Feature requests
- Questions about the codebase
- Clarification on contribution guidelines

## License

By contributing to hurry, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).
