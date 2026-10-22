---
title: "listen_with"
kind: function
path: "iced_futures::event::listen_with"
crate: "iced_futures"
module: "iced_futures::event"
file: "iced_futures/event/fn.listen_with.md"
docs_url: "https://docs.iced.rs/iced_futures/event/fn.listen_with.html"
source_file: "src/iced_futures/event.rs"
source_lines: "26-48"
---

# `iced_futures::event::listen_with`

```rust
pub fn listen_with<Message>(
    f: fn(Event, Status, Id) -> Option<Message>,
) -> Subscription<Message> where
    Message: 'static + MaybeSend,
```

Creates a [`Subscription`](../subscription/struct.Subscription.md) that listens and filters all the runtime events with the provided function, producing messages accordingly.

This subscription will call the provided function for every [`Event`] handled by the runtime. If the function:

- Returns `None`, the [`Event`] will be discarded.
- Returns `Some` message, the `Message` will be produced.

**Source:** [event.rs](../../src/iced_futures/event.rs.md) (lines 26-48)
