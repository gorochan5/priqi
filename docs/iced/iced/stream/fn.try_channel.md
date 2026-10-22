---
title: "try_channel"
kind: function
path: "iced::stream::try_channel"
crate: "iced"
module: "iced::stream"
file: "iced/stream/fn.try_channel.md"
docs_url: "https://docs.iced.rs/iced/stream/fn.try_channel.html"
canonical: "iced_futures/stream/fn.try_channel"
---

# `iced::stream::try_channel`

> **Re-export.** This item is defined in [`iced_futures::stream::try_channel`](../../iced_futures/stream/fn.try_channel.md).

Creates a new `Stream` that produces the items sent from a `Future` that can fail to the `mpsc::Sender` provided to the closure.
