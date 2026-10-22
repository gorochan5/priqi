---
title: "code_block"
kind: function
path: "iced::widget::markdown::code_block"
crate: "iced"
module: "iced::widget::markdown"
file: "iced/widget/markdown/fn.code_block.md"
docs_url: "https://docs.iced.rs/iced/widget/markdown/fn.code_block.html"
source_file: "src/iced_widget/markdown.rs"
source_lines: "1985-1994"
---

# `iced::widget::markdown::code_block`

```rust
pub fn code_block<'a, Message, Theme, Renderer>(
    viewer: &impl Viewer<'a, Message, Theme, Renderer>,
    settings: Settings,
    lines: &'a [Text],
    on_link_click: impl Fn(String) -> Message + Clone + 'a,
) -> impl Widget<Message, Theme, Renderer> + 'awhere
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: Renderer + 'a,
```

Displays a code block using the default look.

**Source:** [markdown.rs](../../../src/iced_widget/markdown.rs.md) (lines 1985-1994)
