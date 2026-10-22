---
title: "Straw"
kind: trait
path: "iced::task::Straw"
crate: "iced"
module: "iced::task"
file: "iced/task/trait.Straw.md"
docs_url: "https://docs.iced.rs/iced/task/trait.Straw.html"
---

# `iced::task::Straw`

```rust
pub trait Straw<Output, Progress = Output, Error = Infallible>: Sipper<Result<Output, Error>, Progress> { }
```

A [`Straw`](trait.Straw.md) is a [`Sipper`](trait.Sipper.md) that can fail.

This is an extension trait of [`Sipper`](trait.Sipper.md), for convenience.

## Implementors

- `impl<S, Output, Progress, Error> Straw<Output, Progress, Error> for S where S: Sipper<Result<Output, Error>, Progress>,`
