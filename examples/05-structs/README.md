# Structs

- Created using `struct` keyword
- Like ruby, if variable name and field names are same, the field name can be avoided during initialization
- Use `struct update syntax`, to create a new struct using some fields of old struct
- Structs can also be tuples
- You can also create unit structs

## Ownership of Struct Data

- To store references, the struct needs to have lifetime parameters in it's definition

## Debugging & Printing structs

- Printing requires implementing `Display` trait
- Instead using the `Debug` trait, and deriving it:

```rust
#[derive(Debug)]
```

- Can print using, `{:?}`, instead of `{}`

- Or use: `{:#?}` for pretty printing

## Methods for struct

- Defined using the `impl` block

- Should take `self` or `&self` reference as first arg

- Methods can take more than one arg

## Associated functions

- `impl` blocks can define other associated functions as well

- Eg. `String::from`

- Typical usage is for creating constructors (`new` is the convention)

You can define multiple `impl` blocks
