---
title: "BoxStream"
kind: type
path: "iced_futures::BoxStream"
crate: "iced_futures"
module: "iced_futures"
file: "iced_futures/type.BoxStream.md"
docs_url: "https://docs.iced.rs/iced_futures/type.BoxStream.html"
source_file: "src/iced_futures/lib.rs"
source_lines: "39"
---

# `iced_futures::BoxStream`

```rust
pub type BoxStream<T> = BoxStream<'static, T>;
```

A boxed static stream.

- On native platforms, it needs a `Send` requirement.
- On the Web platform, it does not need a `Send` requirement.

**Source:** [lib.rs](../src/iced_futures/lib.rs.md) (lines 39)
