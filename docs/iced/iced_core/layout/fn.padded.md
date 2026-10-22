---
title: "padded"
kind: function
path: "iced_core::layout::padded"
crate: "iced_core"
module: "iced_core::layout"
file: "iced_core/layout/fn.padded.md"
docs_url: "https://docs.iced.rs/iced_core/layout/fn.padded.html"
source_file: "src/iced_core/layout.rs"
source_lines: "169-180"
---

# `iced_core::layout::padded`

```rust
pub fn padded(
    tree: &mut Tree,
    limits: &Limits,
    width: impl Into<Length>,
    height: impl Into<Length>,
    padding: impl Into<Padding>,
    layout: impl FnOnce(&mut Tree, &Limits) -> Size,
)
```

Computes the [`Size`](../struct.Size.md) that fits the [`Limits`](struct.Limits.md) given some width, height, and [`Padding`](../padding/struct.Padding.md) requirements and a closure that produces the content inside the given [`Limits`](struct.Limits.md).

**Source:** [layout.rs](../../src/iced_core/layout.rs.md) (lines 169-180)
