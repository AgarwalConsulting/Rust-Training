# Rust for Server-side development by `@algogrit`

Getting experienced software engineers prepared for building production-ready memory-safe applications.

## Objectives

- Understanding Rust's syntax
- Working with Rust's ownership & borrowing
- Structuring and writing testable Rust code
- Working with and writing ReSTful applications
- Working with Rust's concurrency and understanding advanced patterns

## Prerequisites

- At least one year of active programming experience
- Familiarity with client/server Architecture or ReST
- Familiarity with infrastructure tools/platforms like:
  - Linux & Bash
  - Docker or Kubernetes

## Day 1 (Basics)

- Why Rust?

- Getting Started with Rust Programming Language
  - Installation
  - `rustc` vs `cargo`
  - Profiles
    - `release` vs `dev` (debug)

- Variables, Types & Functions
  - Type inference
  - Immutable vs `mut`
  - Shadowing & Scopes
    - Using `let` for binding
  - Type conversion: `as` vs `into` vs `parse`
  - `let` vs `const`
  - `String` vs `&str`
  - Compound Types: array, tuple
  - Function syntax

- Control Flow
  - `if`
    - assignment
  - `loop`
  - `while`
  - `for _ in _`
    - ranges

- Introduction to Ownership
  - Ownership rules
  - Borrowing - through:
    - References
    - Slices
  - Rust Borrow Checker: Mutable vs Immutable References

- Structs
  - Defining & instantiating
  - `struct` as tuple
  - Methods & Associative functions

## Day 2 (Intermediate)

- Enums
  - Defining & instantiating
  - Using `match` for pattern matching
  - Using `if let`
  - Common Enums: `Option` & `Result`
    - `expect`

- Error Handling
  - `panic!` & unrolling of stack
  - Errors as values using `Result`
  - To `panic!` or not to `panic!`?

- Understanding packages, crates & modules
  - Multiple `bin` crates
  - `lib` crate
  - Defining & using modules
  - Controlling access

- Exploring Collections
  - `Vector`
  - String - `as_bytes` vs `as_chars`
  - `HashMap`

## Day 3 (Advanced)

- Generics
  - Defining generic: `fn`, `struct`, `enum` & Methods
  - Constraints using `trait`s
  - Using `where`
  - What are `std::marker::` traits?

- Advanced traits
  - Trait Objects
  - Introducing `Box` smart pointer
  - Using `type` for generalizing traits

- Lifetimes & generics
  - Lifetime ellision rules

- Writing test cases
  - Unit vs Integration vs Doc tests
  - Test Organization
  - Controlling how tests are run

- Functional language features
  - Closures
    - `Fn` vs `FnMut` vs `FnOnce`
  - Iterators
    - `iter` vs `into_iter` vs `iter_mut`
  - Loop vs Iterators

- Smart Pointers
  - `Box` revisited
    - `Deref` & `Drop` traits
  - `Rc`
  - `RefCell`

## Day 4 (Server-side development)

- Concurrency
  - `std::thread`
  - Sharing State: `Arc` & `Mutex`
  - Communicating: `std::sync::mpsc`

- Writing a web server
  - Simple single-threaded server
  - Making it multi-threaded

- Building a ReSTful API
  - JSON serialization & deserialization

- Working with `reqwest` to interact with third-party APIs

- Adding authentication to your APIs

- Building a micro-service based app using Rust
  - Best practices & architecture
  - Adding graceful shutdown & cleanup
