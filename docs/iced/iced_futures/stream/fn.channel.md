---
title: "channel"
kind: function
path: "iced_futures::stream::channel"
crate: "iced_futures"
module: "iced_futures::stream"
file: "iced_futures/stream/fn.channel.md"
docs_url: "https://docs.iced.rs/iced_futures/stream/fn.channel.html"
source_file: "src/iced_futures/stream.rs"
source_lines: "11-17"
---

# `iced_futures::stream::channel`

```rust
pub fn channel<T>(
    size: usize,
    f: impl AsyncFnOnce(Sender<T>),
) -> impl Stream<Item = T>
```

Creates a new [`Stream`] that produces the items sent from a [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html) to the [`mpsc::Sender`] provided to the closure.

This is a more ergonomic [`stream::unfold`], which allows you to go from the “world of futures” to the “world of streams” by simply looping and publishing to an async channel from inside a [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html).

**Source:** [stream.rs](../../src/iced_futures/stream.rs.md) (lines 11-17)
