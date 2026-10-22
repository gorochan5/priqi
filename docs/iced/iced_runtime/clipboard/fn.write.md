---
title: "write"
kind: function
path: "iced_runtime::clipboard::write"
crate: "iced_runtime"
module: "iced_runtime::clipboard"
file: "iced_runtime/clipboard/fn.write.md"
docs_url: "https://docs.iced.rs/iced_runtime/clipboard/fn.write.html"
source_file: "src/iced_runtime/clipboard.rs"
source_lines: "108-112"
---

# `iced_runtime::clipboard::write`

```rust
pub fn write(content: impl Into<Content>) -> Task<Result<(), Error>>
```

Write the given [`Content`] to the clipboard.

**Source:** [clipboard.rs](../../src/iced_runtime/clipboard.rs.md) (lines 108-112)
