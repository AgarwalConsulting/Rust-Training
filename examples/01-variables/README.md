# Variables

- Immutable by default (`mut` keyword)
- Allows shadowing!
- Enum for Errors (Result { Ok, Err})
  - `match` keyword
- Types classification:
  - Scalar - integers, floating-point numbers, Booleans, and characters
  - Compound
    - Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

## Constants

- declared using the `const` keyword
- constants cannot be shadowed!

## Scalar

### Integers

- `i8` to `i128`;
  - -(2^(n - 1)) to (2^(n - 1) - 1)
- `u8` to `u128`
- isize & usize (`arch` dependent)
- Use "_" as decimal separator
- Rust defaults to `i32`

### Floats

- `f32` & `f64`
  - `f64` is default
- `IEEE-754` standard

### Boolean

- `bool`

### Character

- `char`
  - four bytes in size and represents a Unicode Scalar Value
- a "character" isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a char is in Rust

## Compound

### Tuple Type

- A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
- Tuples have a fixed length: once declared, they cannot grow or shrink in size.

- Eg:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

- To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value

```rust
let (x, y, z) = tup;
```

- In addition to destructuring through pattern matching, we can access a tuple element directly by using a period (.) followed by the index of the value we want to access.

```rust
let six_point_four = x.1;
```

- As with most programming languages, the first index in a tuple is 0.

### Array Type

- Every element of an array must have the same type.
- arrays in Rust have a fixed length

- Eg:

```rust
let a = [1, 2, 3, 4, 5];
```

- Arrays are useful when you want your data allocated on the stack rather than the heap

- A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.

- Eg:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

- you can specify the initial value, followed by a semicolon, and then the length of the array in square brackets

```rust
let a = [3; 5];
```

The array named a will contain 5 elements that will all be set to the value 3 initially.

- What happens if you try to access an element of an array that is past the end of the array?
