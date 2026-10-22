---
title: "exit"
kind: function
path: "iced_runtime::exit"
crate: "iced_runtime"
module: "iced_runtime"
file: "iced_runtime/fn.exit.md"
docs_url: "https://docs.iced.rs/iced_runtime/fn.exit.html"
source_file: "src/iced_runtime/lib.rs"
source_lines: "139-141"
---

# `iced_runtime::exit`

```rust
pub fn exit<T>() -> Task<T>
```

Creates a [`Task`](task/struct.Task.md) that exits the iced runtime.

This will normally close any application windows and terminate the runtime loop.

**Source:** [lib.rs](../src/iced_runtime/lib.rs.md) (lines 139-141)
