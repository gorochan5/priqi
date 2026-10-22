---
title: "src/iced_beacon/stream.rs"
kind: source
path: "src/iced_beacon/stream.rs"
crate: "iced_beacon"
file: "src/iced_beacon/stream.rs.md"
docs_url: "https://docs.iced.rs/src/iced_beacon/stream.rs.html"
source_file: "src/iced_beacon/stream.rs"
---

# `src/iced_beacon/stream.rs`

```rust
use futures::Future;
use futures::channel::mpsc;
use futures::stream::{self, Stream, StreamExt};

pub fn channel<T, F>(f: impl Fn(mpsc::Sender<T>) -> F) -> impl Stream<Item = T>
where
    F: Future<Output = ()>,
{
    let (sender, receiver) = mpsc::channel(1);

    stream::select(
        receiver,
        stream::once(f(sender)).filter_map(|_| async { None }),
    )
}
```
