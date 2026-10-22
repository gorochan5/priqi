---
title: "Executor"
kind: type
path: "iced_futures::backend::native::tokio::Executor"
crate: "iced_futures"
module: "iced_futures::backend::native::tokio"
file: "iced_futures/backend/native/tokio/type.Executor.md"
docs_url: "https://docs.iced.rs/iced_futures/backend/native/tokio/type.Executor.html"
source_file: "src/iced_futures/backend/native/tokio.rs"
source_lines: "4"
---

# `iced_futures::backend::native::tokio::Executor`

```rust
pub type Executor = Runtime;
```

A `tokio` executor.

**Source:** [tokio.rs](../../../../src/iced_futures/backend/native/tokio.rs.md) (lines 4)

## Trait Implementations

### `impl Executor for Executor`

#### `new`

```rust
fn new() -> Result<Self, Error>
```

Creates a new [`Executor`](../../../executor/trait.Executor.md).

#### `spawn`

```rust
fn spawn(&self, future: impl Future<Output = ()> + Send + 'static)
```

Spawns a future in the [`Executor`](../../../executor/trait.Executor.md).

#### `enter`

```rust
fn enter<R>(&self, f: impl FnOnce() -> R) -> R
```

Runs the given closure inside the [`Executor`](../../../executor/trait.Executor.md). [Read more](../../../executor/trait.Executor.md)

#### `block_on`

```rust
fn block_on<T>(&self, future: impl Future<Output = T>) -> T
```

Runs a future to completion in the current thread within the [`Executor`](../../../executor/trait.Executor.md).
