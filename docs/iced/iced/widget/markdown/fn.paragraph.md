---
title: "paragraph"
kind: function
path: "iced::widget::markdown::paragraph"
crate: "iced"
module: "iced::widget::markdown"
file: "iced/widget/markdown/fn.paragraph.md"
docs_url: "https://docs.iced.rs/iced/widget/markdown/fn.paragraph.html"
source_file: "src/iced_widget/markdown.rs"
source_lines: "1889-1898"
---

# `iced::widget::markdown::paragraph`

```rust
pub fn paragraph<'a, Message, Theme, Renderer>(
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
    settings: Settings,
    text: &Text,
    on_link_click: impl Fn(String) -> Message + 'a,
) -> impl Widget<Message, Theme, Renderer> + 'awhere
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: Renderer + 'a,
```

Displays a paragraph using the default look.

**Source:** [markdown.rs](../../../src/iced_widget/markdown.rs.md) (lines 1889-1898)
