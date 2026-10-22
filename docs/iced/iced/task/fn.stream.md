---
title: "stream"
kind: function
path: "iced::task::stream"
crate: "iced"
module: "iced::task"
file: "iced/task/fn.stream.md"
docs_url: "https://docs.iced.rs/iced/task/fn.stream.html"
---

# `iced::task::stream`

```rust
pub fn stream<Output>(sipper: impl Sipper<Output>) -> impl Stream<Item = Output>
```

Turns a [`Sipper`](trait.Sipper.md) into a [`Stream`].

This is only possible if the `Output` and `Progress` types of the [`Sipper`](trait.Sipper.md) match!
