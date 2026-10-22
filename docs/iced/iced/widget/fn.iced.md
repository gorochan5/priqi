---
title: "iced"
kind: function
path: "iced::widget::iced"
crate: "iced"
module: "iced::widget"
file: "iced/widget/fn.iced.md"
docs_url: "https://docs.iced.rs/iced/widget/fn.iced.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "1505-1513"
---

# `iced::widget::iced`

```rust
pub fn iced<'a, Message, Theme, Renderer>(
    text_size: impl Into<Pixels>,
) -> impl Widget<Message, Theme, Renderer> where
    Message: 'a,
    Renderer: Renderer + 'a,
    Theme: Catalog + Catalog + 'a,
    <Theme as Catalog>::Class<'a>: From<Box<dyn Fn(&Theme) -> Style + 'a>>,
    <Theme as Catalog>::Class<'a>: From<Box<dyn Fn(&Theme) -> Style + 'a>>,
```

Creates an [`Element`](../advanced/widget/struct.Element.md) that displays the iced logo with the given `text_size`.

Useful for showing some love to your favorite GUI library in your “About” screen, for instance.

**Source:** [helpers.rs](../../src/iced_widget/helpers.rs.md) (lines 1505-1513)
