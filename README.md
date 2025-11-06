# hurry

Convenient macros for creating pointer types (`Box`, `Rc`, `Arc`, etc.) in Rust.

[![Crates.io](https://img.shields.io/crates/v/hurry.svg)](https://crates.io/crates/hurry)
[![Documentation](https://docs.rs/hurry/badge.svg)](https://docs.rs/hurry)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/hurry.svg)](#license)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
hurry = "0.1.3"
```

To use the procedural macro for generating custom shorthand macros, enable the `macros` feature:

```toml
[dependencies]
hurry = { version = "0.1.3", features = ["macros"] }
```

## Usage

The `hurry` crate provides convenient macros for creating common pointer types:

### Basic Pointer Types

```rust
use hurry::*;

// Create a Box
let boxed = boxx!(42);
assert_eq!(*boxed, 42);

// Create an Rc
let shared = rc!(String::from("hello"));
let clone = shared.clone();
assert_eq!(*shared, "hello");

// Create an Arc
let atomic = arc!(vec![1, 2, 3]);
let thread_clone = atomic.clone();
// Can be safely shared across threads
```

### Interior Mutability Types

```rust
use hurry::*;

// Rc<RefCell<T>> for single-threaded interior mutability
let cell = rc_refcell!(0);
*cell.borrow_mut() += 10;
assert_eq!(*cell.borrow(), 10);

// Arc<Mutex<T>> for thread-safe interior mutability
let mutex = arc_mutex!(0);
*x.lock().unwrap() += 20;
assert_eq!(*x.lock().unwrap(), 20);

// Arc<RwLock<T>> for thread-safe read-write access
let rwlock = arc_rwlock!(0);
*x.write().unwrap() += 30;
assert_eq!(*x.read().unwrap(), 30);
```

### Procedural Macros

The `hurry` crate also provides a procedural macro for generating shorthand macros for your own types:

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

// This generates a `my_type!` macro for creating instances
let instance = my_type!(42);
assert_eq!(instance.value, 42);
```

The `#[shorthand]` attribute generates a macro named after the type in snake_case that calls the `new` method.

## Available Macros

### Declarative Macros

- `boxx!(value)` → `Box::new(value)`
- `rc!(value)` → `Rc::new(value)`
- `arc!(value)` → `Arc::new(value)`
- `rc_refcell!(value)` → `Rc<RefCell<T>>::new(RefCell::new(value))`
- `arc_mutex!(value)` → `Arc<Mutex<T>>::new(Mutex::new(value))`
- `arc_rwlock!(value)` → `Arc<RwLock<T>>::new(RwLock::new(value))`
- `mutex!(value)` → `Mutex::new(value)`
- `rwlock!(value)` → `RwLock::new(value)`
- `cell!(value)` → `Cell::new(value)`
- `refcell!(value)` → `RefCell::new(value)`
- `pin_box!(value)` → `Box::pin(value)`
- `vec_box!(values...)` → `vec![Box::new(value), ...]`
- `vec_rc!(values...)` → `vec![Rc::new(value), ...]`
- `vec_arc!(values...)` → `vec![Arc::new(value), ...]`
- `cow_owned!(value)` → `Cow::Owned(value)`
- `cow_borrowed!(value)` → `Cow::Borrowed(value)`

### Procedural Macros

- `#[shorthand]` → Generates a shorthand macro for types with a `new` method

## Documentation

Full API documentation is available at [docs.rs/hurry](https://docs.rs/hurry).

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.