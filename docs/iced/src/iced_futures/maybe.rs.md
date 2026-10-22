---
title: "src/iced_futures/maybe.rs"
kind: source
path: "src/iced_futures/maybe.rs"
crate: "iced_futures"
file: "src/iced_futures/maybe.rs.md"
docs_url: "https://docs.iced.rs/src/iced_futures/maybe.rs.html"
source_file: "src/iced_futures/maybe.rs"
---

# `src/iced_futures/maybe.rs`

```rust
#[cfg(not(target_arch = "wasm32"))]
mod platform {
    /// An extension trait that enforces `Send` only on native platforms.
    ///
    /// Useful for writing cross-platform async code!
    pub trait MaybeSend: Send {}

    impl<T> MaybeSend for T where T: Send {}

    /// An extension trait that enforces `Sync` only on native platforms.
    ///
    /// Useful for writing cross-platform async code!
    pub trait MaybeSync: Sync {}

    impl<T> MaybeSync for T where T: Sync {}
}

#[cfg(target_arch = "wasm32")]
mod platform {
    /// An extension trait that enforces `Send` only on native platforms.
    ///
    /// Useful for writing cross-platform async code!
    pub trait MaybeSend {}

    impl<T> MaybeSend for T {}

    /// An extension trait that enforces `Sync` only on native platforms.
    ///
    /// Useful for writing cross-platform async code!
    pub trait MaybeSync {}

    impl<T> MaybeSync for T {}
}

pub use platform::{MaybeSend, MaybeSync};
```
