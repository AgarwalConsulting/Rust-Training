# Concurrency

Threads can run simultaneously, there’s no inherent guarantee about the order in which parts of your code on different threads will run. This can lead to problems, such as:

- Race conditions, where threads are accessing data or resources in an inconsistent order

- Deadlocks, where two threads are waiting for each other to finish using a resource the other thread has, preventing both threads from continuing

- Bugs that happen only in certain situations and are hard to reproduce and fix reliably

Rust calls the operating system APIs to create threads, which is sometimes called 1:1, meaning one operating system thread per one language thread.

## Using Message Passing to Transfer Data Between Threads

"Do not communicate by sharing memory; instead, share memory by communicating." - Go concurrency slogan

One major tool Rust has for accomplishing message-sending concurrency is the channel, a programming concept that Rust’s standard library provides an implementation of.

A channel in programming has two halves: a transmitter and a receiver.

Channel is defined in `std::sync::mpsc`.

"MPSC" stands for Multiple producer, single consumer.

Created using: `let (tx, rx) = mpsc::channel();`

### Usage

- `tx.send`
- `tx.clone`
- `rx.recv`

## Shared-State Concurrency

- Using Mutexes to Allow Access to Data from One Thread at a Time

### The API of Mutex<T>

- In `std::sync::Mutex`

- `Mutex::new`

- `m.lock`

### How to share a mutex between multiple threads?

- `Arc<T>` is a type like `Rc<T>` that is safe to use in concurrent situations.

- The "a" stands for atomic, meaning it’s an *atomically reference counted* type.

- Rust std lib also has support for atomic operations in `std::sync::atomic`.

### Extending Rust concurrency

- You can write your own concurrency features or use those written by others.

- Two concurrency concepts are embedded in the language: the `std::marker` traits `Sync` and `Send`.

- The `Send` marker trait indicates that ownership of values of the type implementing `Send` can be transferred between threads.

- The `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads.

- Because types that are made up of `Send` and `Sync` traits are automatically also `Send` and `Sync`, we don’t have to implement those traits manually.

- As marker traits, they don’t even have any methods to implement. They’re just useful for enforcing invariants related to concurrency.
