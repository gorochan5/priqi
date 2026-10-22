---
title: "window_event"
kind: function
path: "iced_winit::conversion::window_event"
crate: "iced_winit"
module: "iced_winit::conversion"
file: "iced_winit/conversion/fn.window_event.md"
docs_url: "https://docs.iced.rs/iced_winit/conversion/fn.window_event.html"
source_file: "src/iced_winit/conversion.rs"
source_lines: "149-310"
---

# `iced_winit::conversion::window_event`

```rust
pub fn window_event(
    event: WindowEvent,
    scale_factor: f32,
    modifiers: ModifiersState,
) -> Option<Event>
```

Converts a winit window event into an iced event.

**Source:** [conversion.rs](../../src/iced_winit/conversion.rs.md) (lines 149-310)
