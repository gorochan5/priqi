---
title: "table"
kind: function
path: "iced_widget::markdown::table"
crate: "iced_widget"
module: "iced_widget::markdown"
file: "iced_widget/markdown/fn.table.md"
docs_url: "https://docs.iced.rs/iced_widget/markdown/fn.table.html"
source_file: "src/iced_widget/markdown.rs"
source_lines: "2054-2098"
---

# `iced_widget::markdown::table`

```rust
pub fn table<'a, Message, Theme, Renderer>(
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
    settings: Settings,
    columns: &'a [Column],
    rows: &'a [Row],
) -> impl Widget<Message, Theme, Renderer> + 'awhere
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: Renderer + 'a,
```

Displays a table using the default look.

**Source:** [markdown.rs](../../src/iced_widget/markdown.rs.md) (lines 2054-2098)
