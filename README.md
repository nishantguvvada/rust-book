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

## Functions

- **main()**: entry point of programs.
- The `fn` keyword allows declaring new functions.
- Rust uses **snake_case** as the style for functions and variables.
- Functions are defined by using the `fn` keyword followed by the function name and a set of parentheses. The curly brackets denote the body of the function.
- Functions can be called by mentioning the name followed by a set of parentheses.
- Rust does not care where you define your functions; they must be defined somewhere in a scope.
- The lines execute in the order in which they appear in `main()`.

### Parameters

- Parameters are special variables that are part of a function's signature.
- You can provide concrete values for functions with parameters, called **arguments**.
- In function signatures, you must declare the type of each parameter.
- Rust deliberately requires type annotations in function definitions. This helps the compiler provide better error messages and reduces the need for type annotations elsewhere.

### Statements and Expressions

- Functions are made up of a series of **statements** optionally ending in an **expression**.
- **Statements** are instructions that perform some action and do not return a value.

  - Example: `let x = 5;` is a statement.
  - Function definitions are also statements.

- Statements do not return values, so you cannot assign a statement to another variable. For example:

  ```rust
  let x = (let y = 5); // Invalid
  ```

- In other languages, assignment often returns a value (e.g., `x = y = 6`). Rust does not follow this convention.
- **Expressions** evaluate to a resultant value.

  - Example: `6` in `let y = 6;` is an expression.

- Calling a function or macro is an expression.
- A scope block created with curly brackets is also an expression.
- Expressions do not include ending semicolons. Adding a semicolon turns an expression into a statement.

### Functions with Return Values

- We must declare the return type after an arrow (`->`).
- In Rust, the return value of the function is the value of the final expression in the function body.
- Functions return the last expression implicitly, so you usually don't need the `return` keyword unless you want to return early.

---

## Comments

- Programmers leave comments in their source code that the compiler will ignore to make their code easy to understand, and sometimes extra explanation is warranted.
- idiomatic comment style starts a comment with two slashes

---

## Control Flow

- Ability to run some code depending on whether a condition is true and to run some code repeatedly while a condition is true.
- Most common constructs of Rust that let you control the flow of execution: `if` expressions and loops.

### if Expressions

- Allows branching of code depending on conditions.
- Start with the keyword `if`, followed by a condition.
- We place the block of code to execute if the condition is true immediately after the condition inside curly brackets. Blocks of code associated with conditions in `if` expressions are sometimes called **arms**.
- If you don’t provide an `else` expression and the condition is false, the program will skip the `if` block and move on to the next bit of code.
- The condition in the `if` expression must be a `bool`. We get an error if the condition isn't a `bool`.
- You must be explicit and always provide `if` with a Boolean as its condition.

#### Handling Multiple Conditions with `else if`

- You can use multiple conditions by combining `if` and `else` in an `else if` expression.
- The program checks each `if` expression and executes the first body for which the condition evaluates to `true`.
- Rust only executes the block for the first `true` condition, and once it finds one, it doesn’t check the rest.

#### Using `if` in a `let` Statement

- Because `if` is an expression, we can assign the outcome to the right of the `let` statement.
- The variable will be bound to the value based on the outcome of the `if` expression.
- `if` and `else` arms must have the same value types.

### Repetition with Loops

- Loops enable the program to run through the code inside the loop body to the end and then start immediately back from the beginning.
- Rust has 3 kinds of loops: `loop`, `while`, and `for`.

#### Repeating Code with `loop`

- The `loop` keyword tells Rust to execute a block of code repeatedly forever or until explicitly stopped.
- You can place the `break` keyword within the loop to tell the program when to stop executing the loop.
- The `continue` keyword tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration.

#### Returning Values from Loops

- One of the uses of a loop is to retry an operation you know might fail.
- To pass the result of an operation out of the loop to the rest of your code, add the value you want returned after the `break` expression you use to stop the loop.
- You can also `return` from inside a loop. While `break` only exits the current loop, `return` always exits the current function.

#### Loop Labels to Disambiguate between Multiple Loops

- When you have loops within loops, `break` or `continue` keywords apply to the innermost loop.
- You can specify a loop label on a loop and use it with `break` or `continue` to specify that those keywords apply to the labeled loop instead of the innermost loop.

#### Conditional Loops with `while`

- A program will often need to evaluate a condition within a loop. While the condition is true, the loop runs. When the condition ceases to be true, the program calls `break`, stopping the loop.

#### Looping Through a Collection with `for`

- You can use a `for` loop to execute some code for each item in a collection.
- Machine code generated from `for` loops can be more efficient as well, because the index doesn’t need to be compared to the length of the array at every iteration.

# CHAPTER 4

## Ownership

- Rust's most unique feature
- Enables Rust to make memory safety guarantees without needing a garbage collector

### What is Ownership?

- Ownership is a set of rules that govern how Rust manages memory.
- Some languages have garbage collection that looks for no-longer-used memory as the program runs; in other languages, programmer must explicitly allocate and free the memory. In Rust, memory is managed through a system of ownership with a set of rules that the compiler checks.

#### Stack vs Heap

