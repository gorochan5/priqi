---
title: "draw"
kind: function
path: "iced_widget::text::draw"
crate: "iced_widget"
module: "iced_widget::text"
file: "iced_widget/text/fn.draw.md"
docs_url: "https://docs.iced.rs/iced_widget/text/fn.draw.html"
---

# `iced_widget::text::draw`

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

Draws text using the same logic as the [`Text`] widget.
