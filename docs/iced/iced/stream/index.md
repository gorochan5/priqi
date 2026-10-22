---
title: "stream"
kind: module
path: "iced::stream"
crate: "iced"
module: "iced"
file: "iced/stream/index.md"
docs_url: "https://docs.iced.rs/iced/stream/index.html"
---

# `iced::stream`

Create asynchronous streams of data.

## Functions

- [`channel`](fn.channel.md) — Creates a new [`Stream`] that produces the items sent from a [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html) to the [`mpsc::Sender`] provided to the closure.
- [`try_channel`](fn.try_channel.md) — Creates a new [`Stream`] that produces the items sent from a [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html) that can fail to the [`mpsc::Sender`] provided to the closure.
