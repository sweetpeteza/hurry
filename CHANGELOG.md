# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- Corrected variable names in README code examples (mutex, rwlock)
- Fixed documentation typo: `#[Shorthand]` -> `#[shorthand]`

### Changed
- Removed `hurry-macros/Cargo.lock` from version control
- Improved `.gitignore` to exclude IDE files and nested Cargo.lock files

### Added
- Comprehensive CI workflow for pull requests and main branch
- This CHANGELOG file

## [0.1.3] - 2024-11-07

### Changed
- Updated version to 0.1.3
- Updated documentation

### Fixed
- Resolved merge conflict in Cargo.lock

## [0.1.2] - 2024-11-07

### Fixed
- Corrected `box` macro usage to `boxx` in README example

### Changed
- Updated Cargo.lock for version 0.1.2

## [0.1.1] - 2024-11-07

### Added
- GitHub workflow for automated releases
- MIT and Apache-2.0 license files
- Repository metadata to Cargo.toml files

### Changed
- Made procedural macros opt-in via `macros` feature flag
- Updated workflow to trigger on release creation

## [0.1.0] - 2024-11-07

### Added
- Initial release with declarative macros for common pointer types:
  - `boxx!` for `Box<T>`
  - `rc!` for `Rc<T>`
  - `arc!` for `Arc<T>`
  - `rc_refcell!` for `Rc<RefCell<T>>`
  - `arc_mutex!` for `Arc<Mutex<T>>`
  - `arc_rwlock!` for `Arc<RwLock<T>>`
  - `mutex!` for `Mutex<T>`
  - `rwlock!` for `RwLock<T>`
  - `cell!` for `Cell<T>`
  - `refcell!` for `RefCell<T>`
  - `pin_box!` for `Pin<Box<T>>`
  - `vec_box!`, `vec_rc!`, `vec_arc!` for vectors of wrapped types
  - `cow_owned!` and `cow_borrowed!` for `Cow<T>`
- Procedural `#[shorthand]` attribute macro for custom types
- Comprehensive test coverage
- Documentation with examples

[Unreleased]: https://github.com/sweetpeteza/hurry/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/sweetpeteza/hurry/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/sweetpeteza/hurry/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/sweetpeteza/hurry/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/sweetpeteza/hurry/releases/tag/v0.1.0
