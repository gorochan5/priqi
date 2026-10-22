---
title: "stream"
kind: module
path: "futures_core::stream"
crate: "futures_core"
module: "futures_core"
file: "futures_core/stream/index.md"
docs_url: "https://docs.iced.rs/futures_core/stream/index.html"
source_file: "src/futures_core/stream.rs"
source_lines: "1-245"
---

# `futures_core::stream`

Asynchronous streams.

## Traits

- [`FusedStream`](trait.FusedStream.md) — A stream which tracks whether or not the underlying stream should no longer be polled.
- [`Stream`](trait.Stream.md) — A stream of values produced asynchronously.
- [`TryStream`](trait.TryStream.md) — A convenience for streams that return `Result` values that includes a variety of adapters tailored to such futures.

## Type Aliases

- [`BoxStream`](type.BoxStream.md) — An owned dynamically typed [`Stream`](trait.Stream.md) for use in cases where you can’t statically type your result or need to add some indirection.
- [`LocalBoxStream`](type.LocalBoxStream.md) — `BoxStream`, but without the `Send` requirement.
