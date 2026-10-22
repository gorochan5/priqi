---
title: "Renderer"
kind: type
path: "iced::Renderer"
crate: "iced"
module: "iced"
file: "iced/type.Renderer.md"
docs_url: "https://docs.iced.rs/iced/type.Renderer.html"
source_file: "src/iced_renderer/lib.rs"
source_lines: "18"
---

# `iced::Renderer`

```rust
pub type Renderer = Renderer<Renderer, Renderer>;
```

The default graphics renderer for [`iced`](https://github.com/iced-rs/iced).

**Source:** [lib.rs](../src/iced_renderer/lib.rs.md) (lines 18)

## Variants

### `Primary(Renderer)`

The primary rendering option.

### `Secondary(Renderer)`

The secondary (or fallback) rendering option.
