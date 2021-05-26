# Closures and Iterators

Rust "borrows" a lot of concepts from functional programming languages.

- Closures use ruby like syntax.

- In addition, can have types of input & return specified
  - Otherwise, inferred once

- Closures can be stored & passed around, using `Fn` traits:

- `Fn` traits are inferred by the compiler automatically

  - `FnOnce`: Moves & the closure can be invoked just once.

  - `FnMut`: Borrows mutably

  - `Fn`: Borrows immutably

- `move` keyword can be used explicitly

## Iterators

Custom types can implement `std::iter::Iterator` trait.

- Lazy evaluation by default

- Force it by calling `collect` result!

```rust
pub trait Iterator {
  type Item;
  fn Next(&mut self) -> Option<Self::Item>
}
```

- `Item` & `Self::Item` are associated types

- Methods that call `next` are "consuming adaptors"
  - eg: `sum`

- Other methods are known as "iterator adaptors"
  - eg: `map`, `filter`

### Creating iterators

- `iter` method: immutable references to elements

- `into_iter` method: takes ownership of whole collection

- `iter_mut` method: mutable reference to elements

## Performance

Iterators are Rust's "zero-cost abstraction"

As defined by Bjarne Stroustrup:

> In general, C++ implementations obey the zero-overhead principle: What you don't use, you don't pay for. And further: What you do use, you couldn't hand code any better.
