---
title: "StyleFn"
kind: type
path: "iced_widget::text::StyleFn"
crate: "iced_widget"
module: "iced_widget::text"
file: "iced_widget/text/type.StyleFn.md"
docs_url: "https://docs.iced.rs/iced_widget/text/type.StyleFn.html"
---

# `iced_widget::text::StyleFn`

```rust
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;
```

A styling function for a [`Text`].

This is just a boxed closure: `Fn(&Theme, Status) -> Style`.
