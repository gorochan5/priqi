---
title: "positioned"
kind: function
path: "iced_core::layout::positioned"
crate: "iced_core"
module: "iced_core::layout"
file: "iced_core/layout/fn.positioned.md"
docs_url: "https://docs.iced.rs/iced_core/layout/fn.positioned.html"
source_file: "src/iced_core/layout.rs"
source_lines: "183-205"
---

# `iced_core::layout::positioned`

```rust
pub fn positioned(
    tree: &mut Tree,
    limits: &Limits,
    width: impl Into<Length>,
    height: impl Into<Length>,
    padding: impl Into<Padding>,
    layout: impl FnOnce(&mut Tree, &Limits) -> Size,
    translate: impl FnOnce(Size, Size) -> Vector,
)
```

Computes a [`padded`](fn.padded.md) [`Size`](../struct.Size.md) with a positioning step.

**Source:** [layout.rs](../../src/iced_core/layout.rs.md) (lines 183-205)
