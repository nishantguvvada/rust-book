# CHAPTER 1

## The `main` Function

- The `main` function is **special**: it is always the first code that runs in every executable Rust program.
- `println!` calls a **Rust macro** (macros generate code to extend Rust syntax).
- The `!` symbol means you are calling a **macro** instead of a normal function.

---

## Compiling vs Running

- **Compiling and running are separate steps.**
- Before running a Rust program, you must compile it using the Rust compiler:
  ```bash
  rustc main.rs
  ```
- Rust outputs a **binary executable**, along with other files:

  - `main.rs` → Source code file
  - `main.exe` → Executable file (on Windows)
  - `main` → Executable file (on Linux/macOS)
  - `main.pdb` → Debugging information file (on Windows)

- Rust is an **ahead-of-time compiled language**.
  - You can compile a program once and give the executable to someone else and they can run it even without having Rust installed.

---

## Cargo

- Rust’s **build system and package manager**.
- Cargo handles:

  - Building code
  - Downloading libraries
  - Building dependencies (in Rust, libraries = dependencies)

- To create a new project:

  ```bash
  cargo new hello_cargo
  ```

- Cargo generates:

  - `Cargo.toml` file
  - `src` directory with `main.rs` file

- **TOML (Tom’s Obvious, Minimal Language)** example:

  ```toml
  [package]
  name = "hello_cargo"
  version = "0.1.0"
  edition = "2024"

  [dependencies]
  ```

- In Rust, packages of code are called **crates**.

---

## Building and Running a Cargo Project

- To build the project:

  ```bash
  cargo build
  ```

  - Creates an executable in `target/debug/main.exe` (Windows) or `target/debug/main` (Linux/macOS).
  - The default build is a **debug build**.

- Running `cargo build` for the first time also generates:

  - `Cargo.lock` → Tracks exact versions of dependencies.

- To check code quickly without producing an executable:
  ```bash
  cargo check
  ```

---

## Summary

- Create a project → `cargo new`
- Build a project → `cargo build`
- Build & run a project → `cargo run`
- Check for errors without building → `cargo check`
- Cargo stores build outputs in `target/debug` instead of the source directory.

# CHAPTER 2

This chapter covers how to accept user input, handle errors, use external crates, and build a simple guessing game.

---

## Getting User Input

- To obtain user input, use the **`io` library**.
- The `io` library comes from the **standard library (std)**:
  ```rust
  use std::io;
  ```

By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude.

If a type you want to use isn’t in the prelude, you must bring it into scope explicitly with a use statement.

Using the std::io library provides several useful features, including the ability to accept user input.

---

## Storing Values with Variables

- In Rust, variables are immutable by default, meaning once a value is assigned, it cannot be changed.
- The equal sign (`=`) tells Rust we want to bind something to the variable.
- `String` is a string type provided by the standard library.
- The `::` syntax in `String::new` indicates that `new` is an associated function of the `String` type (similar to methods of classes in Python).
- An associated function is a function implemented on a type.

---

## Receiving User Input

- The `stdin` function returns an instance of `std::io::Stdin`, which is a type representing a handle to standard input for the terminal.
- `.read_line(&mut guess)` calls the `read_line` method on the standard input handle.
- The full job of `read_line` is to take whatever the user types into standard input and append it into a string (without overwriting its contents).
- Therefore, we pass the string as an argument.
- The string argument must be **mutable** so the method can change the string’s content.
- Like variables, references are immutable by default. Hence, you need to write `&mut guess` rather than `&guess` to make it mutable.

---

## Handling Potential Failure with Result

- `Result` is an enumeration (enum), which is a type that can be in one of multiple possible states.
- Each possible state is called a variant.
- The purpose of `Result` is to encode error-handling information.
- `Result` has two variants:

  - **Ok** → operation was successful, contains the generated value.
  - **Err** → operation failed, contains information about the error.

- An instance of `Result` has an `expect` method:
  - If the instance is `Err`, `expect` will cause the program to crash and display the provided message.
  - If the instance is `Ok`, `expect` will extract the value inside `Ok` and return it.

---

## Printing Values with println!

- Curly brackets `{}` act as placeholders.
- When printing the value of a variable, the variable name goes inside the curly brackets.
- When printing the result of evaluating an expression:
  - Use `{}` as placeholders in the format string.
  - Follow the format string with a comma-separated list of expressions to print in order.

