---
title: "default"
kind: module
path: "iced_futures::backend::default"
crate: "iced_futures"
module: "iced_futures::backend"
file: "iced_futures/backend/default/index.md"
docs_url: "https://docs.iced.rs/iced_futures/backend/default/index.html"
source_file: "src/iced_futures/backend/default.rs"
source_lines: "1-30"
---

# `iced_futures::backend::default`

A default, cross-platform backend.

- On native platforms, it will use:

  - `backend::native::tokio` when the `tokio` feature is enabled.
  - `backend::native::smol` when the `smol` feature is enabled.
  - `backend::native::thread_pool` when the `thread-pool` feature is enabled.
  - `backend::null` otherwise.
- On Wasm, it will use `backend::wasm::wasm_bindgen`.

## Re-exports

- [`tokio`](../native/tokio/index.md) *(tokio)*
