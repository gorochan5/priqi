---
title: "find_all"
kind: function
path: "iced::widget::selector::find_all"
crate: "iced"
module: "iced::widget::selector"
file: "iced/widget/selector/fn.find_all.md"
docs_url: "https://docs.iced.rs/iced/widget/selector/fn.find_all.html"
---

# `iced::widget::selector::find_all`

```rust
pub fn find_all<S>(selector: S) -> Task<Vec<<S as Selector>::Output>> where
    S: Selector + Send + 'static,
    <S as Selector>::Output: Send + Clone + 'static,
```

Finds all widgets matching the given [`Selector`](trait.Selector.md).
