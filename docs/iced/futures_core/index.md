---
title: "futures_core"
kind: crate
path: "futures_core"
crate: "futures_core"
file: "futures_core/index.md"
docs_url: "https://docs.iced.rs/futures_core/index.html"
source_file: "src/futures_core/lib.rs"
source_lines: "1-27"
---

# `futures_core`

Core traits and types for asynchronous operations in Rust.

## Re-exports

- [`FusedFuture`](future/trait.FusedFuture.md)
- [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html.md)
- [`TryFuture`](future/trait.TryFuture.md)
- [`FusedStream`](stream/trait.FusedStream.md)
- [`Stream`](stream/trait.Stream.md)
- [`TryStream`](stream/trait.TryStream.md)

## Modules

- [`future`](future/index.md) — Futures.
- [`stream`](stream/index.md) — Asynchronous streams.
- [`task`](task/index.md) — Task notification.

## Macros

- [`ready`](macro.ready.md) — Extracts the successful type of `Poll<T>`.
