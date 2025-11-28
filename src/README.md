# hurry/src

This directory contains the core implementation of the `hurry` crate's declarative macros.

## Overview

The `lib.rs` file defines all the convenience macros for creating common Rust pointer and wrapper types. These macros reduce boilerplate and make code more concise when working with heap-allocated, reference-counted, or interior-mutable types.

## Structure

- `lib.rs` - Main library file containing all macro definitions and tests

## Macro Categories

### Basic Pointer Types
- `boxx!` - Creates `Box<T>` (heap-allocated, single-owned)
- `rc!` - Creates `Rc<T>` (reference-counted, single-threaded)
- `arc!` - Creates `Arc<T>` (atomic reference-counted, thread-safe)
- `pin_box!` - Creates `Pin<Box<T>>` (pinned heap allocation)

### Interior Mutability
- `cell!` - Creates `Cell<T>` (for Copy types)
- `refcell!` - Creates `RefCell<T>` (runtime borrow checking)
- `mutex!` - Creates `Mutex<T>` (thread-safe mutual exclusion)
- `rwlock!` - Creates `RwLock<T>` (thread-safe read-write lock)

### Combined Types
- `rc_refcell!` - Creates `Rc<RefCell<T>>` (single-threaded shared mutable state)
- `arc_mutex!` - Creates `Arc<Mutex<T>>` (thread-safe shared mutable state)
- `arc_rwlock!` - Creates `Arc<RwLock<T>>` (thread-safe shared read-write state)

### Collection Helpers
- `vec_box!` - Creates `Vec<Box<T>>` from multiple values
- `vec_rc!` - Creates `Vec<Rc<T>>` from multiple values
- `vec_arc!` - Creates `Vec<Arc<T>>` from multiple values

### Copy-on-Write
- `cow_owned!` - Creates `Cow::Owned<T>`
- `cow_borrowed!` - Creates `Cow::Borrowed<T>`

## Implementation Details

All macros are declarative (macro_rules!) and are exported for use in downstream crates. They use fully-qualified paths (e.g., `std::rc::Rc::new`) to avoid import conflicts.

## Testing

The `tests` module at the bottom of `lib.rs` contains comprehensive unit tests for each macro, verifying:
- Correct type creation
- Value initialization
- Mutability semantics
- Clone behavior (where applicable)

To run tests:
```bash
cargo test
```

## Documentation

Each macro includes rustdoc comments with:
- Purpose description
- Usage examples
- Type signatures (implied by usage)

Generate documentation:
```bash
cargo doc --open
```