---

## Using a Crate to Get More Functionality

- A crate is a collection of Rust source code files.
- The project we’ve been building is a **binary crate** (executable).
- The **rand crate** is a **library crate**, meaning it contains code intended to be used in other programs and cannot be executed on its own.

- Cargo fetches the latest versions of all dependencies from the **registry** (a copy of data from Crates.io).
- Cargo checks the `[dependencies]` section of `Cargo.toml` and downloads any listed crates not already present.

### Ensuring Reproducible Builds with the Cargo.lock File

- When you build a project for the first time, Cargo figures out all versions of dependencies that fit your criteria and writes them to the **Cargo.lock** file.
- The Cargo.lock file locks the versions until explicitly upgraded.
- On future builds, Cargo uses the versions specified in Cargo.lock instead of re-resolving versions.

### Updating a Crate to Get a New Version

- Cargo provides the `update` command, which ignores the Cargo.lock file and figures out the latest versions that fit the Cargo.toml requirements.
- The `cargo doc --open` command builds and opens documentation provided by all dependencies locally.

---

## Generating a Random Number

- `use rand::Rng;` → brings the `Rng` trait into scope, which defines methods random number generators implement.
- `rand::thread_rng` → a generator local to the current thread of execution, seeded by the operating system.
- `.gen_range(start..=end)` → generates a random number in the given range (inclusive of both bounds).

---

## Comparing a Secret Number to the Guess

- A `match` expression is made up of **arms**.
- An arm consists of a pattern and the code that should run if the value matches the pattern.
- Example: comparing `50` to `38` using `.cmp()` returns `Ordering::Greater`. The match expression then checks arms until it finds `Ordering::Greater`.
- Rust defaults number types to `i32`.
- Rust allows shadowing a variable’s previous value with a new one.
- `.trim()` on a `String` removes whitespace at the beginning and end.
- `.parse()` converts a string to another type. Example:
  ```rust
  let guess: u32
  ```
  tells Rust the exact number type to parse into.

---

## Allowing Multiple Guesses with Looping

- The `loop` keyword creates an infinite loop.
- Ways to exit:
  - Press **CTRL + C**
  - Enter non-number input such as `"quit"`
  - Correct number guessed → game stops.

---

## Quitting After a Correct Guess

- Add a `break` statement to exit the game when the user wins.

---

## Handling Invalid Input

- Use `match` with `.parse()` to handle invalid input.
- If input cannot be parsed, use `Err(_) => continue` to skip and ask again instead of crashing.

---

# CHAPTER 3

- Variables, basic types, functions, comments, and control flows

---

## Variables and Mutability

- Variables are immutable by default.

#### Why Rust Encourages Immutability and When to Opt Out

When a variable is immutable, once a value is bound to the name of the variable, you can't change that value.

If one part of the code operates on the assumption that a value will never change and another part of the code changes that value, it's possible that the first part of the code won't do what it was designed to do. This kind of bug is difficult to track down, especially when the second part of the code changes the value only sometimes.

Mutability can be very useful:

- Variables are immutable by default, but you can make them mutable by adding `mut` in front of the variable name.

---

### Constants

- Constants are values bound to a name and not allowed to change.

#### Differences Between Constants and Immutable Variables:

1. Constants are ALWAYS immutable (you cannot use `mut` with constants).
2. Declared using the `const` keyword instead of `let`, and must always annotate the type.
3. Constants may only be set to a constant expression, not the result of a value computed at runtime.

Example:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

- Rust's naming convention for constants: use all uppercase with underscores between words.
- Constants are valid for the entire time a program runs within the scope in which they were declared.
- Constants are useful for naming hardcoded values used throughout the program.

---

### Shadowing

You can declare a new variable with the same name as a previous variable. The first variable is "shadowed" by the second.  
The second variable is what the compiler sees when you use the variable's name.

Shadowing is different from marking a variable as `mut`. If you try to reassign without `let`, you'll get a compile-time error. Using `let` allows you to perform transformations on a value while keeping the variable immutable after those transformations.

```rust

let x = 5;
let x = x + 1;

{
  let x = x * 2;
  println!("The value of x in the inner scope is: {x}");
}

println!("The value of x is: {x}");

let spaces = "    "; // string type
let spaces = spaces.len(); // number type

// Using mut instead of let
let mut spaces = "    ";
spaces = spaces.len(); // compile-time error: not allowed to mutate a variable's type
```

