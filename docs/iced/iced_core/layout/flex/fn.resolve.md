---
title: "resolve"
kind: function
path: "iced_core::layout::flex::resolve"
crate: "iced_core"
module: "iced_core::layout::flex"
file: "iced_core/layout/flex/fn.resolve.md"
docs_url: "https://docs.iced.rs/iced_core/layout/flex/fn.resolve.html"
source_file: "src/iced_core/layout/flex.rs"
source_lines: "94-509"
---

# `iced_core::layout::flex::resolve`

```rust
pub fn resolve<Message, Theme, Renderer>(
    axis: Axis,
    renderer: &Renderer,
    limits: &Limits,
    width: Length,
    height: Length,
    padding: Padding,
    spacing: f32,
    align_items: Alignment,
    children: &mut [Tree],
    items: &mut [impl Widget<Message, Theme, Renderer>],
    cache: &mut Cache,
) -> Sizewhere
    Renderer: Renderer,
```

Computes the flex layout with the given axis and limits, applying spacing, padding and alignment to the items as needed.

The layout is stored in the provided children [`widget::Tree`](../../widget/tree/struct.Tree.md)s, and the resolved size of the container is returned.

The provided [`Cache`](struct.Cache.md) holds the per-child bookkeeping of the algorithm; it is expected to be reused between calls (see [`Cache`](struct.Cache.md)).

**Source:** [flex.rs](../../../src/iced_core/layout/flex.rs.md) (lines 94-509)
