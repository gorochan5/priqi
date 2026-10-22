---
title: "item"
kind: function
path: "iced_widget::markdown::item"
crate: "iced_widget"
module: "iced_widget::markdown"
file: "iced_widget/markdown/fn.item.md"
docs_url: "https://docs.iced.rs/iced_widget/markdown/fn.item.html"
source_file: "src/iced_widget/markdown.rs"
source_lines: "1797-1830"
---

# `iced_widget::markdown::item`

```rust
pub fn item<'a, Message, Theme, Renderer>(
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
    settings: Settings,
    item: &'a Item,
) -> impl Widget<Message, Theme, Renderer> + 'awhere
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: Renderer + 'a,
```

Displays an [`Item`](enum.Item.md) using the given [`Viewer`](trait.Viewer.md).

**Source:** [markdown.rs](../../src/iced_widget/markdown.rs.md) (lines 1797-1830)
