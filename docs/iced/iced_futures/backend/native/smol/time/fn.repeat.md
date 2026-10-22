---
title: "repeat"
kind: function
path: "iced_futures::backend::native::smol::time::repeat"
crate: "iced_futures"
module: "iced_futures::backend::native::smol::time"
file: "iced_futures/backend/native/smol/time/fn.repeat.md"
docs_url: "https://docs.iced.rs/iced_futures/backend/native/smol/time/fn.repeat.html"
source_file: "src/iced_futures/backend/native/smol.rs"
source_lines: "45-62"
---

# `iced_futures::backend::native::smol::time::repeat`

```rust
pub fn repeat<F, T>(f: fn() -> F, interval: Duration) -> Subscription<T> where
    F: Future<Output = T> + MaybeSend + 'static,
    T: MaybeSend + 'static,
```

Returns a [`Subscription`](../../../../subscription/struct.Subscription.md) that runs the given async function at a set interval; producing the result of the function as output.

**Source:** [smol.rs](../../../../../src/iced_futures/backend/native/smol.rs.md) (lines 45-62)
