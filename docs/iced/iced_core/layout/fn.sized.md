---
title: "sized"
kind: function
path: "iced_core::layout::sized"
crate: "iced_core"
module: "iced_core::layout"
file: "iced_core/layout/fn.sized.md"
docs_url: "https://docs.iced.rs/iced_core/layout/fn.sized.html"
source_file: "src/iced_core/layout.rs"
source_lines: "130-143"
---

# `iced_core::layout::sized`

```rust
pub fn sized(
    limits: &Limits,
    width: impl Into<Length>,
    height: impl Into<Length>,
    f: impl FnOnce(&Limits) -> Size,
) -> Size
```

Computes the resulting [`Size`](../struct.Size.md) that fits the [`Limits`](struct.Limits.md) given some width and height requirements and a closure that produces the intrinsic [`Size`](../struct.Size.md) inside the given [`Limits`](struct.Limits.md).

**Source:** [layout.rs](../../src/iced_core/layout.rs.md) (lines 130-143)
