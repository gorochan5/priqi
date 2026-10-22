---
title: "focus_previous"
kind: function
path: "iced_core::widget::operation::focusable::focus_previous"
crate: "iced_core"
module: "iced_core::widget::operation::focusable"
file: "iced_core/widget/operation/focusable/fn.focus_previous.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/focusable/fn.focus_previous.html"
source_file: "src/iced_core/widget/operation/focusable.rs"
source_lines: "104-137"
---

# `iced_core::widget::operation::focusable::focus_previous`

```rust
pub fn focus_previous<T>() -> impl Operation<T> where
    T: Send + 'static,
```

Produces an [`Operation`](../trait.Operation.md) that searches for the current focused widget, and

- if found, focuses the previous focusable widget.
- if not found, focuses the last focusable widget.

**Source:** [focusable.rs](../../../../src/iced_core/widget/operation/focusable.rs.md) (lines 104-137)
