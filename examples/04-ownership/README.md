# Ownership

Ownership is Rust’s most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector.

All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that constantly looks for no longer used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory.

Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks at compile time. None of the ownership features slow down your program while it’s running.

## What happens when you assign values and pass them around?

- All primitive types (and types allocated on stack) implement a `Copy` trait
- If a type implements the Copy trait, an older variable is still usable after assignment
- Types allocated on Heap usually don't implement the `Copy` trait, they get moved over!

## Ownership Rules

- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### Borrowing

Borrowing doesn't transfer ownership!

#### Using references

- Using `&` and `*`
  - looks similar to pointers in other languages
- Note: In rust, the type info contains `&` instead of `*`

References can be mutable too!

- Like variables, references aren't mutable by default
- You can use the `mut` keyword to create mutable references
- Cannot mix immutable vars with mutable references
- you can have only one mutable reference to a particular piece of data in a particular scope.

##### Rules of References

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

#### Using slices

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

Eg. `[start_index..end_index]`

---

Some of the common problems avoided by Rust's ownership and borrowing:

- Double free (Transferring)
- Dangling Pointers (Borrowing)
- Data Races (Single Mutable References)
- Index out of bound (slices)
