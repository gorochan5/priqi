---
title: "Window"
kind: trait
path: "iced_graphics::compositor::Window"
crate: "iced_graphics"
module: "iced_graphics::compositor"
file: "iced_graphics/compositor/trait.Window.md"
docs_url: "https://docs.iced.rs/iced_graphics/compositor/trait.Window.html"
source_file: "src/iced_graphics/compositor.rs"
source_lines: "106"
---

# `iced_graphics::compositor::Window`

```rust
pub trait Window:
    HasWindowHandle
    + Debug
    + MaybeSend
    + MaybeSync
    + 'static { }
```

A window that can be used in a [`Compositor`](trait.Compositor.md).

This is just a convenient super trait of the `raw-window-handle` traits.

**Source:** [compositor.rs](../../src/iced_graphics/compositor.rs.md) (lines 106)

## Implementors

- `impl<T> Window for T where T: HasWindowHandle + Debug + MaybeSend + MaybeSync + 'static,`
