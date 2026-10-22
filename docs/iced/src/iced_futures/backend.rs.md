---
title: "src/iced_futures/backend.rs"
kind: source
path: "src/iced_futures/backend.rs"
crate: "iced_futures"
file: "src/iced_futures/backend.rs.md"
docs_url: "https://docs.iced.rs/src/iced_futures/backend.rs.html"
source_file: "src/iced_futures/backend.rs"
---

# `src/iced_futures/backend.rs`

```rust
//! The underlying implementations of the `iced_futures` contract!
pub mod null;

#[cfg(not(target_arch = "wasm32"))]
pub mod native;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub mod default;
```
