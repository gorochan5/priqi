---
title: "contained"
kind: function
path: "iced_core::layout::contained"
crate: "iced_core"
module: "iced_core::layout"
file: "iced_core/layout/fn.contained.md"
docs_url: "https://docs.iced.rs/iced_core/layout/fn.contained.html"
source_file: "src/iced_core/layout.rs"
source_lines: "148-164"
---

# `iced_core::layout::contained`

```rust
pub fn contained(
    tree: &mut Tree,
    limits: &Limits,
    width: impl Into<Length>,
    height: impl Into<Length>,
    f: impl FnOnce(&mut Tree, &Limits) -> Size,
)
```

Computes the resulting [`Size`](../struct.Size.md) that fits the [`Limits`](struct.Limits.md) given some width and height requirements and a closure that produces the content inside the given [`Limits`](struct.Limits.md).

**Source:** [layout.rs](../../src/iced_core/layout.rs.md) (lines 148-164)
