---
title: "Display"
kind: trait
path: "iced_graphics::compositor::Display"
crate: "iced_graphics"
module: "iced_graphics::compositor"
file: "iced_graphics/compositor/trait.Display.md"
docs_url: "https://docs.iced.rs/iced_graphics/compositor/trait.Display.html"
source_file: "src/iced_graphics/compositor.rs"
source_lines: "114"
---

# `iced_graphics::compositor::Display`

```rust
pub trait Display:
    HasDisplayHandle
    + Debug
    + Send
    + Sync
    + 'static { }
```

An owned display handle that can be used in a [`Compositor`](trait.Compositor.md).

This is just a convenient super trait of the `raw-window-handle` trait.

**Source:** [compositor.rs](../../src/iced_graphics/compositor.rs.md) (lines 114)

## Implementors

- `impl<T> Display for T where T: HasDisplayHandle + Debug + Send + Sync + 'static,`
