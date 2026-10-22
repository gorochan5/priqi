---
title: "src/iced_futures/stream.rs"
kind: source
path: "src/iced_futures/stream.rs"
crate: "iced_futures"
file: "src/iced_futures/stream.rs.md"
docs_url: "https://docs.iced.rs/src/iced_futures/stream.rs.html"
source_file: "src/iced_futures/stream.rs"
---

# `src/iced_futures/stream.rs`

```rust
//! Create asynchronous streams of data.
use futures::channel::mpsc;
use futures::stream::{self, Stream, StreamExt};

/// Creates a new [`Stream`] that produces the items sent from a [`Future`]
/// to the [`mpsc::Sender`] provided to the closure.
///
/// This is a more ergonomic [`stream::unfold`], which allows you to go
/// from the "world of futures" to the "world of streams" by simply looping
/// and publishing to an async channel from inside a [`Future`].
pub fn channel<T>(size: usize, f: impl AsyncFnOnce(mpsc::Sender<T>)) -> impl Stream<Item = T> {
    let (sender, receiver) = mpsc::channel(size);

    let runner = stream::once(f(sender)).filter_map(|_| async { None });

    stream::select(receiver, runner)
}

/// Creates a new [`Stream`] that produces the items sent from a [`Future`]
/// that can fail to the [`mpsc::Sender`] provided to the closure.
pub fn try_channel<T, E>(
    size: usize,
    f: impl AsyncFnOnce(mpsc::Sender<T>) -> Result<(), E>,
) -> impl Stream<Item = Result<T, E>> {
    let (sender, receiver) = mpsc::channel(size);

    let runner = stream::once(f(sender)).filter_map(|result| async {
        match result {
            Ok(()) => None,
            Err(error) => Some(Err(error)),
        }
    });

    stream::select(receiver.map(Ok), runner)
}
```
