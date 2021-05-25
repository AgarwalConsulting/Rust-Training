# Project Management

Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs.

These features, sometimes collectively referred to as the module system, include:

- Packages: A Cargo feature that lets you build, test, and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and `use`: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a `struct`, function, or module

---

## Packages / Crates

A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.

A crate will group related functionality together in a scope so the functionality is easy to share between multiple projects.

- If a package contains src/main.rs and src/lib.rs, it has two crates: a library and a binary, both with the same name as the package.

## Modules

Modules let us organize code within a crate into groups for readability and easy reuse. Modules also control the privacy of items, which is whether an item can be used by outside code (public) or is an internal implementation detail and not available for outside use (private).
