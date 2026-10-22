---
title: "find"
kind: function
path: "iced::widget::selector::find"
crate: "iced"
module: "iced::widget::selector"
file: "iced/widget/selector/fn.find.md"
docs_url: "https://docs.iced.rs/iced/widget/selector/fn.find.html"
---

# `iced::widget::selector::find`

```rust
pub fn find<S>(selector: S) -> Task<Option<<S as Selector>::Output>> where
    S: Selector + Send + 'static,
    <S as Selector>::Output: Send + Clone + 'static,
```

Finds a widget matching the given [`Selector`](trait.Selector.md).
