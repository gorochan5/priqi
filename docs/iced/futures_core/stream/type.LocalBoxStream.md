---
title: "LocalBoxStream"
kind: type
path: "futures_core::stream::LocalBoxStream"
crate: "futures_core"
module: "futures_core::stream"
file: "futures_core/stream/type.LocalBoxStream.md"
docs_url: "https://docs.iced.rs/futures_core/stream/type.LocalBoxStream.html"
source_file: "src/futures_core/stream.rs"
source_lines: "24"
---

# `futures_core::stream::LocalBoxStream`

```rust
pub type LocalBoxStream<'a, T> = Pin<Box<dyn Stream<Item = T> + 'a>>;
```

`BoxStream`, but without the `Send` requirement.

This type is often created by the [`boxed_local`](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.boxed_local) method on [`StreamExt`](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html). See its documentation for more.

**Source:** [stream.rs](../../src/futures_core/stream.rs.md) (lines 24)
