---
title: "monitor_size"
kind: function
path: "iced::window::monitor_size"
crate: "iced"
module: "iced::window"
file: "iced/window/fn.monitor_size.md"
docs_url: "https://docs.iced.rs/iced/window/fn.monitor_size.html"
---

# `iced::window::monitor_size`

```rust
pub fn monitor_size(id: Id) -> Task<Option<Size>>
```

Gets the logical dimensions of the monitor containing the window with the given [`Id`](struct.Id.md).
