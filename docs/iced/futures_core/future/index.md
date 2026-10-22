---
title: "future"
kind: module
path: "futures_core::future"
crate: "futures_core"
module: "futures_core"
file: "futures_core/future/index.md"
docs_url: "https://docs.iced.rs/futures_core/future/index.html"
source_file: "src/futures_core/future.rs"
source_lines: "1-113"
---

# `futures_core::future`

Futures.

## Re-exports

- [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html.md)

## Traits

- [`FusedFuture`](trait.FusedFuture.md) — A future which tracks whether or not the underlying future should no longer be polled.
- [`TryFuture`](trait.TryFuture.md) — A convenience for futures that return `Result` values that includes a variety of adapters tailored to such futures.

## Type Aliases

- [`BoxFuture`](type.BoxFuture.md) — An owned dynamically typed [`Future`](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html) for use in cases where you can’t statically type your result or need to add some indirection.
- [`LocalBoxFuture`](type.LocalBoxFuture.md) — `BoxFuture`, but without the `Send` requirement.
