---
title: "open_events"
kind: function
path: "iced::window::open_events"
crate: "iced"
module: "iced::window"
file: "iced/window/fn.open_events.md"
docs_url: "https://docs.iced.rs/iced/window/fn.open_events.html"
---

# `iced::window::open_events`

```rust
pub fn open_events() -> Subscription<Id>
```

Subscribes to all [`Event::Opened`](enum.Event.md) occurrences in the running application.
