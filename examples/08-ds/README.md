# Collections in std library

- Have different behavior cause of Heap allocations

- A rust programmer will need to understand ownership better before writing any meaningful applications

We’ll discuss three collections that are used very often in Rust programs:

- A vector allows you to store a variable number of values next to each other.
- A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
- A hash map allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.

## Vectors

A contiguous growable array type, written as Vec<T> and pronounced ‘vector’.

- Creating a vector:
  - `Vec::new()`
  - `vec!()` macro

- Updating a vector:
  - `v.push(5)`

- Dropping a Vector Drops Its Elements

- Reading Elements of Vectors
  - Indexing: `v[n]`
  - get method: `v.get(n)`

- Iterating over the Values in a Vector
  - `for _ in _`

- Using an Enum to Store Multiple Types

## String

A String is a wrapper over a `Vec<u8>`.

Trying with “नमस्ते” or "Здравствуйте"!

Slicing Strings: What are we slicing exactly? bytes!

Iterating Over Strings

## HashMap

- `use std::collections::HashMap;`

- `HashMap::new();`

- Inserting into the HashMap
  - `insert` method

- Another way of constructing a hash map is by using iterators and the collect method on a vector of tuples, where each tuple consists of a key and its value.

- Accessing Values in a Hash Map
  - `get` method

- Updating a Hash Map
  - Overwriting a Value: `insert` methods
  - Inserting if there is no value: `entry` & `or_insert` methods
  - Updating values via mutable references

### Hash Maps and Ownership

For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map. For owned values like `String`, the values will be moved and the hash map will be the owner of those values.

### Hashing Functions

HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables.
