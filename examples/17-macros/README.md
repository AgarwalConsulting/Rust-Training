# Macros

Macros are a way of writing code that writes other code, which is known as *metaprogramming*.

Metaprogramming is useful for reducing the amount of code you have to write and maintain, which is also one of the roles of functions. However, macros have some additional powers that functions don’t.

- macros can take a variable number of parameters

- macros are expanded before the compiler interprets the meaning of the code

  - for example, implement a trait on a given type

- you must define macros or bring them into scope before you call them in a file, as opposed to functions you can define anywhere and call anywhere

## Declarative Macros using `macro_rules!`

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

- The `#[macro_export]` annotation indicates that this macro should be made available whenever the crate in which the macro is defined is brought into scope.

- start the macro definition with `macro_rules!`

- The name, in this case `vec`, is followed by curly brackets denoting the body of the macro definition.

- body is similar to the structure of a match expression

  - one arm with the pattern<sup>*</sup> `( $( $x:expr ),* )`

    - First, we use a set of parentheses to encompass the whole pattern.

    - We use a dollar sign `$` to declare a variable in the macro system that will contain the Rust code matching the pattern.

    - Next comes a set of parentheses that captures values that match the pattern within the parentheses for use in the replacement code.

    - Within `$()` is `$x:expr`, which matches any Rust expression and gives the expression the name `$x`.

- in the body of the code for the pattern associated with this arm

  - temp_vec.push() within `$()*` is generated for each part that matches `$()` in the pattern zero or more times depending on how many times the pattern matches

  <sup>*</sup> [patterns definition in documentation](https://doc.rust-lang.org/reference/macros-by-example.html)

## Procedural Macros for Generating Code from Attributes [Reference](https://doc.rust-lang.org/reference/procedural-macros.html)

Procedural macros accept some code as an input, operate on that code, and produce some code as an output rather than matching against patterns and replacing the code with other code as declarative macros do.

The three kinds of procedural macros are:

- custom derive

- attribute-like

- function-like

When creating procedural macros, the definitions must reside in their own crate with a special crate type.
