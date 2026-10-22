---
title: "focus_next"
kind: function
path: "iced_core::widget::operation::focusable::focus_next"
crate: "iced_core"
module: "iced_core::widget::operation::focusable"
file: "iced_core/widget/operation/focusable/fn.focus_next.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/focusable/fn.focus_next.html"
source_file: "src/iced_core/widget/operation/focusable.rs"
source_lines: "142-169"
---

# `iced_core::widget::operation::focusable::focus_next`

```rust
pub fn focus_next<T>() -> impl Operation<T> where
    T: Send + 'static,
```

Produces an [`Operation`](../trait.Operation.md) that searches for the current focused widget, and

- if found, focuses the next focusable widget.
- if not found, focuses the first focusable widget.

**Source:** [focusable.rs](../../../../src/iced_core/widget/operation/focusable.rs.md) (lines 142-169)
