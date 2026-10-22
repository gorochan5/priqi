---
title: "Never"
kind: type
path: "iced::task::Never"
crate: "iced"
module: "iced::task"
file: "iced/task/type.Never.md"
docs_url: "https://docs.iced.rs/iced/task/type.Never.html"
---

# `iced::task::Never`

```rust
pub type Never = Infallible;
```

A type with no possible values.

This is used to indicate values which can never be created, such as the error type of infallible futures.

This type is a stable equivalent to the `!` type from `std`.

This is currently an alias for [`std::convert::Infallible`](../enum.Never.md), but in the future it may be an alias for [`!`](https://doc.rust-lang.org/nightly/std/primitive.never.html). See [“Future compatibility” section of `std::convert::Infallible`](https://doc.rust-lang.org/nightly/std/convert/enum.Infallible.html#future-compatibility) for more.
