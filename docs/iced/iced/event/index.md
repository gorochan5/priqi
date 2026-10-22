---
title: "event"
kind: module
path: "iced::event"
crate: "iced"
module: "iced"
file: "iced/event/index.md"
docs_url: "https://docs.iced.rs/iced/event/index.html"
source_file: "src/iced/lib.rs"
source_lines: "588"
---

# `iced::event`

Handle events of a user interface.

## Enums

- [`Event`](enum.Event.md) — A user interface event.
- [`Status`](enum.Status.md) — The status of an [`Event`](../enum.Event.md) after being processed.

## Functions

- [`listen`](fn.listen.md) — Returns a [`Subscription`](../struct.Subscription.md) to all the ignored runtime events.
- [`listen_raw`](fn.listen_raw.md) — Creates a [`Subscription`](../struct.Subscription.md) that produces a message for every runtime event, including the redraw request events.
- [`listen_url`](fn.listen_url.md) — Creates a [`Subscription`](../struct.Subscription.md) that notifies of custom application URL received from the system.
- [`listen_with`](fn.listen_with.md) — Creates a [`Subscription`](../struct.Subscription.md) that listens and filters all the runtime events with the provided function, producing messages accordingly.
