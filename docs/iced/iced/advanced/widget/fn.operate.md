---
title: "operate"
kind: function
path: "iced::advanced::widget::operate"
crate: "iced"
module: "iced::advanced::widget"
file: "iced/advanced/widget/fn.operate.md"
docs_url: "https://docs.iced.rs/iced/advanced/widget/fn.operate.html"
---

# `iced::advanced::widget::operate`

```rust
pub fn operate<T>(operation: impl Operation<T> + 'static) -> Task<T> where
    T: Send + 'static,
```

Creates a new [`Task`](../../struct.Task.md) that runs the given [`widget::Operation`](trait.Operation.md) and produces its output.
