---
title: "heading"
kind: function
path: "iced_widget::markdown::heading"
crate: "iced_widget"
module: "iced_widget::markdown"
file: "iced_widget/markdown/fn.heading.md"
docs_url: "https://docs.iced.rs/iced_widget/markdown/fn.heading.html"
source_file: "src/iced_widget/markdown.rs"
source_lines: "1833-1886"
---

# `iced_widget::markdown::heading`

```rust
pub fn heading<'a, Message, Theme, Renderer>(
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
    settings: Settings,
    level: &'a HeadingLevel,
    text: &'a Text,
    on_link_click: impl Fn(Uri) -> Message + 'a,
) -> impl Widget<Message, Theme, Renderer> + 'awhere
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: Renderer + 'a,
```

Displays a heading using the default look.

**Source:** [markdown.rs](../../src/iced_widget/markdown.rs.md) (lines 1833-1886)
