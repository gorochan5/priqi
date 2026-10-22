---
title: "StyleFn"
kind: type
path: "iced_widget::text_input::StyleFn"
crate: "iced_widget"
module: "iced_widget::text_input"
file: "iced_widget/text_input/type.StyleFn.md"
docs_url: "https://docs.iced.rs/iced_widget/text_input/type.StyleFn.html"
source_file: "src/iced_widget/text_input.rs"
source_lines: "565"
---

# `iced_widget::text_input::StyleFn`

```rust
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;
```

A styling function for a [`TextInput`](struct.TextInput.md).

This is just a boxed closure: `Fn(&Theme, Status) -> Style`.

**Source:** [text_input.rs](../../src/iced_widget/text_input.rs.md) (lines 565)
