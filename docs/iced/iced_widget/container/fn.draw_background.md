---
title: "draw_background"
kind: function
path: "iced_widget::container::draw_background"
crate: "iced_widget"
module: "iced_widget::container"
file: "iced_widget/container/fn.draw_background.md"
docs_url: "https://docs.iced.rs/iced_widget/container/fn.draw_background.html"
source_file: "src/iced_widget/container.rs"
source_lines: "377-394"
---

# `iced_widget::container::draw_background`

```rust
pub fn draw_background<Renderer>(
    renderer: &mut Renderer,
    style: &Style,
    bounds: Rectangle,
)where
    Renderer: Renderer,
```

Draws the background of a [`Container`](struct.Container.md) given its [`Style`](struct.Style.md) and its `bounds`.

**Source:** [container.rs](../../src/iced_widget/container.rs.md) (lines 377-394)
