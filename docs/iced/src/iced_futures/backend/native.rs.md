---
title: "src/iced_futures/backend/native.rs"
kind: source
path: "src/iced_futures/backend/native.rs"
crate: "iced_futures"
file: "src/iced_futures/backend/native.rs.md"
docs_url: "https://docs.iced.rs/src/iced_futures/backend/native.rs.html"
source_file: "src/iced_futures/backend/native.rs"
---

# `src/iced_futures/backend/native.rs`

```rust
//! Backends that are only available in native platforms: Windows, macOS, or Linux.
#[cfg(feature = "tokio")]
pub mod tokio;

#[cfg(feature = "smol")]
pub mod smol;

#[cfg(feature = "thread-pool")]
pub mod thread_pool;
```
