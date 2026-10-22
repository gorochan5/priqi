---
title: "raw_id"
kind: function
path: "iced_runtime::window::raw_id"
crate: "iced_runtime"
module: "iced_runtime::window"
file: "iced_runtime/window/fn.raw_id.md"
docs_url: "https://docs.iced.rs/iced_runtime/window/fn.raw_id.html"
source_file: "src/iced_runtime/window.rs"
source_lines: "420-422"
---

# `iced_runtime::window::raw_id`

```rust
pub fn raw_id<Message>(id: Id) -> Task<u64>
```

Gets an identifier unique to the window, provided by the underlying windowing system. This is not to be confused with [`Id`].

**Source:** [window.rs](../../src/iced_runtime/window.rs.md) (lines 420-422)
