---
title: "gain_focus"
kind: function
path: "iced_runtime::window::gain_focus"
crate: "iced_runtime"
module: "iced_runtime::window"
file: "iced_runtime/window/fn.gain_focus.md"
docs_url: "https://docs.iced.rs/iced_runtime/window/fn.gain_focus.html"
source_file: "src/iced_runtime/window.rs"
source_lines: "402-404"
---

# `iced_runtime::window::gain_focus`

```rust
pub fn gain_focus<T>(id: Id) -> Task<T>
```

Brings the window to the front and sets input focus. Has no effect if the window is already in focus, minimized, or not visible.

This [`Task`](../task/struct.Task.md) steals input focus from other applications. Do not use this method unless you are certain that’s what the user wants. Focus stealing can cause an extremely disruptive user experience.

**Source:** [window.rs](../../src/iced_runtime/window.rs.md) (lines 402-404)
