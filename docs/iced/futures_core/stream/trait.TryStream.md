---
title: "TryStream"
kind: trait
path: "futures_core::stream::TryStream"
crate: "futures_core"
module: "futures_core::stream"
file: "futures_core/stream/trait.TryStream.md"
docs_url: "https://docs.iced.rs/futures_core/stream/trait.TryStream.html"
source_file: "src/futures_core/stream.rs"
source_lines: "177-193"
---

# `futures_core::stream::TryStream`

```rust
pub trait TryStream: Stream + Sealed {
    type Ok;
    type Error;

    // Required method
    fn try_poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Ok, Self::Error>>>;
}
```

A convenience for streams that return `Result` values that includes a variety of adapters tailored to such futures.

**Source:** [stream.rs](../../src/futures_core/stream.rs.md) (lines 177-193)

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

### `try_poll_next`

```rust
fn try_poll_next( self: Pin<&mut Self>, cx: &mut Context<'_>, ) -> Poll<Option<Result<Self::Ok, Self::Error>>>
```

Poll this `TryStream` as if it were a `Stream`.

This method is a stopgap for a compiler limitation that prevents us from directly inheriting from the `Stream` trait; in the future it won’t be needed.

## Implementors

- `impl<S, T, E> TryStream for S where S: ?Sized + Stream<Item = Result<T, E>>,`
  - `type Ok = T`
  - `type Error = E`
