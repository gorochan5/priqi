---
title: "run"
kind: function
path: "iced::window::run"
crate: "iced"
module: "iced::window"
file: "iced/window/fn.run.md"
docs_url: "https://docs.iced.rs/iced/window/fn.run.html"
---

# `iced::window::run`

```rust
pub fn run<T>(
    id: Id,
    f: impl FnOnce(&dyn Window) -> T + Send + 'static,
) -> Task<T> where
    T: Send + 'static,
```

Runs the given callback with a reference to the [`Window`](../trait.Window.md) with the given [`Id`](struct.Id.md).

Note that if the window closes before this call is processed the callback will not be run.
