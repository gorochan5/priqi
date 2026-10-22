---
title: "close_requests"
kind: function
path: "iced::window::close_requests"
crate: "iced"
module: "iced::window"
file: "iced/window/fn.close_requests.md"
docs_url: "https://docs.iced.rs/iced/window/fn.close_requests.html"
---

# `iced::window::close_requests`

```rust
pub fn close_requests() -> Subscription<Id>
```

Subscribes to all [`Event::CloseRequested`](enum.Event.md) occurrences in the running application.
