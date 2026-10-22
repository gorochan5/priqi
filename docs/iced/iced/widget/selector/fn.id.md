---
title: "id"
kind: function
path: "iced::widget::selector::id"
crate: "iced"
module: "iced::widget::selector"
file: "iced/widget/selector/fn.id.md"
docs_url: "https://docs.iced.rs/iced/widget/selector/fn.id.html"
---

# `iced::widget::selector::id`

```rust
pub fn id(id: impl Into<Id>) -> impl Selector<Output = Target>
```

Creates a new [`Selector`](trait.Selector.md) that matches widgets with the given [`widget::Id`](../struct.Id.md).
