---
title: "find_focused"
kind: function
path: "iced_core::widget::operation::focusable::find_focused"
crate: "iced_core"
module: "iced_core::widget::operation::focusable"
file: "iced_core/widget/operation/focusable/fn.find_focused.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/focusable/fn.find_focused.html"
source_file: "src/iced_core/widget/operation/focusable.rs"
source_lines: "173-199"
---

# `iced_core::widget::operation::focusable::find_focused`

```rust
pub fn find_focused() -> impl Operation<Id>
```

Produces an [`Operation`](../trait.Operation.md) that searches for the current focused widget and stores its ID. This ignores widgets that do not have an ID.

**Source:** [focusable.rs](../../../../src/iced_core/widget/operation/focusable.rs.md) (lines 173-199)
