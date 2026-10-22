---
title: "paragraph"
kind: function
path: "iced_widget::markdown::paragraph"
crate: "iced_widget"
module: "iced_widget::markdown"
file: "iced_widget/markdown/fn.paragraph.md"
docs_url: "https://docs.iced.rs/iced_widget/markdown/fn.paragraph.html"
source_file: "src/iced_widget/markdown.rs"
source_lines: "1889-1905"
---

# `iced_widget::markdown::paragraph`

```rust
pub fn paragraph<'a, Message, Theme, Renderer>(
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
    settings: Settings,
    text: &Text,
    on_link_click: impl Fn(Uri) -> Message + 'a,
) -> impl Widget<Message, Theme, Renderer> + 'awhere
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: Renderer + 'a,
```

Displays a paragraph using the default look.

**Source:** [markdown.rs](../../src/iced_widget/markdown.rs.md) (lines 1889-1905)
