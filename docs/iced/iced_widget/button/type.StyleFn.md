---
title: "StyleFn"
kind: type
path: "iced_widget::button::StyleFn"
crate: "iced_widget"
module: "iced_widget::button"
file: "iced_widget/button/type.StyleFn.md"
docs_url: "https://docs.iced.rs/iced_widget/button/type.StyleFn.html"
source_file: "src/iced_widget/button.rs"
source_lines: "563"
---

# `iced_widget::button::StyleFn`

```rust
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;
```

A styling function for a [`Button`](struct.Button.md).

**Source:** [button.rs](../../src/iced_widget/button.rs.md) (lines 563)
