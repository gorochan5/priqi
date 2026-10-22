---
title: "listen"
kind: function
path: "iced_futures::event::listen"
crate: "iced_futures"
module: "iced_futures::event"
file: "iced_futures/event/fn.listen.md"
docs_url: "https://docs.iced.rs/iced_futures/event/fn.listen.html"
source_file: "src/iced_futures/event.rs"
source_lines: "11-16"
---

# `iced_futures::event::listen`

```rust
pub fn listen() -> Subscription<Event>
```

Returns a [`Subscription`](../subscription/struct.Subscription.md) to all the ignored runtime events.

This subscription will notify your application of any [`Event`] that was not captured by any widget.

**Source:** [event.rs](../../src/iced_futures/event.rs.md) (lines 11-16)
