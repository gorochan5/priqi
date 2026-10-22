---
title: "touch_event"
kind: function
path: "iced_winit::conversion::touch_event"
crate: "iced_winit"
module: "iced_winit::conversion"
file: "iced_winit/conversion/fn.touch_event.md"
docs_url: "https://docs.iced.rs/iced_winit/conversion/fn.touch_event.html"
source_file: "src/iced_winit/conversion.rs"
source_lines: "524-538"
---

# `iced_winit::conversion::touch_event`

```rust
pub fn touch_event(touch: Touch, scale_factor: f32) -> Event
```

Converts a `Touch` from [`winit`](https://github.com/rust-windowing/winit) to an [`iced`](https://github.com/iced-rs/iced/tree/0.12) touch event.

**Source:** [conversion.rs](../../src/iced_winit/conversion.rs.md) (lines 524-538)
