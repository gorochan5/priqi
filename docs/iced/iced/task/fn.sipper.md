---
title: "sipper"
kind: function
path: "iced::task::sipper"
crate: "iced"
module: "iced::task"
file: "iced/task/fn.sipper.md"
docs_url: "https://docs.iced.rs/iced/task/fn.sipper.html"
---

# `iced::task::sipper`

```rust
pub fn sipper<Progress, F, T>(builder: F) -> impl Sipper<T, Progress> where
    F: AsyncFnOnce(Sender<Progress>) -> T,
```

Creates a new [`Sipper`](trait.Sipper.md) from the given async closure, which receives a [`Sender`] that can be used to notify progress asynchronously.
