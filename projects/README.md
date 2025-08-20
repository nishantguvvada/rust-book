main function is special: it is always the first code that runs in every executable Rust program
println! calls a Rust macro (Rust macros are a way to write code that generates code to extend Rust syntax)
! means calling a macro instead of a normal function

# Compiling and Running are separate steps

Before running a Rust program, you must compile it using the Rust compiler by entering the `rustc main.rs` command.

Rust outputs a binary executable:

- main.exe
- main.pdb
- main.rs

This shows the source code file with the .rs extension, the executable file (main.exe on Windows, but main on all other platforms), and, when using Windows, a file containing debugging information with the .pdb extension.

Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.

# Cargo

Rust's build system and package manager. Cargo handles building code, downloading libraries and building those libraries (in Rust, libraries = dependencies)

The below command creates a new project, Cargo generates 2 files and 1 directory: a Cargo.toml file with src directory with main.rs file

```
cargo new hello_cargo
```

TOML : (Tom’s Obvious, Minimal Language)

[package] - section heading
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies] - project dependencies

In Rust, packages of code are referred to as crates

# Building and running a cargo project

To build the project: cargo build -> creates an executable file in target/debug/main.exe. The default build is a debug build, Cargo puts the binary in a directory named debug.

Running cargo build for the first time creates a new file at the top level: Cargo.lock (this file keeps a track of the exact versions of dependencies in the project)

cargo check - quickly checks your code to make sure it compiles but doesn’t produce an executable

# SUMMARY

We can create a project using cargo new.
We can build a project using cargo build.
We can build and run a project in one step using cargo run.
We can build a project without producing a binary to check for errors using cargo check.
Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.
