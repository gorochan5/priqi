---
title: "every"
kind: function
path: "iced_futures::backend::native::smol::time::every"
crate: "iced_futures"
module: "iced_futures::backend::native::smol::time"
file: "iced_futures/backend/native/smol/time/fn.every.md"
docs_url: "https://docs.iced.rs/iced_futures/backend/native/smol/time/fn.every.html"
source_file: "src/iced_futures/backend/native/smol.rs"
source_lines: "33-41"
---

# `iced_futures::backend::native::smol::time::every`

```rust
pub fn every(duration: Duration) -> Subscription<Instant>
```

Returns a [`Subscription`](../../../../subscription/struct.Subscription.md) that produces messages at a set interval.

The first message is produced after a `duration`, and then continues to produce more messages every `duration` after that.

**Source:** [smol.rs](../../../../../src/iced_futures/backend/native/smol.rs.md) (lines 33-41)
