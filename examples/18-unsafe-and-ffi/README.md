# Unsafe

You can take five actions in unsafe Rust that you can’t in safe Rust, which we call unsafe superpowers. Those superpowers include the ability to:

- Dereference a raw pointer

- Call an unsafe function or method

- Access or modify a mutable static variable

- Implement an unsafe trait

- Access fields of a union

It’s important to understand that unsafe doesn’t turn off the borrow checker or disable any other of Rust’s safety checks: if you use a reference in unsafe code, it will still be checked. The unsafe keyword only gives you access to these five features that are then not checked by the compiler for memory safety. You’ll still get some degree of safety inside of an unsafe block.

## FFI [Demo Repo](https://github.com/wisonye/rust-ffi-demo) or [Code Along](https://doc.rust-lang.org/nomicon/ffi.html)

Rust has the keyword `extern` that facilitates the creation and use of a Foreign Function Interface (FFI).

An FFI is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions.

### Calling Rust Functions from Other Languages

We can also use extern to create an interface that allows other languages to call Rust functions.

## Best practices for minimizing and encapsulating unsafe code

1. Split your code into safe private functions

```rust
// Document the assumptions of this unsafe function here
pub unsafe fn my_function() {
    my_internal_function()
}

// private
fn my_internal_function() {
    // ... doing safe stuff ...
    unsafe {
        // Document the assumptions of this unsafe block here
        // ... doing `unsafe` stuff ...
    }
    // ... doing safe stuff ...
}
```

2. Use `unsafe_block_in_unsafe_fn/unsafe_op_in_unsafe_fn`

If you add a `#![deny(unsafe_op_in_unsafe_fn)]` somewhere like the top of your crate's `main.rs` file, you will get errors for using unsafe operations in unsafe fns unless you add unsafe blocks around them.
