---
title: "diff_children_custom"
kind: function
path: "iced_core::widget::tree::diff_children_custom"
crate: "iced_core"
module: "iced_core::widget::tree"
file: "iced_core/widget/tree/fn.diff_children_custom.md"
docs_url: "https://docs.iced.rs/iced_core/widget/tree/fn.diff_children_custom.html"
source_file: "src/iced_core/widget/tree.rs"
source_lines: "111-128"
---

# `iced_core::widget::tree::diff_children_custom`

```rust
pub fn diff_children_custom<T>(
    old_children: &mut Vec<Tree>,
    new_children: &mut [T],
    diff: impl Fn(&mut Tree, &mut T),
    new_state: impl Fn(&T) -> Tree,
)
```

Reconciles the children of the tree with the provided list of widgets using custom logic both for diffing and creating new widget state.

**Source:** [tree.rs](../../../src/iced_core/widget/tree.rs.md) (lines 111-128)
