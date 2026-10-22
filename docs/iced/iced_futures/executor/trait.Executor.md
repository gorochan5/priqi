---
title: "Executor"
kind: trait
path: "iced_futures::executor::Executor"
crate: "iced_futures"
module: "iced_futures::executor"
file: "iced_futures/executor/trait.Executor.md"
docs_url: "https://docs.iced.rs/iced_futures/executor/trait.Executor.html"
source_file: "src/iced_futures/executor.rs"
source_lines: "5-27"
---

# `iced_futures::executor::Executor`

```rust
pub trait Executor: Sized {
    // Required methods
    fn new() -> Result<Self, Error>
       where Self: Sized;
    fn spawn(&self, future: impl Future<Output = ()> + MaybeSend + 'static);
    fn block_on<T>(&self, future: impl Future<Output = T>) -> T;

    // Provided method
    fn enter<R>(&self, f: impl FnOnce() -> R) -> R { ... }
}
```

A type that can run futures.

**Source:** [executor.rs](../../src/iced_futures/executor.rs.md) (lines 5-27)

## Required Methods

### `new`

```rust
fn new() -> Result<Self, Error> where Self: Sized,
```

Creates a new [`Executor`](trait.Executor.md).

### `spawn`

```rust
fn spawn(&self, future: impl Future<Output = ()> + MaybeSend + 'static)
```

Spawns a future in the [`Executor`](trait.Executor.md).

### `block_on`

```rust
fn block_on<T>(&self, future: impl Future<Output = T>) -> T
```

*Available on non-WebAssembly only.*

Runs a future to completion in the current thread within the [`Executor`](trait.Executor.md).

## Provided Methods

### `enter`

```rust
fn enter<R>(&self, f: impl FnOnce() -> R) -> R
```

Runs the given closure inside the [`Executor`](trait.Executor.md).

Some executors, like `tokio`, require some global state to be in place before creating futures. This method can be leveraged to set up this global state, call a function, restore the state, and obtain the result of the call.

## Implementors

- `impl Executor for iced_futures::backend::native::smol::Executor`
- `impl Executor for iced_futures::backend::null::Executor`
- `impl Executor for iced_futures::backend::native::thread_pool::Executor`
- `impl Executor for iced_futures::backend::native::tokio::Executor`
