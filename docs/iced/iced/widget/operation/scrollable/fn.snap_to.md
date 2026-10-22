---
title: "snap_to"
kind: function
path: "iced::widget::operation::scrollable::snap_to"
crate: "iced"
module: "iced::widget::operation::scrollable"
file: "iced/widget/operation/scrollable/fn.snap_to.md"
docs_url: "https://docs.iced.rs/iced/widget/operation/scrollable/fn.snap_to.html"
---

# `iced::widget::operation::scrollable::snap_to`

```rust
pub fn snap_to<T>(
    id: impl Into<Id>,
    offset: impl Into<RelativeOffset<Option<f32>>>,
    animation: Animation,
) -> Task<T>
```

Snaps the scrollable with the given [`Id`](../../struct.Id.md) to the provided [`RelativeOffset`](../../scrollable/struct.RelativeOffset.md).
