---
title: "atomic"
kind: function
path: "iced_core::layout::atomic"
crate: "iced_core"
module: "iced_core::layout"
file: "iced_core/layout/fn.atomic.md"
docs_url: "https://docs.iced.rs/iced_core/layout/fn.atomic.html"
source_file: "src/iced_core/layout.rs"
source_lines: "117-125"
---

# `iced_core::layout::atomic`

```rust
pub fn atomic(
    limits: &Limits,
    width: impl Into<Length>,
    height: impl Into<Length>,
) -> Size
```

Computes the resulting [`Size`](../struct.Size.md) that fits the [`Limits`](struct.Limits.md) given some width and height requirements and no intrinsic size.

**Source:** [layout.rs](../../src/iced_core/layout.rs.md) (lines 117-125)
