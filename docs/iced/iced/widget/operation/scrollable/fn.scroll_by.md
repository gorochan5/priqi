---
title: "scroll_by"
kind: function
path: "iced::widget::operation::scrollable::scroll_by"
crate: "iced"
module: "iced::widget::operation::scrollable"
file: "iced/widget/operation/scrollable/fn.scroll_by.md"
docs_url: "https://docs.iced.rs/iced/widget/operation/scrollable/fn.scroll_by.html"
---

# `iced::widget::operation::scrollable::scroll_by`

```rust
pub fn scroll_by<T>(
    id: impl Into<Id>,
    offset: AbsoluteOffset,
    animation: Animation,
) -> Task<T>
```

Scrolls the scrollable with the given [`Id`](../../struct.Id.md) by the provided [`AbsoluteOffset`](../../scrollable/struct.AbsoluteOffset.md).
