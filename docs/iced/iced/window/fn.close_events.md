---
title: "close_events"
kind: function
path: "iced::window::close_events"
crate: "iced"
module: "iced::window"
file: "iced/window/fn.close_events.md"
docs_url: "https://docs.iced.rs/iced/window/fn.close_events.html"
---

# `iced::window::close_events`

```rust
pub fn close_events() -> Subscription<Id>
```

Subscribes to all [`Event::Closed`](enum.Event.md) occurrences in the running application.
