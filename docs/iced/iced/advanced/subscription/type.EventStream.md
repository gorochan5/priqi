---
title: "EventStream"
kind: type
path: "iced::advanced::subscription::EventStream"
crate: "iced"
module: "iced::advanced::subscription"
file: "iced/advanced/subscription/type.EventStream.md"
docs_url: "https://docs.iced.rs/iced/advanced/subscription/type.EventStream.html"
---

# `iced::advanced::subscription::EventStream`

```rust
pub type EventStream = Pin<Box<dyn Stream<Item = Event> + Send>>;
```

A stream of runtime events.

It is the input of a [`Subscription`](../../struct.Subscription.md).
