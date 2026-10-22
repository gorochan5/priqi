---
title: "shell"
kind: module
path: "iced::advanced::shell"
crate: "iced"
module: "iced::advanced"
file: "iced/advanced/shell/index.md"
docs_url: "https://docs.iced.rs/iced/advanced/shell/index.html"
---

# `iced::advanced::shell`

Communicate with the iced runtime from widgets.

## Structs

- [`Bus`](struct.Bus.md) — A channel of messages published by a [`Shell`](../struct.Shell.md).
- [`IntoIter`](struct.IntoIter.md) — An iterator returned by the implementation of [`IntoIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html) for [`Bus`](struct.Bus.md).
- [`Receipt`](struct.Receipt.md) — Proof that a message has been received.
- [`Shell`](struct.Shell.md) — A connection to the state of a shell.
- [`Tracking`](struct.Tracking.md) — A message tracking returned by [`Shell::publish`](../struct.Shell.md).
- [`Waker`](struct.Waker.md) — A waker can be used to wake up the iced runtime and, consequently, trigger wake events concurrently from widget logic.

## Enums

- [`Diff`](enum.Diff.md) — The diffing strategy to follow when invalidating some layout.
- [`Invalidation`](enum.Invalidation.md) — The most severe invalidation requested by a [`Shell`](../struct.Shell.md) while processing an event.
