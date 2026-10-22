---
title: "layout"
kind: function
path: "iced_widget::text::layout"
crate: "iced_widget"
module: "iced_widget::text"
file: "iced_widget/text/fn.layout.md"
docs_url: "https://docs.iced.rs/iced_widget/text/fn.layout.html"
---

# `iced_widget::text::layout`

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

Computes the [`Size`] of a [`Text`] widget.
