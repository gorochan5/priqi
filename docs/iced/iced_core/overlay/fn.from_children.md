---
title: "from_children"
kind: function
path: "iced_core::overlay::from_children"
crate: "iced_core"
module: "iced_core::overlay"
file: "iced_core/overlay/fn.from_children.md"
docs_url: "https://docs.iced.rs/iced_core/overlay/fn.from_children.html"
source_file: "src/iced_core/overlay.rs"
source_lines: "81-100"
---

# `iced_core::overlay::from_children`

```rust
pub fn from_children<'a, Message, Theme, Renderer>(
    children: &'a mut [impl Widget<Message, Theme, Renderer>],
    tree: &'a mut Tree,
    layout: Layout,
    renderer: &Renderer,
    viewport: &Rectangle,
    translation: Vector,
    window: Size,
) -> Vec<Element<'a, Message, Theme, Renderer>> where
    Renderer: Renderer,
```

Returns the overlays of the given [`Widget`](../widget/trait.Widget.md) children.

This method will generally only be used by advanced users that are implementing the [`Widget`](../widget/trait.Widget.md) trait.

**Source:** [overlay.rs](../../src/iced_core/overlay.rs.md) (lines 81-100)
