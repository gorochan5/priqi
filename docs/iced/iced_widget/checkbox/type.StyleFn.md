---
title: "StyleFn"
kind: type
path: "iced_widget::checkbox::StyleFn"
crate: "iced_widget"
module: "iced_widget::checkbox"
file: "iced_widget/checkbox/type.StyleFn.md"
docs_url: "https://docs.iced.rs/iced_widget/checkbox/type.StyleFn.html"
source_file: "src/iced_widget/checkbox.rs"
source_lines: "568"
---

# `iced_widget::checkbox::StyleFn`

```rust
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;
```

A styling function for a [`Checkbox`](struct.Checkbox.md).

This is just a boxed closure: `Fn(&Theme, Status) -> Style`.

**Source:** [checkbox.rs](../../src/iced_widget/checkbox.rs.md) (lines 568)
