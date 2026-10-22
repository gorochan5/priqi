---
title: "draw"
kind: function
path: "iced_core::widget::text::draw"
crate: "iced_core"
module: "iced_core::widget::text"
file: "iced_core/widget/text/fn.draw.md"
docs_url: "https://docs.iced.rs/iced_core/widget/text/fn.draw.html"
source_file: "src/iced_core/widget/text.rs"
source_lines: "553-592"
---

# `iced_core::widget::text::draw`

```rust
pub fn draw<Renderer>(
    renderer: &mut Renderer,
    style: &Style,
    bounds: Rectangle,
    paragraph: &Renderer::Paragraph,
    appearance: Style,
    selection_color: Color,
    viewport: &Rectangle,
)where
    Renderer: Renderer,
```

Draws text using the same logic as the [`Text`](struct.Text.md) widget.

**Source:** [text.rs](../../../src/iced_core/widget/text.rs.md) (lines 553-592)
