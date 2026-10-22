---
title: "move_cursor_to"
kind: function
path: "iced_runtime::widget::operation::text_input::move_cursor_to"
crate: "iced_runtime"
module: "iced_runtime::widget::operation::text_input"
file: "iced_runtime/widget/operation/text_input/fn.move_cursor_to.md"
docs_url: "https://docs.iced.rs/iced_runtime/widget/operation/text_input/fn.move_cursor_to.html"
source_file: "src/iced_runtime/widget/operation.rs"
source_lines: "146-151"
---

# `iced_runtime::widget::operation::text_input::move_cursor_to`

```rust
pub fn move_cursor_to<T>(id: impl Into<Id>, position: Position) -> Task<T>
```

Moves the cursor of the widget with the given [`Id`] to the provided position.

**Source:** [operation.rs](../../../../src/iced_runtime/widget/operation.rs.md) (lines 146-151)
