---
title: "try_channel"
kind: function
path: "iced_futures::stream::try_channel"
crate: "iced_futures"
module: "iced_futures::stream"
file: "iced_futures/stream/fn.try_channel.md"
docs_url: "https://docs.iced.rs/iced_futures/stream/fn.try_channel.html"
source_file: "src/iced_futures/stream.rs"
source_lines: "21-35"
---

# `iced_futures::stream::try_channel`

```rust
pub fn try_channel<T, E>(
    size: usize,
    f: impl AsyncFnOnce(Sender<T>) -> Result<(), E>,
) -> impl Stream<Item = Result<T, E>>
```

Creates a new [`Stream`] that produces the items sent from a [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html) that can fail to the [`mpsc::Sender`] provided to the closure.

**Source:** [stream.rs](../../src/iced_futures/stream.rs.md) (lines 21-35)
