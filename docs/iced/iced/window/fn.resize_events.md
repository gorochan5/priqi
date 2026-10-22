---
title: "resize_events"
kind: function
path: "iced::window::resize_events"
crate: "iced"
module: "iced::window"
file: "iced/window/fn.resize_events.md"
docs_url: "https://docs.iced.rs/iced/window/fn.resize_events.html"
---

# `iced::window::resize_events`

```rust
pub fn resize_events() -> Subscription<(Id, Size)>
```

Subscribes to all [`Event::Resized`](enum.Event.md) occurrences in the running application.
