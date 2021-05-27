# Generics, traits and lifetimes

## Generics

Generics are useful for static dispatching.

You can define generic:

- Functions
- Structs
- Enums
- Methods

Compiler does "Monomorphization"! Generics have zero-cost.

## Traits

Allow us to define constraints on generic features.

- Defined using `trait` keyword
- Can contain default method definitions too!

### Coherence (Orphan Rule)

Trait can be implemented by structs in different modules too

- You can either:
  - implement a trait from same module for an external type
  - implement an external trait for type from same module

### Advanced Traits

- Traits can be used as restrictions for params using `impl`
  - This is a syntactic sugar

- Can specify multiple trait bounds using `+`

- `where` clause, an alternative to improve readability

- Using trait bounds you can also have a "selective" generic implementation

#### Trait Objects

Trait Objects allow dynamic dispatching or runtime polymorphism.

A Trait Object usually takes the form: `Box<dyn {trait-name}>`

## Object Safety Is Required for Trait Objects

You can only make object-safe traits into trait objects.

A trait is object safe if all the methods defined in the trait have the following properties:

- The return type isn’t `Self`.

- There are no generic type parameters.

## Lifetimes

Rust doesn't come with a GC.

- The compiler does come with a "Borrow Checker".

- Some code possible in other languages isn't possible in Rust

- The compiler needs to ensure that there aren't any dangling pointers in your code.

- It does it by keeping track of the lifetime of each variable & it's reference.

- In some cases, the compiler isn't smart enough to keep track of the lifetimes.

- In such cases, a programmer can provide hints: lifetime annotations.

### Lifetime annotations

Similar to generics.

- generics are for types

- lifetimes are for scopes

- Eg: `'a`

- You can define them for functions, structs & methods when working with references

- When passing multiple references with same lifetime annotation
  - the compile uses the lifetime of the reference which is shortest

- The compiler user "lifetime ellision rules" to auto-add lifetime annotations

- There is also `'static` lifetime

As rust programmers, we have an added responsibility of thinking of lifetimes while writing code.

### Lifetime ellision rules

There are two kinds to lifetimes:

- input (parameter)

- output (return)

Some code in rust, using references, can be compiled without lifetime annotations, due to the following rules:

- Each param that is a reference gets its own lifetime parameter

- If there is only one input lifetime param, then that lifetime is assigned to all output lifetime parameters

- For methods: If there is `&self` or `&mut self` input param then that lifetime is assigned to all the outputs
