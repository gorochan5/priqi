---
title: "Window"
kind: trait
path: "iced::Window"
crate: "iced"
module: "iced"
file: "iced/trait.Window.md"
docs_url: "https://docs.iced.rs/iced/trait.Window.html"
---

# `iced::Window`

```rust
pub trait Window:
    HasWindowHandle
    + HasDisplayHandle
    + Debug { }
```

A window managed by iced.

It implements both [`HasWindowHandle`](window/raw_window_handle/trait.HasWindowHandle.md) and [`HasDisplayHandle`](window/raw_window_handle/trait.HasDisplayHandle.md).

## Implementors

- `impl<T> Window for T where T: HasWindowHandle + HasDisplayHandle + Debug,`
