---
title: "diff_children_custom_with_search"
kind: function
path: "iced_core::widget::tree::diff_children_custom_with_search"
crate: "iced_core"
module: "iced_core::widget::tree"
file: "iced_core/widget/tree/fn.diff_children_custom_with_search.md"
docs_url: "https://docs.iced.rs/iced_core/widget/tree/fn.diff_children_custom_with_search.html"
source_file: "src/iced_core/widget/tree.rs"
source_lines: "135-209"
---

# `iced_core::widget::tree::diff_children_custom_with_search`

```rust
pub fn diff_children_custom_with_search<T>(
    current_children: &mut Vec<Tree>,
    new_children: &mut [T],
    diff: impl Fn(&mut Tree, &mut T),
    maybe_changed: impl Fn(usize) -> bool,
    new_state: impl Fn(&T) -> Tree,
)
```

Reconciles the `current_children` with the provided list of widgets using custom logic both for diffing and creating new widget state.

The algorithm will try to minimize the impact of diffing by querying the `maybe_changed` closure.

**Source:** [tree.rs](../../../src/iced_core/widget/tree.rs.md) (lines 135-209)
