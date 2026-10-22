---
title: "src/iced/time.rs"
kind: source
path: "src/iced/time.rs"
crate: "iced"
file: "src/iced/time.rs.md"
docs_url: "https://docs.iced.rs/src/iced/time.rs.html"
source_file: "src/iced/time.rs"
---

# `src/iced/time.rs`

```rust
//! Listen and react to time.
pub use crate::core::time::*;

#[allow(unused_imports)]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "tokio", feature = "smol", target_arch = "wasm32")))
)]
pub use iced_futures::backend::default::time::*;

use crate::Task;

/// Returns a [`Task`] that produces the current [`Instant`]
/// by calling [`Instant::now`].
///
/// While you can call [`Instant::now`] directly in your application;
/// that renders your application "impure" (i.e. no referential transparency).
///
/// You may care about purity if you want to leverage the `time-travel`
/// feature properly.
pub fn now() -> Task<Instant> {
    Task::future(async { Instant::now() })
}
```
