---
title: "src/futures_core/task/__internal/mod.rs"
kind: source
path: "src/futures_core/task/__internal/mod.rs"
crate: "futures_core"
file: "src/futures_core/task/__internal/mod.rs.md"
docs_url: "https://docs.iced.rs/src/futures_core/task/__internal/mod.rs.html"
source_file: "src/futures_core/task/__internal/mod.rs"
---

# `src/futures_core/task/__internal/mod.rs`

```rust
#[cfg_attr(target_os = "none", cfg(any(target_has_atomic = "ptr", feature = "portable-atomic")))]
mod atomic_waker;
#[cfg_attr(
    target_os = "none",
    cfg(any(target_has_atomic = "ptr", feature = "portable-atomic"))
)]
pub use self::atomic_waker::AtomicWaker;
```
