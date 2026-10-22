---
title: "event"
kind: module
path: "iced_futures::event"
crate: "iced_futures"
module: "iced_futures"
file: "iced_futures/event/index.md"
docs_url: "https://docs.iced.rs/iced_futures/event/index.html"
source_file: "src/iced_futures/event.rs"
source_lines: "1-92"
---

# `iced_futures::event`

Listen to runtime events.

## Functions

- [`listen`](fn.listen.md) — Returns a [`Subscription`](../subscription/struct.Subscription.md) to all the ignored runtime events.
- [`listen_raw`](fn.listen_raw.md) — Creates a [`Subscription`](../subscription/struct.Subscription.md) that produces a message for every runtime event, including the redraw request events.
- [`listen_url`](fn.listen_url.md) — Creates a [`Subscription`](../subscription/struct.Subscription.md) that notifies of custom application URL received from the system.
- [`listen_with`](fn.listen_with.md) — Creates a [`Subscription`](../subscription/struct.Subscription.md) that listens and filters all the runtime events with the provided function, producing messages accordingly.
