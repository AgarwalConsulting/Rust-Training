# Errors & panics

Two options in Rust:

- `panic!`
  - No recovery

- `Result<T, E>`
  - Variants: `Ok(T)`, `Err(E)`

## Panic

Can get backtrace using: `RUST_BACKTRACE=1` env variable

### Turn off [unwinding](https://doc.rust-lang.org/nomicon/unwinding.html)

```toml
[profile.release]
panic = 'abort'
```

## Result

- Use `match`
  - match on different errors using `kind` method

- Use methods like
  - `unwrap_or_else` <- allows err handling
  - `unwrap` <- panic
  - `expect` <- panic with custom message

- Propagating errors
  - `?` <- can return even from main (`Result<(), Box<dyn Error>>`)
