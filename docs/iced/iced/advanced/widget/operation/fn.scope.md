---
title: "scope"
kind: function
path: "iced::advanced::widget::operation::scope"
crate: "iced"
module: "iced::advanced::widget::operation"
file: "iced/advanced/widget/operation/fn.scope.md"
docs_url: "https://docs.iced.rs/iced/advanced/widget/operation/fn.scope.html"
---

# `iced::advanced::widget::operation::scope`

```rust
pub fn scope<T>(
    target: Id,
    operation: impl Operation<T> + 'static,
) -> impl Operation<T> where
    T: 'static,
```

Produces an [`Operation`](../trait.Operation.md) that applies the given [`Operation`](../trait.Operation.md) to the children of a container with the given [`Id`](../../../widget/struct.Id.md).
