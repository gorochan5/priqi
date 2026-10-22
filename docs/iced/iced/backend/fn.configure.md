---
title: "configure"
kind: function
path: "iced::backend::configure"
crate: "iced"
module: "iced::backend"
file: "iced/backend/fn.configure.md"
docs_url: "https://docs.iced.rs/iced/backend/fn.configure.html"
---

# `iced::backend::configure`

```rust
pub fn configure(settings: Settings) -> Task<Result<(), Error>>
```

Returns a [`Task`](../struct.Task.md) that switches the [`backend::Settings`](struct.Settings.md) of the current application.

This can be leveraged to switch renderers at runtime.
