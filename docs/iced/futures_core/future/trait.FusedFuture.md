---
title: "FusedFuture"
kind: trait
path: "futures_core::future::FusedFuture"
crate: "futures_core"
module: "futures_core::future"
file: "futures_core/future/trait.FusedFuture.md"
docs_url: "https://docs.iced.rs/futures_core/future/trait.FusedFuture.html"
source_file: "src/futures_core/future.rs"
source_lines: "37-40"
---

# `futures_core::future::FusedFuture`

```rust
pub trait FusedFuture: Future {
    // Required method
    fn is_terminated(&self) -> bool;
}
```

A future which tracks whether or not the underlying future should no longer be polled.

`is_terminated` will return `true` if a future should no longer be polled. Usually, this state occurs after `poll` (or `try_poll`) returned `Poll::Ready`. However, `is_terminated` may also return `true` if a future has become inactive and can no longer make progress and should be ignored or dropped rather than being `poll`ed again.

**Source:** [future.rs](../../src/futures_core/future.rs.md) (lines 37-40)

## Required Methods

### `is_terminated`

```rust
fn is_terminated(&self) -> bool
```

Returns `true` if the underlying future should no longer be polled.

## Implementors
