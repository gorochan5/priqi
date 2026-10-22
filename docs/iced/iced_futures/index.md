---
title: "iced_futures"
kind: crate
path: "iced_futures"
crate: "iced_futures"
file: "iced_futures/index.md"
docs_url: "https://docs.iced.rs/iced_futures/index.html"
source_file: "src/iced_futures/lib.rs"
source_lines: "1-77"
---

# `iced_futures`

Asynchronous tasks for GUI programming, inspired by Elm.

![The foundations of the Iced ecosystem](https://github.com/iced-rs/iced/blob/0525d76ff94e828b7b21634fa94a747022001c83/docs/graphs/foundations.png?raw=true)

## Re-exports

- [`Executor`](executor/trait.Executor.md)
- [`Subscription`](subscription/struct.Subscription.md)
- `pub use futures;`
- `pub use iced_core as core;`

## Modules

- [`backend`](backend/index.md) — The underlying implementations of the `iced_futures` contract!
- [`event`](event/index.md) — Listen to runtime events.
- [`executor`](executor/index.md) — Choose your preferred executor to power a runtime.
- [`keyboard`](keyboard/index.md) — Listen to keyboard events.
- [`stream`](stream/index.md) — Create asynchronous streams of data.
- [`subscription`](subscription/index.md) — Listen to external events in your application.

## Structs

- [`Runtime`](struct.Runtime.md) — A batteries-included runtime of commands and subscriptions.

## Traits

- [`MaybeSend`](trait.MaybeSend.md) — An extension trait that enforces `Send` only on native platforms.
- [`MaybeSync`](trait.MaybeSync.md) — An extension trait that enforces `Sync` only on native platforms.

## Functions

- [`boxed_stream`](fn.boxed_stream.md) — Boxes a stream.

## Type Aliases

- [`BoxFuture`](type.BoxFuture.md) — A boxed static future.
- [`BoxStream`](type.BoxStream.md) — A boxed static stream.
