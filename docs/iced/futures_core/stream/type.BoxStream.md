---
title: "BoxStream"
kind: type
path: "futures_core::stream::BoxStream"
crate: "futures_core"
module: "futures_core::stream"
file: "futures_core/stream/type.BoxStream.md"
docs_url: "https://docs.iced.rs/futures_core/stream/type.BoxStream.html"
source_file: "src/futures_core/stream.rs"
source_lines: "15"
---

# `futures_core::stream::BoxStream`

```rust
pub type BoxStream<'a, T> = Pin<Box<dyn Stream<Item = T> + Send + 'a>>;
```

An owned dynamically typed [`Stream`](trait.Stream.md) for use in cases where you can’t statically type your result or need to add some indirection.

This type is often created by the [`boxed`](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.boxed) method on [`StreamExt`](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html). See its documentation for more.

**Source:** [stream.rs](../../src/futures_core/stream.rs.md) (lines 15)
