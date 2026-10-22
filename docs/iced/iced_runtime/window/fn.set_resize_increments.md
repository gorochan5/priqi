---
title: "set_resize_increments"
kind: function
path: "iced_runtime::window::set_resize_increments"
crate: "iced_runtime"
module: "iced_runtime::window"
file: "iced_runtime/window/fn.set_resize_increments.md"
docs_url: "https://docs.iced.rs/iced_runtime/window/fn.set_resize_increments.html"
source_file: "src/iced_runtime/window.rs"
source_lines: "317-321"
---

# `iced_runtime::window::set_resize_increments`

```rust
pub fn set_resize_increments<T>(id: Id, increments: Option<Size>) -> Task<T>
```

Set the window size increment.

This is usually used by apps such as terminal emulators that need “blocky” resizing.

**Source:** [window.rs](../../src/iced_runtime/window.rs.md) (lines 317-321)
