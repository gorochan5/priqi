---
title: "StyleFn"
kind: type
path: "iced_widget::text_editor::StyleFn"
crate: "iced_widget"
module: "iced_widget::text_editor"
file: "iced_widget/text_editor/type.StyleFn.md"
docs_url: "https://docs.iced.rs/iced_widget/text_editor/type.StyleFn.html"
source_file: "src/iced_widget/text_editor.rs"
source_lines: "781"
---

# `iced_widget::text_editor::StyleFn`

```rust
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;
```

A styling function for a [`TextEditor`](struct.TextEditor.md).

**Source:** [text_editor.rs](../../src/iced_widget/text_editor.rs.md) (lines 781)
