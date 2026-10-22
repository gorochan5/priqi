---
title: "StyleFn"
kind: type
path: "iced_widget::radio::StyleFn"
crate: "iced_widget"
module: "iced_widget::radio"
file: "iced_widget/radio/type.StyleFn.md"
docs_url: "https://docs.iced.rs/iced_widget/radio/type.StyleFn.html"
source_file: "src/iced_widget/radio.rs"
source_lines: "496"
---

# `iced_widget::radio::StyleFn`

```rust
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;
```

A styling function for a [`Radio`](struct.Radio.md).

**Source:** [radio.rs](../../src/iced_widget/radio.rs.md) (lines 496)
