---
title: "ordered_list"
kind: function
path: "iced_widget::markdown::ordered_list"
crate: "iced_widget"
module: "iced_widget::markdown"
file: "iced_widget/markdown/fn.ordered_list.md"
docs_url: "https://docs.iced.rs/iced_widget/markdown/fn.ordered_list.html"
source_file: "src/iced_widget/markdown.rs"
source_lines: "1951-1982"
---

# `iced_widget::markdown::ordered_list`

```rust
pub fn ordered_list<'a, Message, Theme, Renderer>(
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
    settings: Settings,
    start: u64,
    bullets: &'a [Bullet],
) -> impl Widget<Message, Theme, Renderer> + 'awhere
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: Renderer + 'a,
```

Displays an ordered list using the default look and calling the [`Viewer`](trait.Viewer.md) for each numbered item.

**Source:** [markdown.rs](../../src/iced_widget/markdown.rs.md) (lines 1951-1982)
