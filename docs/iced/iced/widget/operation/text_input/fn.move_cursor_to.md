---
title: "move_cursor_to"
kind: function
path: "iced::widget::operation::text_input::move_cursor_to"
crate: "iced"
module: "iced::widget::operation::text_input"
file: "iced/widget/operation/text_input/fn.move_cursor_to.md"
docs_url: "https://docs.iced.rs/iced/widget/operation/text_input/fn.move_cursor_to.html"
---

# `iced::widget::operation::text_input::move_cursor_to`

```rust
pub fn move_cursor_to<T>(id: impl Into<Id>, position: Position) -> Task<T>
```

Moves the cursor of the widget with the given [`Id`](../../struct.Id.md) to the provided position.
