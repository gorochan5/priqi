---
title: "read"
kind: function
path: "iced_runtime::clipboard::read"
crate: "iced_runtime"
module: "iced_runtime::clipboard"
file: "iced_runtime/clipboard/fn.read.md"
docs_url: "https://docs.iced.rs/iced_runtime/clipboard/fn.read.html"
source_file: "src/iced_runtime/clipboard.rs"
source_lines: "33-36"
---

# `iced_runtime::clipboard::read`

```rust
pub fn read(kind: Kind) -> Task<Result<Arc<Content>, Error>>
```

Read the given [`Kind`] of [`Content`] from the clipboard.

**Source:** [clipboard.rs](../../src/iced_runtime/clipboard.rs.md) (lines 33-36)
