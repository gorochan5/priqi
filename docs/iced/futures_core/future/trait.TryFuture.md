---
title: "TryFuture"
kind: trait
path: "futures_core::future::TryFuture"
crate: "futures_core"
module: "futures_core::future"
file: "futures_core/future/trait.TryFuture.md"
docs_url: "https://docs.iced.rs/futures_core/future/trait.TryFuture.html"
source_file: "src/futures_core/future.rs"
source_lines: "68-81"
---

# `futures_core::future::TryFuture`

```rust
pub trait TryFuture: Future + Sealed {
    type Ok;
    type Error;

    // Required method
    fn try_poll(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Self::Ok, Self::Error>>;
}
```

A convenience for futures that return `Result` values that includes a variety of adapters tailored to such futures.

**Source:** [future.rs](../../src/futures_core/future.rs.md) (lines 68-81)

## Required Associated Types

### `Ok`

```rust
type Ok
```

The type of successful values yielded by this future

### `Error`

```rust
type Error
```

The type of failures yielded by this future

## Required Methods

### `try_poll`

```rust
fn try_poll( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Result<Self::Ok, Self::Error>>
```

Poll this `TryFuture` as if it were a `Future`.

This method is a stopgap for a compiler limitation that prevents us from directly inheriting from the `Future` trait; in the future it won’t be needed.

## Implementors

- `impl<F, T, E> TryFuture for F where F: ?Sized + Future<Output = Result<T, E>>,`
  - `type Ok = T`
  - `type Error = E`
