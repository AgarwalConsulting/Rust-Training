# Tokio

Tokio is an asynchronous runtime for the Rust programming language. It provides the building blocks needed for writing networking applications. It gives the flexibility to target a wide range of systems, from large servers with dozens of cores to small embedded devices.

At a high level, Tokio provides a few major components:

- A multi-threaded runtime for executing asynchronous code.

- An asynchronous version of the standard library.

- A large ecosystem of libraries.

## Primer

The `#[tokio::main]` function is a macro. It transforms the `async fn main()` into a synchronous `fn main()` that initializes a runtime instance and executes the async main function.

or

```rust
let mut rt = tokio::runtime::Runtime::new().unwrap();
rt.block_on(async {
    println!("hello");
})
```

Provides ability to create `async` task and `await` on the value (a future) returned by the task.

A future is a value that implements the `std::future::Future` trait provided by the standard library. They are values that contain the in-progress asynchronous computation.

With asynchronous Rust, cancellation is performed by dropping a future.

## [Internals](https://tokio.rs/tokio/tutorial/async)

## Spawning

A Tokio task is an asynchronous green thread.

They are created by passing an async block to `tokio::spawn`.

When spawning tasks, the spawned async expression must own all of its data.

## Shared State

There are a couple of different ways to share state in Tokio.

- Guard the shared state with a Mutex.

  - `std::sync::Mutex` vs `tokio::sync::Mutex`
    - A common error is to unconditionally use `tokio::sync::Mutex` from within async code. An async mutex is a mutex that is locked across calls to `.await`.

  - This will not only block the current task but it will also block all other tasks scheduled on the current thread.

- Spawn a task to manage the state and use message passing to operate on it.
  - via Channels

## Channels

- `use tokio::sync::mpsc;`

Tokio provides a number of channels, each serving a different purpose.

- `mpsc`: multi-producer, single-consumer channel. Many values can be sent.
- `oneshot`: single-producer, single consumer channel. A single value can be sent.
- `broadcast`: multi-producer, multi-consumer. Many values can be sent. Each receiver sees every value.
- `watch`: multi-producer, multi-consumer. Many values can be sent, but no history is kept. Receivers only see the most recent value.

## select

- `tokio::select!`

allows waiting on multiple async computations and returns when a single computation completes

> The `select!` macro can handle more than two branches. The current limit is `64` branches.

The `select!` macro randomly picks branches to check first for readiness.

When multiple channels have pending values, a random channel will be picked to receive from.

If when `select!` is evaluated, multiple channels have pending messages, only one channel has a value popped. All other channels remain untouched, and their messages stay in those channels until the next loop iteration. No messages are lost.

## Streams

A stream is an asynchronous series of values.

It is the asynchronous equivalent to Rust's `std::iter::Iterator` and is represented by the `Stream` trait.

Tokio provides stream support in a separate crate: `tokio-stream`.

---

## I/O

- `use tokio::io::{self, AsyncReadExt, AsyncWriteExt};`

- `use tokio::net::{TcpListener, TcpStream};`
