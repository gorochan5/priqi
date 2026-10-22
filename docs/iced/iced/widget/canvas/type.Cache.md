---
title: "Cache"
kind: type
path: "iced::widget::canvas::Cache"
crate: "iced"
module: "iced::widget::canvas"
file: "iced/widget/canvas/type.Cache.md"
docs_url: "https://docs.iced.rs/iced/widget/canvas/type.Cache.html"
source_file: "src/iced_widget/canvas.rs"
source_lines: "79"
---

# `iced::widget::canvas::Cache`

```rust
pub type Cache<Renderer = Renderer<Renderer, Renderer>> = Cache<Renderer>;
```

A simple cache that stores generated [`Geometry`](type.Geometry.md) to avoid recomputation.

A [`Cache`](type.Cache.md) will not redraw its geometry unless the dimensions of its layer change or it is explicitly cleared.

**Source:** [canvas.rs](../../../src/iced_widget/canvas.rs.md) (lines 79)
