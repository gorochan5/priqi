---
title: "items"
kind: function
path: "iced_widget::markdown::items"
crate: "iced_widget"
module: "iced_widget::markdown"
file: "iced_widget/markdown/fn.items.md"
docs_url: "https://docs.iced.rs/iced_widget/markdown/fn.items.html"
source_file: "src/iced_widget/markdown.rs"
source_lines: "2101-2126"
---

# `iced_widget::markdown::items`

```rust
pub fn items<'a, Message, Theme, Renderer>(
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
    settings: Settings,
    items: &'a [Item],
) -> impl Widget<Message, Theme, Renderer> + 'awhere
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: Renderer + 'a,
```

Displays a column of items with the default look.

**Source:** [markdown.rs](../../src/iced_widget/markdown.rs.md) (lines 2101-2126)
