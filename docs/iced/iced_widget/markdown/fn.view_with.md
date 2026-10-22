---
title: "view_with"
kind: function
path: "iced_widget::markdown::view_with"
crate: "iced_widget"
module: "iced_widget::markdown"
file: "iced_widget/markdown/fn.view_with.md"
docs_url: "https://docs.iced.rs/iced_widget/markdown/fn.view_with.html"
source_file: "src/iced_widget/markdown.rs"
source_lines: "1783-1794"
---

# `iced_widget::markdown::view_with`

```rust
pub fn view_with<'a, Message, Theme, Renderer>(
    items: &'a [Item],
    settings: impl Into<Settings>,
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
) -> impl Widget<Message, Theme, Renderer> + 'awhere
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

Runs [`view`](../fn.markdown.md) but with a custom [`Viewer`](trait.Viewer.md) to turn an [`Item`](enum.Item.md) into a widget.

This is useful if you want to customize the look of certain Markdown elements.

**Source:** [markdown.rs](../../src/iced_widget/markdown.rs.md) (lines 1783-1794)
