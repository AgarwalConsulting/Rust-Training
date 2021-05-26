# Enums

- Similar to algebraic data types in ML family of languages.

- Defined using `enum` keyword

Eg:

```rust
enum Action {
  Jump(i32)
  Stop
  Turn(Direction)
  Walk
}
```

- Methods can be defined on `enums` as well, similar to `struct` using `impl`

## Common Enums

### `Option<T>` (`std::option::Option`)

- Two variants: `Some<T>` and `None`

- Rust also doesn't have `nil` or `null`!

- Instead it relies on the `Option` enum in the standard library

### Result<T, Err>

## Pattern matching

- Use `match`

- `match` is exhaustive

- You can use `_` (underscore), as a catch all "match arm"

- You can use `()` as a do nothing, `noop`!

- `if let` is a more concise way to perform the same things as above!
  - can also have `else` & `else if let` blocks