- Both stack and heap are parts of memory available to your code to use at runtime, but structured in different ways.
- The stack stores values in the order it gets them (last in, first out) and removes in opposite order.
- Adding data is called pushing onto the stack, removing data is called popping off the stack.
- All data stored on the stack must have known, fixed size.
- Data with an unknown size at compile time or a size that might change must be stored on the heap.
- Heap is less organized - when you put data on the heap, you request a certain amount of space.
- The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap.
- The pointer to the heap is a known, fixed size, you can store the pointer on the stack.
- Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.
- Accessing data in the heap is generally slower than accessing data on the stack because you have to follow a pointer to get there.
- When you code calls a function, the values passed into the function and the functional's local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

**The main purpose of ownership is to manage heap data**

#### Ownership Rules

- Each value has an owner.
- There can be only one owner at a time.
- When the owner goes out of scope, the value will be dropped.

#### Variable Scope

- Scope is the range within a program for which an item is valid
- The variables are valid from the point at which it's declared until the end of the current scope.
- `let s = "hello";` refers to a string literal, where the value of the string is hardcoded into the text of our program.
- The variable is valid from the point at which it's declared until the end of the current scope.

#### The String Type

- Variables of a known size, can be stored on the stack and popped off the stack when their scope is over, can be quickly copied to make a new, independent instance if another part of code needs to use the same value in a different scope.
- To explore how Rust knows when to clean up the data stored on the heap: String type is a great example
- String literals are hardcoded into our program and are immutable.
- String type manages data allocated on the heap and is able to store an amount of text that is unknown at compile time
- To create a String type from a String literal, use the `from` function: `let s = String::from("hello");`
- Variables are String type can be mutated but literals cannot. Why? - difference is in how these two types deal with memory. String literal contents are known at compile time, are hardcoded directly into the final executable, this is why String literals are fast and efficient.
- In order to store a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time i.e.
  - memory must be requested from the memory allocator at runtime (done by us when we call `String::from`)
  - we need a way to return this memory to the alloactor (GC/programmer/ownership)
- In Rust, memory is automatically returned once the variable that owns it goes out of scope. When a variable goes out of scope, Rust calls a special function `drop`

#### Variables and Data Interacting with Move

```
let x = 5; // bind the value 5 to x
let y = x; // make a copy of the value in x and bind it to y
```

- The second line makes a copy of the value in x and bind it to y.
- Integers are simple values with a known, fixed size. The two 5 values are pushed onto the stack.

```
let s1 = String::from("Hello");
let s2 = s1
```

- A String type variable is made up of 3 parts, a pointer to the memory, a length and a capacity. (stored on the stack). The memory on the heap allocated to the variable holds the contents of the variable.
- Length - how much memory in bytes the content is currently using.
- Capacity - total amount of memory in bytes the variable has received from the allocator.
- When we assign s1 to s2, String data is copied i.e. the pointer, the length and the capacity. We do not copy the data on the heap.
- When s2 and s1 go out of scope, they will both try to free the same memory. (double free error - freeing memory twice)
- To ensure memory safety, Rust considers s1 as no longer valid. Using s1 after s2 is created won't work. Rust prevents from using the invalidated reference.
- Shallow Copy = copying the pointer, length, and capacity without copying the data
- Rust invalidates the first variable along with a shallow copy, it's known as a move. (s1 was moved into s2)

#### Scope and Assignment

- We initially declare a variable s and bind it to a String with value "hello"
- We immediately create a new String with the value "ahoy" and assign it to s. (At this point, nothing is referring to the original value on the heap)
- The original string goes out of scope, Rust will run the drop function and it's memory will be freed.

#### Variables and Data Interacting with Clone

- To deeply copy the heap data of the String, not just the stack data, use `clone`.

```
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {s1}, s2 = {s2}");
```

#### Stack-Only Data: Copy

```
let x = 5;
let y = x;

println!("x = {x}, y = {y}");
```

