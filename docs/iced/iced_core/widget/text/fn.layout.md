---
title: "layout"
kind: function
path: "iced_core::widget::text::layout"
crate: "iced_core"
module: "iced_core::widget::text"
file: "iced_core/widget/text/fn.layout.md"
docs_url: "https://docs.iced.rs/iced_core/widget/text/fn.layout.html"
source_file: "src/iced_core/widget/text.rs"
source_lines: "517-550"
---

# `iced_core::widget::text::layout`

```rust
pub fn layout<Renderer>(
    paragraph: &mut Plain<Renderer::Paragraph>,
    renderer: &Renderer,
    limits: &Limits,
    content: &str,
    format: Format,
) -> Sizewhere
    Renderer: Renderer,
```

Computes the [`Size`](../../struct.Size.md) of a [`Text`](struct.Text.md) widget.

**Source:** [text.rs](../../../src/iced_core/widget/text.rs.md) (lines 517-550)
