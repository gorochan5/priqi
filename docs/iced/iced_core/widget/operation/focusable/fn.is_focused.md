---
title: "is_focused"
kind: function
path: "iced_core::widget::operation::focusable::is_focused"
crate: "iced_core"
module: "iced_core::widget::operation::focusable"
file: "iced_core/widget/operation/focusable/fn.is_focused.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/focusable/fn.is_focused.html"
source_file: "src/iced_core/widget/operation/focusable.rs"
source_lines: "204-234"
---

# `iced_core::widget::operation::focusable::is_focused`

```rust
pub fn is_focused(target: Id) -> impl Operation<bool>
```

Produces an [`Operation`](../trait.Operation.md) that searches for the focusable widget and stores whether it is focused or not. This ignores widgets that do not have an ID.

**Source:** [focusable.rs](../../../../src/iced_core/widget/operation/focusable.rs.md) (lines 204-234)
