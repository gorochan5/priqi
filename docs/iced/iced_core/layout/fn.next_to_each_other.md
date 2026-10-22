---
title: "next_to_each_other"
kind: function
path: "iced_core::layout::next_to_each_other"
crate: "iced_core"
module: "iced_core::layout"
file: "iced_core/layout/fn.next_to_each_other.md"
docs_url: "https://docs.iced.rs/iced_core/layout/fn.next_to_each_other.html"
source_file: "src/iced_core/layout.rs"
source_lines: "96-113"
---

# `iced_core::layout::next_to_each_other`

```rust
pub fn next_to_each_other(
    tree: &mut Tree,
    left: Size,
    right: Size,
    spacing: f32,
)
```

Lays out two children side by side in the provided [`widget::Tree`](../widget/tree/struct.Tree.md).

The tree must have exactly two children. The first is placed at the left edge, and the second to its right after `spacing`, both vertically centered with respect to each other.

The resulting size is stored in the tree, along with the children’s sizes and translations.

**Source:** [layout.rs](../../src/iced_core/layout.rs.md) (lines 96-113)
