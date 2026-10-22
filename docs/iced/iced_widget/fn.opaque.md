---
title: "opaque"
kind: function
path: "iced_widget::opaque"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.opaque.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.opaque.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "534-662"
---

# `iced_widget::opaque`

```rust
pub fn opaque<'a, Message, Theme, Renderer>(
    content: impl Widget<Message, Theme, Renderer> + 'a,
) -> impl Widget<Message, Theme, Renderer> + 'awhere
    Message: 'a,
    Theme: 'a,
    Renderer: Renderer + 'a,
```

Wraps the given widget and captures any mouse button presses inside the bounds of the widget—effectively making it *opaque*.

This helper is meant to be used to mark elements in a `Stack` to avoid mouse events from passing through layers.

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 534-662)
