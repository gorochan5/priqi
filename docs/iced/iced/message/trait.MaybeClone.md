---
title: "MaybeClone"
kind: trait
path: "iced::message::MaybeClone"
crate: "iced"
module: "iced::message"
file: "iced/message/trait.MaybeClone.md"
docs_url: "https://docs.iced.rs/iced/message/trait.MaybeClone.html"
---

# `iced::message::MaybeClone`

```rust
pub trait MaybeClone: Clone { }
```

A trait alias for [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html), but only when the `time-travel` feature is enabled.

## Implementors

- `impl<T> MaybeClone for T where T: Clone,`
