# Outline

## Day 1 (Basics)

- Why Rust?

- Getting Started with Rust Programming Language
  - Installation
  - First Rust Program
  - Programming Concepts

- Introduction to Ownership
  - References and Borrowing
  - Slice Type

- Using Structs to Structure Related Data
  - Defining and Instantiating Structs
  - Method Syntax

- Enums and Pattern Matching
  - Defining an Enum
  - `match` Control Flow operator
  - Control flow with `if let`

- Manage Projects with Packages, Crates, and Modules
  - Packages & Crates
  - Modules
  - Separating modules into different files

## Day 2 (Intermediate)

- Exploring Collections
  - Storing Lists of Values with Vectors
  - Storing UTF-8 Encoded Text with Strings
  - Storing Keys with Associated Values in Hash Maps

- Error Handling
  - Unrecoverable Errors with panic!
  - Recoverable Errors with Result
  - To panic! Or Not to panic!

- Generic Types, Traits, and Lifetimes
  - Generic data types
  - Traits: Defining shared behavior
  - Validating references with Lifetimes

- Writing Automated Tests
  - How to Write Tests?
  - Controlling How Tests Are Run
  - Test Organisation

- Functional Language Features – Iterators and Closures
  - Loops vs Iterators

- Cargo and Crates.io extended
  - Customise Builds with Release Profiles
  - Publish a Crate to Crates.io
  - Cargo Workspaces
  - Install Binaries from Crates.io with Cargo Install
  - Extend Cargo with Custom Command

## Day 3 (Advanced)

- Exploring Smart Pointers

- Fearless Concurrency
  - Threads
  - Message passing between threads
  - Shared-State concurrency
  - Extensible concurrency with the `Sync` & `Send` traits

- Object-Oriented Programming Features of Rust
  - Object-Oriented Languages Characteristics
  - Using Trait Objects
  - Implementing an Object

- Patterns and Matching

- Advanced Features
  - Unsafe Rust
  - Advanced traits
  - Advanced types
  - Advanced functions & closures
  - Macros

## Day 4 (Server-side development)

- Build a Web Server
  - Building a single-threaded web server
  - Turning our single-threaded web server into multi-threaded

- Building a ReSTful API
  - Serialization & deserialization
    - JSON
    - XML

- Working with `reqwest` to interact with third-party APIs

- Adding authentication to your APIs

- Building a micro-service based app using Rust
  - Adding graceful shutdown & cleanup
