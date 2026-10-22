---
title: "MaybeDebug"
kind: trait
path: "iced::message::MaybeDebug"
crate: "iced"
module: "iced::message"
file: "iced/message/trait.MaybeDebug.md"
docs_url: "https://docs.iced.rs/iced/message/trait.MaybeDebug.html"
---

# `iced::message::MaybeDebug`

```rust
pub trait MaybeDebug: Debug { }
```

A trait alias for [`Debug`](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html), but only when the `debug` feature is enabled.

## Implementors

- `impl<T> MaybeDebug for T where T: Debug,`