---

## Data Types

- Rust is a statically typed language, which means that it must know the types of all variables at compile time.
- The compiler usually infers what type we want to use based on the value and how we use it. In cases when many types are possible, we must add a type annotation.

### Scalar Types

- A scalar type represents a single value.
- Rust has 4 primary scalar types: integers, floating-point numbers, booleans, and characters.

#### Integer Types

An integer is a number without a fractional component.

| Length                 | Signed | Unsigned |
| ---------------------- | ------ | -------- |
| 8-bit                  | i8     | u8       |
| 16-bit                 | i16    | u16      |
| 32-bit                 | i32    | u32      |
| 64-bit                 | i64    | u64      |
| 128-bit                | i128   | u128     |
| Architecture dependent | isize  | usize    |

- An integer can be 'Signed' or 'Unsigned'. Signed and unsigned refer to whether the number can be negative. Signed numbers include a sign, while unsigned numbers are always positive.
- Signed numbers are stored using two’s complement representation.
- Each signed variant can store numbers from `-(2^(n-1))` to `2^(n-1) - 1` inclusive, where `n` is the number of bits.

Example of 8-bit numbers:

```
_ _ _ _ _ _ _ _ = 8 bits
0    = 00000000
127  = 01111111
-128 = 10000000
43   = 00101011
```

- Unsigned variants store numbers from `0` to `2^n - 1`.
- `isize` and `usize` types depend on the computer architecture your program runs on.
- Number literals can use `_` as a visual separator: `1_000` is the same as `1000`.
- Integer types default to `i32`.
- On overflow, Rust performs two’s complement wrapping:
  - For example, in `u8`, 256 becomes 0, 257 becomes 1, etc.

To explicitly handle overflow, you can use:

- `wrapping_*` methods (e.g., `wrapping_add`) – wrap around.
- `checked_*` methods – return `None` on overflow.
- `overflowing_*` methods – return value and a Boolean overflow flag.
- `saturating_*` methods – clamp to min or max value.

#### Floating-Point Types

- Rust has `f32` and `f64` types for floating-point numbers (32 and 64 bits, respectively).
- All floating-point types are signed.

#### Numeric Operations

- Rust supports addition, subtraction, multiplication, division, and remainder operations.
- Integer division truncates toward zero to the nearest integer.

#### Boolean Types

- Boolean type has two values: `true` and `false`.
- Booleans are 1 byte (8 bits) in size.
- Declared with the `bool` keyword.

#### Character Types

- `char` literals are specified with single quotes (e.g., `'a'`), whereas strings use double quotes (e.g., `"hello"`).
- Rust’s `char` type is 4 bytes and represents a Unicode scalar value, meaning it can represent far more than ASCII characters.

### Compound Types

- Compound types can group multiple values into one type.
- Rust has 2 primitive compound types: **tuples** and **arrays**.

---

#### Tuple Type

- A general way of grouping together a number of values with a variety of types into one compound type.
- Tuples have a **fixed length**; they cannot grow or shrink in size.
- A tuple is created by writing a **comma-separated list of values inside parentheses**.  
  The types of different values in the tuple don't have to be the same.
- We can use **pattern matching** to destructure a tuple value.
- **Destructuring** refers to extracting the values of the tuple by breaking the single tuple into different parts.
- You can access a tuple element directly by using a **period followed by the index** of the value.
- A tuple without any values is called a **unit**.  
  The unit represents an empty value or empty return type.  
  _(Expressions implicitly return the unit value if they don't return any other value.)_

---

#### Array Type

- Another way to have a collection of multiple values.  
  **Every element of an array must have the same type.**
- Arrays in Rust have a **fixed length**.
- An array is created by writing a **comma-separated list of values inside square brackets**.
- Arrays are useful when:
  - You want the data to be allocated on the **stack** rather than the heap.
  - You want to ensure you always have a **fixed number of elements**.
- A **vector** is a similar collection type provided by the standard library that is allowed to grow or shrink in size because its contents live on the **heap**.
- You can initialize an array to contain the **same value** for each element by specifying:
  - The initial value
  - Followed by a semicolon
  - Then the length of the array inside square brackets.
- You can access elements of an array using **indexing**.

---
