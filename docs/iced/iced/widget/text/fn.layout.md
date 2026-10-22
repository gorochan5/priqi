---
title: "layout"
kind: function
path: "iced::widget::text::layout"
crate: "iced"
module: "iced::widget::text"
file: "iced/widget/text/fn.layout.md"
docs_url: "https://docs.iced.rs/iced/widget/text/fn.layout.html"
---

# `iced::widget::text::layout`

```rust
pub fn layout<Renderer>(
    paragraph: &mut Plain<<Renderer as Renderer>::Paragraph>,
    renderer: &Renderer,
    limits: &Limits,
    content: &str,
    format: Format,
) -> Sizewhere
    Renderer: Renderer,
```

Computes the [`Size`](../../struct.Size.md) of a [`Text`](../../advanced/widget/struct.Text.md) widget.
