---
title: "EventStream"
kind: type
path: "iced_futures::subscription::EventStream"
crate: "iced_futures"
module: "iced_futures::subscription"
file: "iced_futures/subscription/type.EventStream.md"
docs_url: "https://docs.iced.rs/iced_futures/subscription/type.EventStream.html"
source_file: "src/iced_futures/subscription.rs"
source_lines: "59"
---

# `iced_futures::subscription::EventStream`

```rust
pub type EventStream = BoxStream<Event>;
```

A stream of runtime events.

It is the input of a [`Subscription`](struct.Subscription.md).

**Source:** [subscription.rs](../../src/iced_futures/subscription.rs.md) (lines 59)
