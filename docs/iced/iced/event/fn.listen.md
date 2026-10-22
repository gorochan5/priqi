---
title: "listen"
kind: function
path: "iced::event::listen"
crate: "iced"
module: "iced::event"
file: "iced/event/fn.listen.md"
docs_url: "https://docs.iced.rs/iced/event/fn.listen.html"
---

# `iced::event::listen`

```rust
pub fn listen() -> Subscription<Event>
```

Returns a [`Subscription`](../struct.Subscription.md) to all the ignored runtime events.

This subscription will notify your application of any [`Event`](../enum.Event.md) that was not captured by any widget.
