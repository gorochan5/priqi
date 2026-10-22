---
title: "StyleFn"
kind: type
path: "iced_core::widget::text::StyleFn"
crate: "iced_core"
module: "iced_core::widget::text"
file: "iced_core/widget/text/type.StyleFn.md"
docs_url: "https://docs.iced.rs/iced_core/widget/text/type.StyleFn.html"
source_file: "src/iced_core/widget/text.rs"
source_lines: "635"
---

# `iced_core::widget::text::StyleFn`

```rust
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;
```

A styling function for a [`Text`](struct.Text.md).

This is just a boxed closure: `Fn(&Theme, Status) -> Style`.

**Source:** [text.rs](../../../src/iced_core/widget/text.rs.md) (lines 635)
