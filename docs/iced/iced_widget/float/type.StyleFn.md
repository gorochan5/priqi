---
title: "StyleFn"
kind: type
path: "iced_widget::float::StyleFn"
crate: "iced_widget"
module: "iced_widget::float"
file: "iced_widget/float/type.StyleFn.md"
docs_url: "https://docs.iced.rs/iced_widget/float/type.StyleFn.html"
source_file: "src/iced_widget/float.rs"
source_lines: "367"
---

# `iced_widget::float::StyleFn`

```rust
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;
```

A styling function for a [`Float`](struct.Float.md).

**Source:** [float.rs](../../src/iced_widget/float.rs.md) (lines 367)
