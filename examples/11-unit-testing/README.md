# Testing

> Program testing can be a very effective way to show the presence of bugs, but it is hopelessly inadequate for showing their absence.

Credits: 1972 essay “The Humble Programmer,” Edsger W. Dijkstra

Unit tests need to be defined in the same file as the Source code.

Integration tests can be defined in the `/tests` directory.

Rust also has doc tests.

## Unit tests

- Need to have `#[cfg(test)]` before module

- Need to have `#[test]` before test case

- Can ignore test with `#[ignore]`

- Assertions: all macros!
  - assert_eq!
  - assert!

- Tests can fail, due to panics!

- Panics can be tested using `#[should_panic]`

- Can import `private` & `public` code from `super` module using `use super::*`

## Integration tests

Can test only public API

## Doc tests

Defined using `///`

## Running tests with `cargo test`

- Parallel tests by default: `--test-threads=1`

- Output of passing test: `--show-output`

- Run subset by name: `cargo test <subset-name>`

- Run ignored tests: `--ignored`
