---
title: "id"
kind: function
path: "iced_runtime::widget::selector::id"
crate: "iced_runtime"
module: "iced_runtime::widget::selector"
file: "iced_runtime/widget/selector/fn.id.md"
docs_url: "https://docs.iced.rs/iced_runtime/widget/selector/fn.id.html"
---

# `iced_runtime::widget::selector::id`

```rust
pub fn id(id: impl Into<Id>) -> impl Selector<Output = Target>
```

Creates a new [`Selector`](trait.Selector.md) that matches widgets with the given [`widget::Id`].
