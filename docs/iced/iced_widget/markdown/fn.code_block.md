---
title: "code_block"
kind: function
path: "iced_widget::markdown::code_block"
crate: "iced_widget"
module: "iced_widget::markdown"
file: "iced_widget/markdown/fn.code_block.md"
docs_url: "https://docs.iced.rs/iced_widget/markdown/fn.code_block.html"
source_file: "src/iced_widget/markdown.rs"
source_lines: "1985-2017"
---

# `iced_widget::markdown::code_block`

```rust
pub fn code_block<'a, Message, Theme, Renderer>(
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
    settings: Settings,
    lines: &'a [Text],
    on_link_click: impl Fn(Uri) -> Message + Clone + 'a,
) -> impl Widget<Message, Theme, Renderer> + 'awhere
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: Renderer + 'a,
```

Displays a code block using the default look.

**Source:** [markdown.rs](../../src/iced_widget/markdown.rs.md) (lines 1985-2017)
