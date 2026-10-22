---
title: "never"
kind: function
path: "iced::never"
crate: "iced"
module: "iced"
file: "iced/fn.never.md"
docs_url: "https://docs.iced.rs/iced/fn.never.html"
---

# `iced::never`

```rust
pub fn never<T>(never: Infallible) -> T
```

A function that can *never* be called.

This is useful to turn generic types into anything you want by coercing them into a type with no possible values.