- The above code is valid. (x is still valid and wasn't moved into y)
- The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y. In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying, and we can leave it out.
- Rust has a special annotation called `Copy` trait placed on types stored on the stack
- If your type doesn’t implement `Copy`, assignment or passing by value moves it and invalidates the source.
  If your type implements `Copy`, assignment copies it, keeping both variables valid.
- Rust won’t let us annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop` trait.

**How to add the Copy annotation to your type to implement the trait**: https://doc.rust-lang.org/book/appendix-03-derivable-traits.html

#### Ownership and Functions

- Passing a value to a function = assigning a value to a variable (Passing a variable to a function will move or copy just as assignment does)

#### Return Values and Scopes

- Returning values can also transfer ownership.

```
fn main() {
    let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

**What if we want to let a function use a value but not take ownership?**

- Rust let us return multiple values using a tuple

```
fn main() {
    let s1 = String::from("hello");

    let (s1, len) = calculate_length(s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

- This is a lot of work: Rust has a feature for using a value without transferring ownership: references

### References and Borrowing

- A reference is a pointer we can follow to access the data stored; the data is owned by some other variable.
- Unlike a pointer, a reference is guaranteed to point to a valid value for the life of the reference.
- Ampersands represent references, they allow you to refer to some value without taking ownership of it.

```
let s1 = String::from("hello");

let len = calculate_length(&s1);
```

- &s1 creates a reference that refers to the value of s1 but does not own it. The value will not be dropped when the reference stops being used.
- When functions have references as parameters instead of the actual values, we won’t need to return the values in order to give back ownership, because we never had ownership.

**Borrowing is the action of creating a reference**

What happens if we try to modify something we're borrowing?

- References are immutable by default.

#### Mutable References

```
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

- Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.

```
// This code attempts to create two mutable references to s, fails.
let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{r1}, {r2}");
```

- The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion.
- The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:

  - Two or more pointers access the same data at the same time.
  - At least one of the pointers is being used to write to the data.
  - There’s no mechanism being used to synchronize access to the data.

- We also cannot have a mutable reference while we have an immutable one to the same value.
- Users of an immutable reference don’t expect the value to suddenly change out from under them. However, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data.

#### Dangling References

- A pointer that references a location in memory that may have been given to someone else by freeing some memory while preserving a pointer to that memory.

```
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope and is dropped, so its memory goes away.
  // Danger!
```

- Because s is created inside dangle, when the code of dangle is finished, s will be deallocated. But we tried to return a reference to it. That means this reference would be pointing to an invalid String.

#### Rules of References

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

### The Slice Type

- Slices let you reference a contiguous sequence of elements in a collection. Slice is a reference, so it does not have ownership.
  (We don’t really have a way to talk about part of a string.)
- `iter` is a method that returns each element in a collection.
- `as_bytes` is a method that converts String to an array of bytes.

#### String Slices

- A string slice is a reference to a contiguous sequence of the elements of a String.
- Rather than a reference to the entire String, Slice is a reference to a portion of the String.
- We create slices using a range within brackets by specifying [starting_index..ending_index], where starting_index is the first position in the slice and ending_index is one more than the last position in the slice.
- Slice data structure stores the starting position and the length of the slice. So, in the case of let world = &s[6..11];, world would be a slice that contains a pointer to the byte at index 6 of s with a length value of 5.

```
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[3..];
let slice = &s[..];
```

- The type that signifies 'String Slice' is &str.

#### String Literals as Slices

- String literals are stored inside the binary.

```
let s = "Hello, world!";
```

- The type of s here is &str: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference.

#### String Slices as Parameters

```
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

#### Other Slices

```
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.

# CHAPTER 5

Using Structs to Structure Related Data

- struct or structure is a custom data type that lets you package and name multiple related values that make up a meaningful group.
- a struct is like an object's data attributes (in object-oriented language)

## Defining and Instantiating Structs

- Structs are similar to tuples (both hold multiple related values), like tuples, pieces of a struct can be different types.
- Unlike tuples, in a struct you'll name each piece of data: you don't have to rely on the order of the data to specify or access the values of an instance.

```
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64 // pieces of data of a struct = fields
}
```

- The struct definition is like a general template for the type, and instances fill in that template with particular data to create values of the type.

- To get a specific value from a struct, we use dot notation: `user1.email`
- To change the value in the email field of a mutable User instance: `user1.email = String::from("anotheremail@example.com");`

### Using the Field init shorthand

```
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // instead of using username: username, we only need to write username
        email,
        sign_in_count: 1,
    }
}
```

### Creating Instances from Other Instances with Struct Update Syntax

- Without update syntax:

```
let user2 = User {
  active: user1.active,
  username: user1.username,
  email: String::from("anotheremail@example.com"),
  sign_in_count: user1.sign_in_count
};
```

- With update syntax: `..` specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.

```
let user2 = User{
  email: String::from("anotheremail@example.com"),
  ..user1
}
```

- struct update syntax uses = like an assignment; this is because it moves the data. We can no longer use user1 after creating user2 because the String in the username field of user1 was (ownership) moved into user2.
- If we had given user2 new String values for both email and username, and thus only used the active and sign_in_count values from user1, then user1 would still be valid after creating user2. Both active and sign_in_count are types that implement the Copy trait

### Using Tuple Structs without Named Fields to create different types

- structs that look similar to tuples: tuple structs
- Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

- Each struct you define is its own type, even though the fields within the struct might have the same types.
- To destructure the values in the origin point into variables named x, y and z: `let Point(x, y, z) = origin;`

#### Unit-Like Structs without any Fields

- unit-like structs: structs that don't have any fields
- Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

```
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

## An Example Program using Structs

- Using single variables

```
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

- It is not clear that the parameters are related.

### Refactoring with Tuples

- Tuples add a bit of structure, typles don't name their elements.

```
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

### Refactoring with Structs

- Structs add meaning by labeling the data.

```
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

### Adding Useful Functionality with Derived Traits

- The println! macro can do many kinds of formatting, and by default, the curly brackets tell println! to use formatting known as Display: output intended for direct end user consumption.
- The primitive types we’ve seen so far implement Display by default because there’s only one way you’d want to show a 1 or any other primitive type to a user.
- But with structs, the way println! should format the output is less clear because there are more display possibilities: Do you want commas or not? Do you want to print the curly brackets? Should all the fields be shown? Due to this ambiguity, Rust doesn’t try to guess what we want, and structs don’t have a provided implementation of Display to use with println! and the {} placeholder.
- Putting the specifier `:?` inside the curly brackets tells println! we want to use an output format called Debug
- To add the functionality to print out debugging information for the struct, we add the outer attribute `#[derive(Debug)]`
- When we have larger structs, use `:#?`
- dbg! macro takes ownership of an experssion, prints the file and line number of where that dbg! macro call occurs and returns ownership of the value.

## Method Syntax

- Methods are similar to functions: declare them using `fn` keyword and a name.
- Unlike functions, methods are defined in the context of a struct (or an enum or a trait object).
- The first parameter of a method is always `self` which represents the instance of the struct.

### Defining Methods

- To define the function within the context of Rectangle, we start an impl (implementation) block for Rectangle. Everything within this impl block will be associated with the Rectangle type.
- The &self is actually short for self: &Self. Within an impl block, the type Self is an alias for the type that the impl block is for.
- If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter.
- Often, but not always, when we give a method the same name as a field we want it to only return the value in the field and do nothing else. Methods like this are called getters, and Rust does not implement them automatically for struct fields as some other languages do. Getters are useful because you can make the field private but the method public, and thus enable read-only access to that field as part of the type’s public API.

# CHAPTER 6

Enums and Pattern Matching

- Enums allow you to define a type by enumerating its possible variants
- Enums encode meaning along with the data
- Option enum expresses that a value can either be something or nothing
- Match expression makes it easy to run different code for different variants of the enum
- `if let` construct also handles enums

## Defining an Enum

- An enum value can only be one of its variants. They should be treated as the same type.

```
enum IpAddrKind {
  v4,
  v6
}
```

### Enum Values

- We can create instances of each of the variants. `let four = IpAddrKind::v4`
- We can put data directly into each enum variant.

```
enum IpAddr {
  v4(String),
  v6(String)
}

let home = IpAddr::v4(String::from("127.0.0.1"))
```

- The name of each enum variant becomes a function that constructs an instance of the enum.
- Each variant can have different types and amounts of associated data.
- The standard library defines IpAdd with address data embedded inside each variant in the form of structs

```
struct Ipv4Addr {

}

struct Ipv6Addr {

}

enum IpAddr {
  v4(Ipv4Addr)
  v6(Ipv6Addr)
}
```

- But if we used the different structs, each of which has its own type, we couldn’t as easily define a function to take any of the kinds.
- We can define methods on enums.

```
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
```

### Option Enum

- A null is a value that is currently invalid or absent for some reason.
- `Option<T>` : an enum that can encode the concept of a value being present or absent.

```
enum Option<T> {
  None,
  Some(T)
}
```

- When we have a Some value, we know that a value is present and the value is held within the Some. When we have a None value, in some sense it means the same thing as null: we don’t have a valid value.
- You have to convert an Option<T> to a T before you can perform T operations with it.

## The match Control Flow Construct

- `match` a control flow construct that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.
- Values go through each pattern in a match construct and at the first pattern the value fits, the value falls into the associated code block to be used during execution.
- We list the `match` keyword followed by an expression similar to a conditional expression used with `if`. With `if`, the condition needs to evaluate to a Boolean value but with `match`, it can be any type.
- An amr has 2 parts: a pattern and some code.
- `=>` operator separates the pattern and the code to run.
- When the `match` expression executes, it compares the resultant value against the pattern of each arm, in order. If a pattern matches the value, the code associated with that pattern is executed. If that pattern doesn’t match the value, execution continues to the next arm.
- The code associated with each arm is an expression, and the resultant value of the expression in the matching arm is the value that gets returned for the entire `match` expression.

```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### Pattern that Binds to Values

- `match` arms can bind to the parts of the values that match the pattern.

### Matching with Option<T>

```
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
```

- When we call plus_one(five), the variable x in the body of plus_one will have the value Some(5). We then compare that against each match arm.

### Matches are Exhaustive

- The arms' patterns must cover all possibilities.

### Catch-All Patterns and the \_ Placeholder

- A pattern we can use when we want a catch-all but don’t want to use the value in the catch-all pattern: \_ is a special pattern that matches any value and does not bind to that value.

### Concise Control Flow with `if let` and `let else`

- `if let` lets you handle values that match one pattern while igorning the rest.

```
let config_max = Some(3u8);
match config_max { // executes code when the value is some
  Some(max) => println!("The maximum is configured to be {max}),
  _ => ()
}
```

```
let config_max = Some(3u8);
if let Some(max) = config_max {
  println!("The maximum is configured to be {max});
}
```

- `if let` takes a pattern (Some(max)) and an expression (println!) separated by a `=`
- Using `if let` means losing exhaustive checking

### Staying on the "Happy Path" with let...else

```
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```

# CHAPTER 7

- Managing Growing Projects with Packages, Crates and Modules
- Packages: A Cargo feature that lets you build, test, and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module

## Packages and Crates

- Crate is the smallest amount of code that the Rust compiler considers.
- Rust considers a single source code file to be a crate as well.
- Crates can contain modules (modules maybe defined in other files).
- Crate can be : binary crate or library crate
- Binary crate: programs compiled to an executable, must have a function called main.
- Library crate: don't have a main function, don't compile to an executable. They define functionality intended to be shared with multiple projects.
- Crate root is a source file that Rust compiler starts from. Root module of the crate.
- Package is a bundle of one or more crates, contains a Cargo.toml file that describes how to build those crates.
- Cargo is actually a package that contains the binary crate for the command line tool you’ve been using to build your code. The Cargo package also contains a library crate that the binary crate depends on.
- A package can contain as many binary crates but at most only one library crate. A package must contain at least one crate.
- Running `cargo new my-project` creates a new project with a Cargo.toml and src/main.rs.
- src/main.rs is the crate root of a binary crate with the same name as the package.
- Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root.
- Cargo passes the crate root files to rustc to build the library or binary.
- A package can have multiple binary crates by placing files in the src/bin directory.

## Defining Modules to Controle Scope and Privacy

- Start from the crate root: when compiling a crate, the compiler first looks in the crate root file for code to compile.
- Declaring modules: In the crate root file, you can declare new modules such as `mod garden`. The compiler will look for the module's code:
  - inline, within curly brackets following `mod garden`.
  - in the file src/garden.rs.
  - in the file src/garden/mod.rs.
- Declaring submodules: You can declare submodules in any file other than the crate root. The compiler will look for the submodule's code:
  - inline, within curly brackets following `mod vegetables`.
  - in the file src/garden/vegetables.rs.
  - in the file src/garden/vegetables/mod.rs.
- Paths to code in modules: you can access paths to code in modules using crate::restaurant::frontend::Bookings
- Private vs Public: Code within a module is private from it's parent modules by default. Use `pub mod` to make a module public.
- `use` keyword: To reduce repetition of long paths, the `use` keyword creates shortcuts.

```
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

- Earlier, we mentioned that src/main.rs and src/lib.rs are called crate roots. The reason for their name is that the contents of either of these two files form a module named crate at the root of the crate’s module structure, known as the module tree.

## Paths for Referring to an Item in the Module Tree

A path can take 2 forms:

1. Absolute path is a full path starting from a crate root. For code from external crate, absolute path begins with crate name and for code from current crate, it starts with the litral `crate`.
2. Relative path starts from the current module and uses `self`, `super` or an identifier in the current modules

```
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist!");
        }
    }
}

pub mod dining {
    pub fn eat_at_restaurant() {
        // Absolute path (unchanged)
        crate::front_of_house::hosting::add_to_waitlist();

        // ❌ Relative path (this would fail now)
        // front_of_house::hosting::add_to_waitlist();

        // ✅ Corrected relative path (go up one level to crate root)
        super::front_of_house::hosting::add_to_waitlist();
    }
}
```

- In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default.
- Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules.

### Exposing Paths with the `pub` Keyword

- Adding the `pub` keyword in front of `mod hosting` makes the hosting module public.
- Making the module public doesn’t make its contents public. The pub keyword on a module only lets code in its ancestor modules refer to it, not access its inner code.
- If functions are defined in the same module i.e. function are siblings, we can refer one function from the other.

### Starting Relative Paths with super

- `super` is equivalent to `..` syntax that means to go to the parent directory.

### Making Structs and Enums Public

- We can also use pub to designate structs and enums as public, but there are a few extra details to the usage of pub with structs and enums. If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private.
- If we make an enum public, all of its variants are then public.

## Bringing Paths into Scope with the `use` keyword

- we can create a shortcut to a path with the `use` keyword

```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

- `use` only creates the shortcut for the particular scope in which the `use` occurs.

### Creating Idiomatic `use` Paths

- The above code is the idiomatic way to bring a function into scope with `use`. Bringing the function’s parent module into scope with use means we have to specify the parent module when calling the function. Specifying the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path.
- when bringing in structs, enums, and other items with `use`, it’s idiomatic to specify the full path.
- If we’re bringing two items with the same name into scope with use statements, Rust doesn’t allow that.

```
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

- Using the parent modules distinguishes the two Result types.

### Providing New Names with the `as` keyword

- There’s another solution to the problem of bringing two types of the same name into the same scope with `use`: after the path, we can specify `as` and a new local name, or alias, for the type.

```
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

### Re-exporting Names with `pub use`

- Re-exporting: To enable code outside the scope into which we imported a name with the `use` keyword, to refer to that name as if it had been defined in that scope, we can combine `pub` and `use`.
- This technique is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope.

```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

- Before this change, external code would have to call the add_to_waitlist function by using the path restaurant::front_of_house::hosting::add_to_waitlist(), which also would have required the front_of_house module to be marked as pub. Now that this pub use has re-exported the hosting module from the root module, external code can use the path restaurant::hosting::add_to_waitlist() instead.

### Using External Packages

- Adding the crate name as a dependency in Cargo.toml tells Cargo to download the package and any dependencies from crates.io and make the crate available to our project.
- To bring rand definitions into the scope of our package, we add a `use` line starting with the name of the crate and list the items we wanted to bring into scope.

### Using Nested Paths to Clean Up Large use Lists

- We can use nested paths to bring the same items into scope in one line. We do this by specifying the common part of the path, followed by two colons, and then curly brackets around a list of the parts of the paths that differ

```
// --snip--
use std::{cmp::Ordering, io};
// --snip--

```

```
use std::io;
use std::io::Write;

use std::io::{self, Write};
```

- This line brings std::io and std::io::Write into scope.

### Glob Operator

- If we want to bring all public items defined in a path into scope, we can specify that path followed by the \* glob operator.

## Separating Modules into Different Files

- When modules get large, you might want to move to a separate file to make the code easier to navigate.
- Instead of having all the modules defined in the crate root file, extract the modules into it's own file.
- Leave the `mod front_of_house` in the crate root. Move the code in the module to a new file. The compiler knows to look in this file because it came across the module declaration in the crate root with the module name.
- Module code is private by default, but you can make definitions public by adding the `pub` keyword.

# CHAPTER 8

## Common Collections

- Rust's standard library includes a number of very useful data structures called collections.
- Collections can contain multiple values
- Data that collection points to is stored on the heap; amount of data is not known at compile time and can grow or shrink as the program runs
- Collections very often used:

1. Vector: allows to store a variable number of values next to each other
2. String: a collection of characters
3. Hash Map: allows to associate a value with a specific key

## Storing Lists of Values with Vectors

- Vector `Vec<T>` allows you to store more than one value in a single data structure that puts all the values next to each other in memory.
- Vectors can only store values of the same type.

### Creating a New Vector

- Use type annotation to create a new, empty vector : `let v: Vec<i32> = Vec::new()`
- Use the vec! macro to create a new vector with initial values : `let v = vec![1, 2, 3]`

### Updating a Vector

- `push` method can be used to add elements to the vector

```
let mut v = Vec::new();

v.push(4);
v.push(5);
v.push(6);
```

### Reading Elements of Vectors

- There are 2 ways to reference a value stored in a vector: indexing and using `get` method

```
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
```

- When we use the get method with the index passed as an argument, we get an Option<&T> that we can use with match.
- The first [] method will cause the program to panic because it references a nonexistent element. This method is best used when you want your program to crash if there’s an attempt to access an element past the end of the vector.
- When the get method is passed an index that is outside the vector, it returns None without panicking.

### Iterating Over the Values in a Vector

To access each element in a vector in turn:

```
// Ierating over immutaable references
let v = vec![100, 32, 57];
for i in &v {
  println!("{i}");
}
```

```
// Iterating over mutable references
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

**To change the value that the mutable reference refers to, we have to use the \* dereference operator to get to the value in i before we can use the += operator.**

### Using an Enum to Store Multiple Types

- Vectors can only stores values of the same type. The variants of an enum are of the same enum type, so we can store the enum in vectors.

```
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```

### Dropping a Vector Drops its Elements

```
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
```

## Storing UTF-8 Encoded Text with Strings

- Rust has only one string type in the core language: string slice denoted by `str`.
- String literals, are stored in the program’s binary and are therefore string slices.
- String type provided by Rust's standard library is a growable, mutable, owned, UTF-8 encoded string type.
- Both `String` and string slices are UTF-8 encoded.

### Creating a New String

- Many of the same operations of Vec<T> are available with `String` as well because String is implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions and capabilities. Example: `new()`.
- To start the string with some initial data (create a `String` from a string literal): `to_string()` method
- `String::from` can also be used to create a `String` from a string litral.

### Updating a String

- Use the `+` operator or the `format!` macro to concatenate `String` values

#### Appending to a String with push_str and push

```
    let mut s = String::from("foo");
    s.push_str("bar");
```

```
// Adding one character to a String value using push
    let mut s = String::from("lo");
    s.push('l');
```

#### Concatenation with the + Operator or the format! Macro

```
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

- The string s3 will contain Hello, world!. The reason s1 is no longer valid after the addition, and the reason we used a reference to s2, has to do with the signature of the method that’s called when we use the + operator. The + operator uses the add method, whose signature looks something like this: `fn add(self, s: &str) -> String {`
- So, although `let s3 = s1 + &s2;` looks like it will copy both strings and create a new one, this statement actually takes ownership of s1, appends a copy of the contents of s2, and then returns ownership of the result. In other words, it looks like it’s making a lot of copies, but it isn’t; the implementation is more efficient than copying.

- For combining strings in more complicated ways, we can instead use the format! macro:

```
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
```

- code generated by the format! macro uses references so that this call doesn’t take ownership of any of its parameters.

#### Indexing into Strings

- A String is a wrapper over a Vec<u8>. Let’s look at some of our properly encoded UTF-8 example strings.
  First, this one: `let hello = String::from("Hola");`
- In this case, len will be 4, which means the vector storing the string "Hola" is 4 bytes long. Each of these letters takes one byte when encoded in UTF-8. The following line, however, may surprise you (note that this string begins with the capital Cyrillic letter Ze, not the number 3): `let hello = String::from("Здравствуйте");`
- If you were asked how long the string is, you might say 12. In fact, Rust’s answer is 24: that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8, because each Unicode scalar value in that string takes 2 bytes of storage. Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value. That is why indexing to access parts of a String is not supported in Rust.
- A final reason Rust doesn’t allow us to index into a String to get a character is that indexing operations are expected to always take constant time (O(1)). But it isn’t possible to guarantee that performance with a String, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.

#### Slicing Strings

- Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice.
- Rather than indexing using [] with a single number, you can use [] with a range to create a string slice containing particular bytes.

```
let hello = "Здравствуйте";

let s = &hello[0..4];
```

#### Methods for Iterating Over Strings

- The bestway to operate on pieces of strings is to be explicit about whether you want characters or bytes/
- For individual Unicode scalar values, use `chars` methods.

```
for c in "3A".chars() {
  // Calling `chars` on "3A" separates out and returns two values of type `char`
  println!("{c}")
}
```

- The `bytes` method returns each raw byte.

```
for b in "3A".bytes() {
  println!("{b}");
}
```

## Storing Keys with Associated Values in Hash Maps

- The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function which determines how it places these keys and values in the memory.

### Creating a New Hash Map

- Use `new` to create an empty hash map and use `insert` to add elements

```
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue), 10);
scores.insert(String::("Yellow"), 50);
```

- Hash maps are homogeneous: all of the keyws must have the smae type and all of the values must have the same type.

### Accessing Values in a Hash Map

- We can get a value out of a Hash Map by providing the key to the `get` method:

```
let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```

- The `get` method returns an Option<&V>
- If no value for that key, `get` will return `None`.
- This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>, then unwrap_or to set score to zero if scores doesn’t have an entry for the key.
- We can iterate over each key-value pair in a hash map.

```
for (key, value) in &scores {
  println!("{key}: {value}");
}
```

### Hash Maps and Ownership

- Types that implement the `Copy` trait like `i32` are copied into the hash map. For owned values like `String`, the values will be moved and hash map will be the owner of those values.

```
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// We aren’t able to use the variables field_name and field_value after they’ve been moved into the hash map with the call to insert.
```

- If we insert references to values into the hash map, the values won't be moved into the hash map. The values that the references point to must be valid for at least as long as the hash map is valid.

### Updating a Hash Map

#### Overwriting a Value

- When a key already has a value assigned, if we insert the same key with a different, new value into the hash map, the old value gets overwritten or replaced by the new value.

```
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{scores:?});
```

#### Adding a Key and Value Only If a Key isn't Present

- It is common to check if a key already exists in a hash map and then take one of the following actions: if the key already exists, the existing value should remain as is, if the key does not exist, insert the key and value pair.
- Use `entry` API that takes the key you want to check, it returns an enum `Entry` that represents a value that might or might not exist.

```
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Blue")).or_insert(50);

println!("{scores:?}");
```

- The `or_insert` method on `Entry` is defined to return the mutable reference to the value for the corresponding key if that key exists else it inserts the parameter as the new value and returns a mutable reference to the new value.

#### Updating a Value Based on the Old Value

```
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
  let count = map.entry(word).or_insert(0);
  *count += 1;
}
```

- The split_whitespace method returns an iterator over subslices, separated by whitespace, of the value in text. The or_insert method returns a mutable reference (&mut V) to the value for the specified key. Here, we store that mutable reference in the count variable, so in order to assign to that value, we must first dereference count using the asterisk (\*). The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.
- By default, HashMap uses a hashing function called SipHash

| Concept               | Example                  | Meaning                                                  |
| --------------------- | ------------------------ | -------------------------------------------------------- |
| **Mutable binding**   | `let mut x = something;` | You can _reassign_ `x` itself (e.g., `x = other_thing;`) |
| **Mutable reference** | `let x: &mut i32 = ...;` | You can _mutate the value that x points to_ (`*x += 1`)  |

# CHAPTER 9

- Error Handling: Rust requires you to acknowledge the possibility of error and take some action before your code compiles.
- Rust groups errors into 2 major categories: recoverable and unrecoverable
- For a recoverable error: we want to report the problem and retry
- For unrecoverable error: we want to stop the program
- Rust doesn't have exception mechanism. Rust has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops execution when unrecoverable error is encountered.

## Unrecoverable Errors with panic!

- 2 ways to cause a panic: by taking an action that causes a panic (accessing array past the end) or calling the `panic!` macro
- panics will print a failure messsage, unwind, clean up the stack and quit.

### Unwinding the Stack or Aborting in Response to a Panic

- By default, when a panic occurs, the program starts unwinding which means Rust walks back up the stack and cleans up the data from each function. Walking back and cleaning up is a lot of work, therefore, Rust allows you to choose the alternative of immediately aborting which ends the program without cleaning up.

```
fn main() {
  panic!("crash and burn");
}
```

- Backtrace is a list of all the functions that have been called to get to this point. Backtraces in Rust work as they do in other languages: the key to reading the backtrace is to start from the top and read until you see files you wrote. That’s the spot where the problem originated. The lines above that spot are code that your code has called; the lines below are code that called your code.

## Recoverable Errors with Result

- We can use Result enum in situations where the success value and error value we want to return differs.

```
use std::fs::File;

fn main() {
  let greeting_file_result = File::open("hello.txt");

  let greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the file: {error:?}")
  };
}
```

### Matching on Different Errors

- If we want to take different actions for different failure reasons

```
use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let greeting_file_result = File::open("hello.txt");

  let greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {e:?}),
      },
      _ => {
        panic!("Problem opening the file: {error:?}");
      }
    }
  }
}
```

- The type of the value that File::open returns inside the Err variant is io::Error, which is a struct provided by the standard library. This struct has a method kind that we can call to get an io::ErrorKind value. The enum io::ErrorKind is provided by the standard library and has variants representing the different kinds of errors that might result from an io operation. The variant we want to use is ErrorKind::NotFound, which indicates the file we’re trying to open doesn’t exist yet. So we match on greeting_file_result, but we also have an inner match on error.kind().
- The condition we want to check in the inner match is whether the value returned by error.kind() is the NotFound variant of the ErrorKind enum. If it is, we try to create the file with File::create. However, because File::create could also fail, we need a second arm in the inner match expression. When the file can’t be created, a different error message is printed. The second arm of the outer match stays the same, so the program panics on any error besides the missing file error.

### Shortcuts for Panic on Error: unwrap and expect

- Using `match` can be verbose and doesn't always communicate intent.
- `unwrap` method of `Result<T, E>` type is a shortcut. If the `Result` value is the `Ok` variant, `unwrap` will return the value inside `Ok`. If the `Result` is the `Err` variant, `unwrap` will call the `panic!`.

```
fn main() {
  let greeting_file = File::open("hello.txt").unwrap();
}
```

- Using `expect` method allows to choose the panic! error message.

```
fn main() {
  let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be included in this project");
}
```

**We use expect in the same way as unwrap: to return the file handle or call the panic! macro.**

### Propagating Errors

- We can return the error originating from an implementation that might fail to the calling code. This is known as propagating the error.

### A Shortcut for Propagating Errors: ? Operator

- The `?` placed after a Result value is defined to work in almost the same way as the `match` expressions
- If the value of the Result is an `Ok`, the value inside the `Ok` will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.
- The difference between `?` and `match` is that the error values that have `?` operator go through the `from` function defined in the `From` trait in the standard library, which is used to convert values from one type into another.

### Where the ? Operator can be Used

The `?` operator can only be used in functions whose return type is compatible with the value on which `?` is used.

```
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;
}
```

- The above implementation fails because the value returned by `File::open` follows `Result` but the main function has the return type of `()`, not `Result`.
- Note that you can use the `?` operator on a Result in a function that returns Result, and you can use the `?` operator on an Option in a function that returns Option, but you can’t mix and match.

```
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```

- `Box<dyn Error>` type is a trait object. "Any kind of error".
- Even though the body of this main function will only ever return errors of type std::io::Error, by specifying Box<dyn Error>, this signature will continue to be correct even if more code that returns other errors is added to the body of main.

## To panic! or Not to panic!

- When the code panics, there is no way to recover.
- When you call `panic!` you are making a decision on behalf of the calling code that the situation will be unrecoverable. When you return a `Result` value, you give options to the calling code.
  **Returning a `Result` is a good default choice.**

### Examples, Prototype Code and Tests

- It is good to panic in examples, prototype code and tests. If a method call fails in a test, we want the whole test to fail, even if that method isn't under test.

### Cases in which you have more information than the compiler

```
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");
```

- We’re creating an IpAddr instance by parsing a hardcoded string. We can see that 127.0.0.1 is a valid IP address, so it’s acceptable to use expect here. However, having a hardcoded, valid string doesn’t change the return type of the parse method: we still get a Result value, and the compiler will still make us handle the Result as if the Err variant is a possibility because the compiler isn’t smart enough to see that this string is always a valid IP address. If the IP address string came from a user rather than being hardcoded into the program and therefore did have a possibility of failure, we’d definitely want to handle the Result in a more robust way instead. Mentioning the assumption that this IP address is hardcoded will prompt us to change expect to better error-handling code if, in the future, we need to get the IP address from some other source instead.

```
pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100");
    }
    Guess { value }
  }
  pub fn value(&self) -> i32 {
    self.value
  }
}
```

# CHATPER 10

## Generic Types, Traits and Lifetimes

- generics - a tool for handling duplication of concepts.
- traits - to define behaviour in a generic way.
- lifetimes - allows to give enough information about borrowed values so that references will be valid.

### Removing duplication by extracting a function

- Generics allow us to replace specific types with placeholder that represents multiple types to remove code duplication.

```
fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let mut largest = &number_list[0];

  for number in &number_list {
    if number > largest {
      largest = number;
    }
  }

  println!("The largest number is {largest});
}
```

## Generic Data Types

- Generics can be used to create definitions of function signature or structs
- How to declare generic types? : declare the name of the type parameter inside angle brackets just after the name of the function/struct/enum etc. then use the generic type inside the definition where we would otherwise specify concrete data types.

### Function Definition

- When defining a function that uses generics, we place generics in the signature of the function where we specify data types of parameters and return value.
- We have to mention the type parameter name before using in the signature of the function.

```
fn largest<T>(list: &[T]) -> &T {}
// function largest is generic over some type T
```

### Struct Definition

- define structs to use a generic type parameter using `<>` syntax

```
struct Point<T> {
  x: T,
  y: T
}

fn main() {
  let integer = Point { x: 5, y: 6};
  let float = Point { x: 4.5, y: 0.6 };
}
```

- We can change the definition of Point to be generic over types T and U where x is of type T and y is of type U.

### Enum Definition

- define enums to hold generic data types in their variants
- `Option<T>` enum is generic over type T and has 2 variants: `Some` which holds one value of type T and `None` variant that doesn't hold any value.
- As `Option<T>` enum is generic, we can use it to express the abstract concept of an optional value no matter what the type of value is.

### Method Definition

- We can implement methods on structs and enums and use generic types in their definition.

```
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
```

- `impl` can have separate generic types than the methods within `impl`.
- In the above example, `impl` is generic over X1 and Y1 but the mixup method is generic over X2 and Y2.

### Performance of Code using Generics

- There is no runtime cost by using generics
- Rust uses monomorphization of the code, a process that looks at all the places where generic code is called and generates the code using concrete types.

## Traits: Defining Shared Behavior

- trait defines the functionality a particular type has.
- trait bounds specify that a generic type can be any type that has a certain behavior

### Defining a Trait

- a type's behavior consists of the methods we can call. Different types share the same behavior if we can call the same methods of those types.

```
pub trait Summary {
    fn summarize(&self) -> String;
}
```

- we declare a trait using the `trait` keyword.
- A trait can have multiple methods in its body: each method signature ends in a semicolon.
- Inside the curly brackets, we declare the method signatures that describe the behaviors of the types that implement this trait.
- Each type must provide it's own custom behavior for the body of the method.
