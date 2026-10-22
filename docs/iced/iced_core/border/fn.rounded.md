---
title: "rounded"
kind: function
path: "iced_core::border::rounded"
crate: "iced_core"
module: "iced_core::border"
file: "iced_core/border/fn.rounded.md"
docs_url: "https://docs.iced.rs/iced_core/border/fn.rounded.html"
source_file: "src/iced_core/border.rs"
source_lines: "24-26"
---

# `iced_core::border::rounded`

```rust
pub fn rounded(radius: impl Into<Radius>) -> Border
```

Creates a new [`Border`](struct.Border.md) with the given [`Radius`](struct.Radius.md).

```rust
assert_eq!(border::rounded(10), Border::default().rounded(10));
```

**Source:** [border.rs](../../src/iced_core/border.rs.md) (lines 24-26)
