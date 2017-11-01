## Synopsis

Year 2015 is not (yet) finished. This is implemented as one big Rust project, so compilation takes a while.

## Installation

You should use Cargo for building the project or running tests.

Build:

	cargo build --release

Or just run

	cargo run --release

## Tests

Tests are included in main.rs. Tests check that all implemented daily puzzle solvers return expected results. Run tests with 

	cargo test --release

You should use release build, since debug build is quite slow on some tasks.

