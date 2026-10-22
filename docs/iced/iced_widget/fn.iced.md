---
title: "iced"
kind: function
path: "iced_widget::iced"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.iced.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.iced.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "1505-1545"
---

# `iced_widget::iced`

```rust
pub fn iced<'a, Message, Theme, Renderer>(
    text_size: impl Into<Pixels>,
) -> impl Widget<Message, Theme, Renderer> where
    Message: 'a,
    Renderer: Renderer + 'a,
    Theme: Catalog + Catalog + 'a,
    <Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme>>,
    <Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme>>,
```

Creates an [`Element`] that displays the iced logo with the given `text_size`.

Useful for showing some love to your favorite GUI library in your “About” screen, for instance.

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 1505-1545)
