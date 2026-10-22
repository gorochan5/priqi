---
title: "Window"
kind: trait
path: "iced_core::window::Window"
crate: "iced_core"
module: "iced_core::window"
file: "iced_core/window/trait.Window.md"
docs_url: "https://docs.iced.rs/iced_core/window/trait.Window.html"
source_file: "src/iced_core/window.rs"
source_lines: "37"
---

# `iced_core::window::Window`

```rust
pub trait Window:
    HasWindowHandle
    + HasDisplayHandle
    + Debug { }
```

A window managed by iced.

It implements both [`HasWindowHandle`] and [`HasDisplayHandle`].

**Source:** [window.rs](../../src/iced_core/window.rs.md) (lines 37)

## Implementors

- `impl<T> Window for T where T: HasWindowHandle + HasDisplayHandle + Debug,`
