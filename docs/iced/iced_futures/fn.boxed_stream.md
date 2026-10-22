---
title: "boxed_stream"
kind: function
path: "iced_futures::boxed_stream"
crate: "iced_futures"
module: "iced_futures"
file: "iced_futures/fn.boxed_stream.md"
docs_url: "https://docs.iced.rs/iced_futures/fn.boxed_stream.html"
source_file: "src/iced_futures/lib.rs"
source_lines: "45-50"
---

# `iced_futures::boxed_stream`

```rust
pub fn boxed_stream<T, S>(stream: S) -> BoxStream<T> where
    S: Stream<Item = T> + Send + 'static,
```

Boxes a stream.

- On native platforms, it needs a `Send` requirement.
- On the Web platform, it does not need a `Send` requirement.

**Source:** [lib.rs](../src/iced_futures/lib.rs.md) (lines 45-50)
