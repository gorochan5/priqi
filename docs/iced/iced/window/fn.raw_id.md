---
title: "raw_id"
kind: function
path: "iced::window::raw_id"
crate: "iced"
module: "iced::window"
file: "iced/window/fn.raw_id.md"
docs_url: "https://docs.iced.rs/iced/window/fn.raw_id.html"
---

# `iced::window::raw_id`

```rust
pub fn raw_id<Message>(id: Id) -> Task<u64>
```

Gets an identifier unique to the window, provided by the underlying windowing system. This is not to be confused with [`Id`](struct.Id.md).
