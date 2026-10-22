---
title: "MaybeSync"
kind: trait
path: "iced_futures::MaybeSync"
crate: "iced_futures"
module: "iced_futures"
file: "iced_futures/trait.MaybeSync.md"
docs_url: "https://docs.iced.rs/iced_futures/trait.MaybeSync.html"
source_file: "src/iced_futures/maybe.rs"
source_lines: "13"
---

# `iced_futures::MaybeSync`

```rust
pub trait MaybeSync: Sync { }
```

An extension trait that enforces `Sync` only on native platforms.

Useful for writing cross-platform async code!

**Source:** [maybe.rs](../src/iced_futures/maybe.rs.md) (lines 13)

## Implementors

- `impl<T> MaybeSync for T where T: Sync,`
