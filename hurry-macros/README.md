# hurry-macros

This crate provides procedural macros for the `hurry` library, enabling automatic generation of shorthand macros for custom types.

## Overview

The `hurry-macros` crate is a proc-macro crate that extends the `hurry` library with code generation capabilities. It allows developers to automatically create ergonomic macro shortcuts for their own types.

## Structure

```
hurry-macros/
├── src/
│   └── lib.rs          # Procedural macro implementations
├── tests/
│   └── shorthand.rs    # Integration tests for macros
├── Cargo.toml          # Package configuration
└── README.md           # This file
```

## Macros

### `#[shorthand]`

The `shorthand` attribute macro generates a convenience macro for types that have a `new` constructor method.

**Usage:**
```rust
use hurry_macros::shorthand;

struct MyType {
    value: i32,
}

#[shorthand]
impl MyType {
    pub fn new(value: i32) -> Self {
        MyType { value }
    }
}

// Generated macro can be used like:
let instance = my_type!(42);
```

**What it does:**
1. Analyzes the `impl` block for a named type
2. Verifies the presence of a `new` method
3. Generates a snake_case macro name from the type name
4. Creates a declarative macro that forwards arguments to `Type::new()`

**Requirements:**
- Must be applied to an `impl` block
- The impl block must be for a named type (not a generic or trait type)
- The impl block must contain a method named `new`

**Generated Output:**
The macro generates a `#[macro_export]` declarative macro that can be used throughout your codebase.

## Dependencies

- `proc-macro2` - Core proc macro utilities
- `quote` - Code generation
- `syn` - Rust syntax parsing
- `convert_case` - Name conversion (PascalCase to snake_case)

## Testing

Integration tests are located in `tests/shorthand.rs` and verify:
- Basic macro generation
- Correct forwarding of arguments
- Type inference
- Error cases

Run tests:
```bash
cd hurry-macros
cargo test
```

## Development

### Adding New Macros

When adding new procedural macros:
1. Add the function to `src/lib.rs`
2. Mark it with `#[proc_macro]`, `#[proc_macro_attribute]`, or `#[proc_macro_derive]`
3. Add comprehensive tests in the `tests/` directory
4. Document the macro with rustdoc examples

### Code Style

Follow the project's style guidelines:
- Use `syn` for parsing
- Use `quote!` for code generation
- Provide clear error messages with `syn::Error`
- Include usage examples in documentation

## Building

As a proc-macro crate, this must be built as a dynamic library:

```bash
cargo build
```

The compiled proc-macro is used at compile-time by crates that depend on it.

## Usage in hurry

This crate is an optional dependency of the main `hurry` crate, enabled via the `macros` feature:

```toml
[dependencies]
hurry = { version = "0.1", features = ["macros"] }
```
