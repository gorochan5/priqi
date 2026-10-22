---
title: "src/iced_futures/backend/default.rs"
kind: source
path: "src/iced_futures/backend/default.rs"
crate: "iced_futures"
file: "src/iced_futures/backend/default.rs.md"
docs_url: "https://docs.iced.rs/src/iced_futures/backend/default.rs.html"
source_file: "src/iced_futures/backend/default.rs"
---

# `src/iced_futures/backend/default.rs`

```rust
//! A default, cross-platform backend.
//!
//! - On native platforms, it will use:
//!   - `backend::native::tokio` when the `tokio` feature is enabled.
//!   - `backend::native::smol` when the `smol` feature is enabled.
//!   - `backend::native::thread_pool` when the `thread-pool` feature is enabled.
//!   - `backend::null` otherwise.
//!
//! - On Wasm, it will use `backend::wasm::wasm_bindgen`.
#[cfg(not(target_arch = "wasm32"))]
mod platform {
    #[cfg(feature = "tokio")]
    pub use crate::backend::native::tokio::*;

    #[cfg(all(feature = "smol", not(feature = "tokio"),))]
    pub use crate::backend::native::smol::*;

    #[cfg(all(feature = "thread-pool", not(any(feature = "tokio", feature = "smol"))))]
    pub use crate::backend::native::thread_pool::*;

    #[cfg(not(any(feature = "tokio", feature = "smol", feature = "thread-pool")))]
    pub use crate::backend::null::*;
}

#[cfg(target_arch = "wasm32")]
mod platform {
    pub use crate::backend::wasm::wasm_bindgen::*;
}

pub use platform::*;
```
