---
title: "color"
kind: function
path: "iced_core::border::color"
crate: "iced_core"
module: "iced_core::border"
file: "iced_core/border/fn.color.md"
docs_url: "https://docs.iced.rs/iced_core/border/fn.color.html"
source_file: "src/iced_core/border.rs"
source_lines: "36-38"
---

# `iced_core::border::color`

```rust
pub fn color(color: impl Into<Color>) -> Border
```

Creates a new [`Border`](struct.Border.md) with the given [`Color`](../color/struct.Color.md).

```rust
assert_eq!(border::color(Color::BLACK), Border::default().color(Color::BLACK));
```

**Source:** [border.rs](../../src/iced_core/border.rs.md) (lines 36-38)
