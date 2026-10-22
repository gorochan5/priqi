---
title: "open"
kind: function
path: "iced::window::open"
crate: "iced"
module: "iced::window"
file: "iced/window/fn.open.md"
docs_url: "https://docs.iced.rs/iced/window/fn.open.html"
---

# `iced::window::open`

```rust
pub fn open(settings: Settings) -> (Id, Task<Id>)
```

Opens a new window with the given [`Settings`](struct.Settings.md); producing the [`Id`](struct.Id.md) of the new window on completion.
