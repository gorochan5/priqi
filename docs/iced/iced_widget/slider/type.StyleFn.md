---
title: "StyleFn"
kind: type
path: "iced_widget::slider::StyleFn"
crate: "iced_widget"
module: "iced_widget::slider"
file: "iced_widget/slider/type.StyleFn.md"
docs_url: "https://docs.iced.rs/iced_widget/slider/type.StyleFn.html"
source_file: "src/iced_widget/slider.rs"
source_lines: "621"
---

# `iced_widget::slider::StyleFn`

```rust
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;
```

A styling function for a [`Slider`](struct.Slider.md).

**Source:** [slider.rs](../../src/iced_widget/slider.rs.md) (lines 621)
