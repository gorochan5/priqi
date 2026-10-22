---
title: "listen_raw"
kind: function
path: "iced_futures::event::listen_raw"
crate: "iced_futures"
module: "iced_futures::event"
file: "iced_futures/event/fn.listen_raw.md"
docs_url: "https://docs.iced.rs/iced_futures/event/fn.listen_raw.html"
source_file: "src/iced_futures/event.rs"
source_lines: "55-74"
---

# `iced_futures::event::listen_raw`

```rust
pub fn listen_raw<Message>(
    f: fn(Event, Status, Id) -> Option<Message>,
) -> Subscription<Message> where
    Message: 'static + MaybeSend,
```

Creates a [`Subscription`](../subscription/struct.Subscription.md) that produces a message for every runtime event, including the redraw request events.

**Warning:** This [`Subscription`](../subscription/struct.Subscription.md), if unfiltered, may produce messages in an infinite loop.

**Source:** [event.rs](../../src/iced_futures/event.rs.md) (lines 55-74)
