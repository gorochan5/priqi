---
title: "diff_children"
kind: function
path: "iced_core::widget::tree::diff_children"
crate: "iced_core"
module: "iced_core::widget::tree"
file: "iced_core/widget/tree/fn.diff_children.md"
docs_url: "https://docs.iced.rs/iced_core/widget/tree/fn.diff_children.html"
source_file: "src/iced_core/widget/tree.rs"
source_lines: "98-107"
---

# `iced_core::widget::tree::diff_children`

```rust
pub fn diff_children<Message, Theme, Renderer>(
    old_children: &mut Vec<Tree>,
    new_children: &mut [impl Widget<Message, Theme, Renderer>],
)where
    Renderer: Renderer,
```

Reconciles the children of the tree with the provided list of widgets.

**Source:** [tree.rs](../../../src/iced_core/widget/tree.rs.md) (lines 98-107)
