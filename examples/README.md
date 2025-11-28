# Examples

This directory contains example programs demonstrating the usage of the `hurry` crate's macros.

## Running Examples

To run an example:
```bash
cargo run --example <example_name>
```

For instance:
```bash
cargo run --example basic_pointers
```

## Available Examples

### basic_pointers.rs

Demonstrates the fundamental pointer wrapper macros provided by `hurry`.

**Covers:**
- `boxx!` - Creating `Box<T>` for heap allocation
- `rc!` - Creating `Rc<T>` for reference-counted shared ownership
- `arc!` - Creating `Arc<T>` for thread-safe shared ownership
- `pin_box!` - Creating `Pin<Box<T>>` for pinned allocations
- `vec_box!` - Creating vectors of boxed values

**Use cases:**
- When to use `Box` vs `Rc` vs `Arc`
- Checking reference counts
- Creating collections of heap-allocated values

**Run:**
```bash
cargo run --example basic_pointers
```

### cell_types.rs

Demonstrates interior mutability patterns using `Cell` and `RefCell`.

**Covers:**
- `cell!` - Creating `Cell<T>` for Copy types with interior mutability
- `refcell!` - Creating `RefCell<T>` for runtime borrow checking
- `cow_borrowed!` - Creating borrowed `Cow<T>`
- `cow_owned!` - Creating owned `Cow<T>`

**Use cases:**
- Interior mutability without `mut` keyword
- Runtime borrow checking with `RefCell`
- Copy-on-write semantics with `Cow`

**Run:**
```bash
cargo run --example cell_types
```

### interior_mutability.rs

Demonstrates advanced interior mutability patterns for both single-threaded and multi-threaded scenarios.

**Covers:**
- `rc_refcell!` - Creating `Rc<RefCell<T>>` for shared mutable state (single-threaded)
- `arc_mutex!` - Creating `Arc<Mutex<T>>` for shared mutable state (thread-safe)
- `arc_rwlock!` - Creating `Arc<RwLock<T>>` for concurrent read-write access

**Use cases:**
- Shared mutable state within a single thread
- Thread-safe shared mutable state
- Concurrent readers with exclusive writer pattern
- Multi-threaded counter implementations

**Run:**
```bash
cargo run --example interior_mutability
```

## Example Structure

Each example follows a similar pattern:
1. Import necessary macros from `hurry`
2. Demonstrate basic usage of relevant macros
3. Show practical use cases
4. Print results to demonstrate functionality

## Learning Path

We recommend exploring the examples in this order:

1. **basic_pointers.rs** - Start here to understand the fundamental pointer types
2. **cell_types.rs** - Learn about interior mutability for single-threaded code
3. **interior_mutability.rs** - Explore thread-safe patterns and advanced usage

## Adding New Examples

When adding new examples:
1. Create a new `.rs` file in this directory
2. Add a descriptive header comment explaining what the example demonstrates
3. Import only the macros you need
4. Include `println!` statements to show the output
5. Update this README with a description of the new example
6. Test that the example compiles and runs:
   ```bash
   cargo run --example your_example_name
   ```

## Common Patterns

### Error Handling
Examples use `.unwrap()` for simplicity. Production code should use proper error handling.

### Thread Safety
- Use `Rc` and `RefCell` for single-threaded code
- Use `Arc`, `Mutex`, and `RwLock` for multi-threaded code

### Performance
- `Box` has minimal overhead
- `Rc`/`Arc` add reference counting overhead
- `Mutex`/`RwLock` add synchronization overhead
- `Cell`/`RefCell` add runtime borrow checking (RefCell only)

## Additional Resources

- [Main README](../README.md) - Project overview and installation
- [hurry API docs](https://docs.rs/hurry) - Complete API documentation
- [The Rust Book - Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
