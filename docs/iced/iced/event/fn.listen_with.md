---
title: "listen_with"
kind: function
path: "iced::event::listen_with"
crate: "iced"
module: "iced::event"
file: "iced/event/fn.listen_with.md"
docs_url: "https://docs.iced.rs/iced/event/fn.listen_with.html"
---

# `iced::event::listen_with`

```rust
pub fn listen_with<Message>(
    f: fn(Event, Status, Id) -> Option<Message>,
) -> Subscription<Message> where
    Message: 'static + MaybeSend,
```

Creates a [`Subscription`](../struct.Subscription.md) that listens and filters all the runtime events with the provided function, producing messages accordingly.

This subscription will call the provided function for every [`Event`](../enum.Event.md) handled by the runtime. If the function:

- Returns `None`, the [`Event`](../enum.Event.md) will be discarded.
- Returns `Some` message, the `Message` will be produced.
