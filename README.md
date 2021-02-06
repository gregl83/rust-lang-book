# rust-lang-book

Rust Language Book Chapter Work

## Introduction

Rust programming language is designed for low-level operations requiring low power consumption, low memory usage, and high performance.

There is no "golden hammer" with software systems but Rust fills the need for a modern programming language requiring a very high level of control. 

## Requirements

- rustc 1.45.2 (d3fb005a3 2020-07-31)
- mdbook v0.4.6 or greater (for convenience)

## Workspace

This project is dependent on the cargo build system and leverages workspaces to package chapter work respectively.  

Not every chapter in the Rust Book has transferable source code but for the chapters that do a package is included in the root directory of this repository.

## Structure

A Rust Book chapter package has been created in this repository workspace for chapters including source code examples.

Path Convention: `rust-lang-book/<chapter-uri-path>`

The Rust Language Book itself is included as a GIT submodule. This tethers chapter work to a specific verison of the book.

*Hint: use mdbook for stable references even if outdated*

## References

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust Standard Library](https://doc.rust-lang.org/std/)
- [Cargo](https://doc.rust-lang.org/cargo/)

## License
MIT