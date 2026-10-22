---
title: "select_range"
kind: function
path: "iced::widget::operation::text_input::select_range"
crate: "iced"
module: "iced::widget::operation::text_input"
file: "iced/widget/operation/text_input/fn.select_range.md"
docs_url: "https://docs.iced.rs/iced/widget/operation/text_input/fn.select_range.html"
---

# `iced::widget::operation::text_input::select_range`

```rust
pub fn select_range<T>(
    id: impl Into<Id>,
    start: Position,
    end: Position,
) -> Task<T>
```

Selects the given content range of the widget with the given [`Id`](../../struct.Id.md).
