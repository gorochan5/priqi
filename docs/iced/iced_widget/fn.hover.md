---
title: "hover"
kind: function
path: "iced_widget::hover"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.hover.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.hover.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "670-672"
---

# `iced_widget::hover`

```rust
pub fn hover<W, V>(base: W, top: V) -> Hover<W, V>
```

Displays a widget on top of another one, only when the base widget is hovered.

This works analogously to a [`stack`](fn.stack.md), but it will only display the layer on top when the cursor is over the base. It can be useful for removing visual clutter.

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 670-672)
