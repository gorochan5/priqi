---
title: "IntoFragment"
kind: trait
path: "iced_widget::text::IntoFragment"
crate: "iced_widget"
module: "iced_widget::text"
file: "iced_widget/text/trait.IntoFragment.md"
docs_url: "https://docs.iced.rs/iced_widget/text/trait.IntoFragment.html"
---

# `iced_widget::text::IntoFragment`

```rust
pub trait IntoFragment<'a> {
    // Required method
    fn into_fragment(self) -> Cow<'a, str>;
}
```

A trait for converting a value to some text [`Fragment`](type.Fragment.md).

## Required Methods

### `into_fragment`

```rust
fn into_fragment(self) -> Cow<'a, str>
```

Converts the value to some text [`Fragment`](type.Fragment.md).

## Implementors
