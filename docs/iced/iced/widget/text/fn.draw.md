---
title: "draw"
kind: function
path: "iced::widget::text::draw"
crate: "iced"
module: "iced::widget::text"
file: "iced/widget/text/fn.draw.md"
docs_url: "https://docs.iced.rs/iced/widget/text/fn.draw.html"
---

# `iced::widget::text::draw`

```rust
pub fn draw<Renderer>(
    renderer: &mut Renderer,
    style: &Style,
    bounds: Rectangle,
    paragraph: &<Renderer as Renderer>::Paragraph,
    appearance: Style,
    selection_color: Color,
    viewport: &Rectangle,
)where
    Renderer: Renderer,
```

Draws text using the same logic as the [`Text`](../../advanced/widget/struct.Text.md) widget.
