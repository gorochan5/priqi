---
title: "Fragment"
kind: type
path: "iced_widget::text::Fragment"
crate: "iced_widget"
module: "iced_widget::text"
file: "iced_widget/text/type.Fragment.md"
docs_url: "https://docs.iced.rs/iced_widget/text/type.Fragment.html"
---

# `iced_widget::text::Fragment`

```rust
pub type Fragment<'a> = Cow<'a, str>;
```

A fragment of [`Text`].

This is just an alias to a string that may be either borrowed or owned.

## Variants

### `Borrowed(&'a str)`

Borrowed data.

### `Owned(String)`

Owned data.
