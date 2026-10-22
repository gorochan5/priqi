---
title: "src/futures_core/task/poll.rs"
kind: source
path: "src/futures_core/task/poll.rs"
crate: "futures_core"
file: "src/futures_core/task/poll.rs.md"
docs_url: "https://docs.iced.rs/src/futures_core/task/poll.rs.html"
source_file: "src/futures_core/task/poll.rs"
---

# `src/futures_core/task/poll.rs`

```rust
/// Extracts the successful type of `Poll<T>`.
///
/// This macro bakes in propagation of `Pending` signals by returning early.
///
/// **Note:** Since Rust 1.64, this macro is soft-deprecated in favor of
/// [`ready!`](core::task::ready) macro in the standard library.
#[macro_export]
macro_rules! ready {
    ($e:expr $(,)?) => {
        match $e {
            $crate::task::Poll::Ready(t) => t,
            $crate::task::Poll::Pending => return $crate::task::Poll::Pending,
        }
    };
}
```
