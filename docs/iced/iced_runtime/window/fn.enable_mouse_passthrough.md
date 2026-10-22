---
title: "enable_mouse_passthrough"
kind: function
path: "iced_runtime::window::enable_mouse_passthrough"
crate: "iced_runtime"
module: "iced_runtime::window"
file: "iced_runtime/window/fn.enable_mouse_passthrough.md"
docs_url: "https://docs.iced.rs/iced_runtime/window/fn.enable_mouse_passthrough.html"
source_file: "src/iced_runtime/window.rs"
source_lines: "455-457"
---

# `iced_runtime::window::enable_mouse_passthrough`

```rust
pub fn enable_mouse_passthrough<Message>(id: Id) -> Task<Message>
```

Enables mouse passthrough for the given window.

This disables mouse events for the window and passes mouse events through to whatever window is underneath.

**Source:** [window.rs](../../src/iced_runtime/window.rs.md) (lines 455-457)
