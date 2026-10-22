---
title: "ready"
kind: macro
path: "futures_core::ready"
crate: "futures_core"
module: "futures_core"
file: "futures_core/macro.ready.md"
docs_url: "https://docs.iced.rs/futures_core/macro.ready.html"
source_file: "src/futures_core/task/poll.rs"
source_lines: "8-15"
---

# `futures_core::ready`

```rust
macro_rules! ready {
    ($e:expr $(,)?) => { ... };
}
```

Extracts the successful type of `Poll<T>`.

This macro bakes in propagation of `Pending` signals by returning early.

**Note:** Since Rust 1.64, this macro is soft-deprecated in favor of [`ready!`](https://doc.rust-lang.org/nightly/core/task/ready/macro.ready.html) macro in the standard library.

**Source:** [poll.rs](../src/futures_core/task/poll.rs.md) (lines 8-15)
