---
title: "disable_mouse_passthrough"
kind: function
path: "iced_runtime::window::disable_mouse_passthrough"
crate: "iced_runtime"
module: "iced_runtime::window"
file: "iced_runtime/window/fn.disable_mouse_passthrough.md"
docs_url: "https://docs.iced.rs/iced_runtime/window/fn.disable_mouse_passthrough.html"
source_file: "src/iced_runtime/window.rs"
source_lines: "463-465"
---

# `iced_runtime::window::disable_mouse_passthrough`

```rust
pub fn disable_mouse_passthrough<Message>(id: Id) -> Task<Message>
```

Disables mouse passthrough for the given window.

This enables mouse events for the window and stops mouse events from being passed to whatever is underneath.

**Source:** [window.rs](../../src/iced_runtime/window.rs.md) (lines 463-465)
