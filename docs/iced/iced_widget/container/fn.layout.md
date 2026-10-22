---
title: "layout"
kind: function
path: "iced_widget::container::layout"
crate: "iced_widget"
module: "iced_widget::container"
file: "iced_widget/container/fn.layout.md"
docs_url: "https://docs.iced.rs/iced_widget/container/fn.layout.html"
source_file: "src/iced_widget/container.rs"
source_lines: "347-374"
---

# `iced_widget::container::layout`

```rust
pub fn layout(
    tree: &mut Tree,
    limits: &Limits,
    width: Length,
    height: Length,
    padding: Padding,
    horizontal_alignment: Horizontal,
    vertical_alignment: Vertical,
    layout_content: impl FnOnce(&mut Tree, &Limits) -> Size,
)
```

Computes the layout of a [`Container`](struct.Container.md).

**Source:** [container.rs](../../src/iced_widget/container.rs.md) (lines 347-374)
