---
title: "filter_map"
kind: function
path: "iced_futures::subscription::filter_map"
crate: "iced_futures"
module: "iced_futures::subscription"
file: "iced_futures/subscription/fn.filter_map.md"
docs_url: "https://docs.iced.rs/iced_futures/subscription/fn.filter_map.html"
source_file: "src/iced_futures/subscription.rs"
source_lines: "464-479"
---

# `iced_futures::subscription::filter_map`

```rust
pub fn filter_map<I, F, T>(id: I, f: F) -> Subscription<T> where
    I: Hash + 'static,
    F: Fn(Event) -> Option<T> + MaybeSend + 'static,
    T: 'static + MaybeSend,
```

Creates a [`Subscription`](struct.Subscription.md) from a hashable id and a filter function.

**Source:** [subscription.rs](../../src/iced_futures/subscription.rs.md) (lines 464-479)
