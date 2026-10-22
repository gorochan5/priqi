---
title: "Compositor"
kind: type
path: "iced_renderer::Compositor"
crate: "iced_renderer"
module: "iced_renderer"
file: "iced_renderer/type.Compositor.md"
docs_url: "https://docs.iced.rs/iced_renderer/type.Compositor.html"
source_file: "src/iced_renderer/lib.rs"
source_lines: "23"
---

# `iced_renderer::Compositor`

```rust
pub type Compositor = Compositor<Compositor, Compositor>;
```

The default graphics compositor for [`iced`](https://github.com/iced-rs/iced).

**Source:** [lib.rs](../src/iced_renderer/lib.rs.md) (lines 23)

## Variants

### `Primary(Compositor)`

The primary compositing option.

### `Secondary(Compositor)`

The secondary (or fallback) compositing option.
