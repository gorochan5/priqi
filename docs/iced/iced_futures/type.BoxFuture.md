---
title: "BoxFuture"
kind: type
path: "iced_futures::BoxFuture"
crate: "iced_futures"
module: "iced_futures"
file: "iced_futures/type.BoxFuture.md"
docs_url: "https://docs.iced.rs/iced_futures/type.BoxFuture.html"
source_file: "src/iced_futures/lib.rs"
source_lines: "33"
---

# `iced_futures::BoxFuture`

```rust
pub type BoxFuture<T> = BoxFuture<'static, T>;
```

A boxed static future.

- On native platforms, it needs a `Send` requirement.
- On the Web platform, it does not need a `Send` requirement.

**Source:** [lib.rs](../src/iced_futures/lib.rs.md) (lines 33)
