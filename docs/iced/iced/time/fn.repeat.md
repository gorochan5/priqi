---
title: "repeat"
kind: function
path: "iced::time::repeat"
crate: "iced"
module: "iced::time"
file: "iced/time/fn.repeat.md"
docs_url: "https://docs.iced.rs/iced/time/fn.repeat.html"
---

# `iced::time::repeat`

```rust
pub fn repeat<F, T>(f: fn() -> F, interval: Duration) -> Subscription<T> where
    F: Future<Output = T> + MaybeSend + 'static,
    T: MaybeSend + 'static,
```

Returns a [`Subscription`](../struct.Subscription.md) that runs the given async function at a set interval; producing the result of the function as output.
