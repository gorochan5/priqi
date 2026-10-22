---
title: "src/futures_core/task/mod.rs"
kind: source
path: "src/futures_core/task/mod.rs"
crate: "futures_core"
file: "src/futures_core/task/mod.rs.md"
docs_url: "https://docs.iced.rs/src/futures_core/task/mod.rs.html"
source_file: "src/futures_core/task/mod.rs"
---

# `src/futures_core/task/mod.rs`

```rust
//! Task notification.

#[macro_use]
mod poll;

#[doc(hidden)]
pub mod __internal;

#[doc(no_inline)]
pub use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
```
