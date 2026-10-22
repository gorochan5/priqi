---
title: "StyleFn"
kind: type
path: "iced_widget::scrollable::StyleFn"
crate: "iced_widget"
module: "iced_widget::scrollable"
file: "iced_widget/scrollable/type.StyleFn.md"
docs_url: "https://docs.iced.rs/iced_widget/scrollable/type.StyleFn.html"
source_file: "src/iced_widget/scrollable.rs"
source_lines: "3543"
---

# `iced_widget::scrollable::StyleFn`

```rust
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;
```

A styling function for a [`Scrollable`](struct.Scrollable.md).

**Source:** [scrollable.rs](../../src/iced_widget/scrollable.rs.md) (lines 3543)
