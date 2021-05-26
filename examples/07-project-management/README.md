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

- If a package contains `src/main.rs` and `src/lib.rs`, it has two crates: a library and a binary, both with the same name as the package.

- Binary Packages: `src/main.rs`

- Library Packages: `src/lib.rs`

## Modules

Modules let us organize code within a crate into groups for readability and easy reuse. Modules also control the privacy of items, which is whether an item can be used by outside code (public) or is an internal implementation detail and not available for outside use (private).

Related keywords: `mod`, `pub`, `use`, `as`

- `mod`: used to define a module

  - Alternatively, can create a file with same name or `dir/mod.rs` file

- `pub`: Used to make modules & selected functionality public.

  - Related: `in`

- `use`: For importing a path to the current module

  - Related: `crate`, `super`, `self`, `extern`
  - Also has a multi-import syntax as well

- `as`: Is used to aliasing an import

---

## Customizing builds with release profiles

Cargo has two main profiles:

- "dev" profile Cargo uses when you run `cargo build`
- "release" profile Cargo uses when you run `cargo build --release`

The dev profile is defined with good defaults for development, and the release profile has good defaults for release builds.

The opt-level setting controls the number of optimizations Rust will apply to your code, with a range of 0 to 3.

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

## Workspaces

```bash
cat Cargo.toml
```

```toml
[workspace]

members = [
    "<crate names>",
]
```
