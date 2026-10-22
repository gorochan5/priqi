---
title: "configure"
kind: function
path: "iced_runtime::backend::configure"
crate: "iced_runtime"
module: "iced_runtime::backend"
file: "iced_runtime/backend/fn.configure.md"
docs_url: "https://docs.iced.rs/iced_runtime/backend/fn.configure.html"
source_file: "src/iced_runtime/backend.rs"
source_lines: "20-22"
---

# `iced_runtime::backend::configure`

```rust
pub fn configure(settings: Settings) -> Task<Result<(), Error>>
```

Returns a [`Task`](../task/struct.Task.md) that switches the [`backend::Settings`] of the current application.

This can be leveraged to switch renderers at runtime.

**Source:** [backend.rs](../../src/iced_runtime/backend.rs.md) (lines 20-22)
