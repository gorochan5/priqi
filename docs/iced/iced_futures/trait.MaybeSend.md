---
title: "MaybeSend"
kind: trait
path: "iced_futures::MaybeSend"
crate: "iced_futures"
module: "iced_futures"
file: "iced_futures/trait.MaybeSend.md"
docs_url: "https://docs.iced.rs/iced_futures/trait.MaybeSend.html"
source_file: "src/iced_futures/maybe.rs"
source_lines: "6"
---

# `iced_futures::MaybeSend`

```rust
pub trait MaybeSend: Send { }
```

An extension trait that enforces `Send` only on native platforms.

Useful for writing cross-platform async code!

**Source:** [maybe.rs](../src/iced_futures/maybe.rs.md) (lines 6)

## Implementors

- `impl<T> MaybeSend for T where T: Send,`
