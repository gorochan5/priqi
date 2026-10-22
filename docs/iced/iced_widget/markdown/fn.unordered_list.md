---
title: "unordered_list"
kind: function
path: "iced_widget::markdown::unordered_list"
crate: "iced_widget"
module: "iced_widget::markdown"
file: "iced_widget/markdown/fn.unordered_list.md"
docs_url: "https://docs.iced.rs/iced_widget/markdown/fn.unordered_list.html"
source_file: "src/iced_widget/markdown.rs"
source_lines: "1909-1947"
---

# `iced_widget::markdown::unordered_list`

```rust
pub fn unordered_list<'a, Message, Theme, Renderer>(
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
    settings: Settings,
    bullets: &'a [Bullet],
) -> impl Widget<Message, Theme, Renderer> + 'awhere
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: Renderer + 'a,
```

Displays an unordered list using the default look and calling the [`Viewer`](trait.Viewer.md) for each bullet point item.

**Source:** [markdown.rs](../../src/iced_widget/markdown.rs.md) (lines 1909-1947)
