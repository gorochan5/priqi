---
title: "move_cursor_to"
kind: function
path: "iced_core::widget::operation::text_input::move_cursor_to"
crate: "iced_core"
module: "iced_core::widget::operation::text_input"
file: "iced_core/widget/operation/text_input/fn.move_cursor_to.md"
docs_url: "https://docs.iced.rs/iced_core/widget/operation/text_input/fn.move_cursor_to.html"
source_file: "src/iced_core/widget/operation/text_input.rs"
source_lines: "82-104"
---

# `iced_core::widget::operation::text_input::move_cursor_to`

```rust
pub fn move_cursor_to<T>(target: Id, position: Position) -> impl Operation<T>
```

Produces an [`Operation`](../trait.Operation.md) that moves the cursor of the widget with the given [`Id`](../../struct.Id.md) to the provided position.

**Source:** [text_input.rs](../../../../src/iced_core/widget/operation/text_input.rs.md) (lines 82-104)
