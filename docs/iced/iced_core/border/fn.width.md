---
title: "width"
kind: function
path: "iced_core::border::width"
crate: "iced_core"
module: "iced_core::border"
file: "iced_core/border/fn.width.md"
docs_url: "https://docs.iced.rs/iced_core/border/fn.width.html"
source_file: "src/iced_core/border.rs"
source_lines: "48-50"
---

# `iced_core::border::width`

```rust
pub fn width(width: impl Into<Pixels>) -> Border
```

Creates a new [`Border`](struct.Border.md) with the given `width`.

```rust
assert_eq!(border::width(10), Border::default().width(10));
```

**Source:** [border.rs](../../src/iced_core/border.rs.md) (lines 48-50)
