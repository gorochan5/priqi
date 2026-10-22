---
title: "stream"
kind: module
path: "iced_futures::stream"
crate: "iced_futures"
module: "iced_futures"
file: "iced_futures/stream/index.md"
docs_url: "https://docs.iced.rs/iced_futures/stream/index.html"
source_file: "src/iced_futures/stream.rs"
source_lines: "1-35"
---

# `iced_futures::stream`

Create asynchronous streams of data.

## Functions

- [`channel`](fn.channel.md) — Creates a new [`Stream`] that produces the items sent from a [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html) to the [`mpsc::Sender`] provided to the closure.
- [`try_channel`](fn.try_channel.md) — Creates a new [`Stream`] that produces the items sent from a [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html) that can fail to the [`mpsc::Sender`] provided to the closure.
