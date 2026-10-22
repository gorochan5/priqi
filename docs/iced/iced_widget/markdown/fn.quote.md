---
title: "quote"
kind: function
path: "iced_widget::markdown::quote"
crate: "iced_widget"
module: "iced_widget::markdown"
file: "iced_widget/markdown/fn.quote.md"
docs_url: "https://docs.iced.rs/iced_widget/markdown/fn.quote.html"
source_file: "src/iced_widget/markdown.rs"
source_lines: "2020-2041"
---

# `iced_widget::markdown::quote`

```rust
pub fn quote<'a, Message, Theme, Renderer>(
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
    settings: Settings,
    contents: &'a [Item],
) -> impl Widget<Message, Theme, Renderer> + 'awhere
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: Renderer + 'a,
```

Displays a quote using the default look.

**Source:** [markdown.rs](../../src/iced_widget/markdown.rs.md) (lines 2020-2041)
