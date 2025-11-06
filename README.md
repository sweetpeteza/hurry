# hurry

Convenient macros for creating pointer types (`Box`, `Rc`, `Arc`, etc.) in Rust.

[![Crates.io](https://img.shields.io/crates/v/hurry.svg)](https://crates.io/crates/hurry)
[![Documentation](https://docs.rs/hurry/badge.svg)](https://docs.rs/hurry)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/hurry.svg)](#license)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
hurry = "0.1"
```

## Usage

The `hurry` crate provides convenient macros for creating common pointer types:

### Basic Pointer Types

```rust
use hurry::*;

// Create a Box
let boxed = box!(42);
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

## Available Macros

- `boxx!(value)` → `Box::new(value)`
- `rc!(value)` → `Rc::new(value)`
- `arc!(value)` → `Arc::new(value)`
- `rc_refcell!(value)` → `Rc<RefCell<T>>::new(RefCell::new(value))`
- `arc_mutex!(value)` → `Arc<Mutex<T>>::new(Mutex::new(value))`
- `arc_rwlock!(value)` → `Arc<RwLock<T>>::new(RwLock::new(value))`

## Documentation

Full API documentation is available at [docs.rs/hurry](https://docs.rs/hurry).

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.