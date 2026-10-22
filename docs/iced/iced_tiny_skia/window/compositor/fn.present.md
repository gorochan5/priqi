---
title: "present"
kind: function
path: "iced_tiny_skia::window::compositor::present"
crate: "iced_tiny_skia"
module: "iced_tiny_skia::window::compositor"
file: "iced_tiny_skia/window/compositor/fn.present.md"
docs_url: "https://docs.iced.rs/iced_tiny_skia/window/compositor/fn.present.html"
source_file: "src/iced_tiny_skia/window/compositor.rs"
source_lines: "133-203"
---

# `iced_tiny_skia::window::compositor::present`

```rust
pub fn present(
    renderer: &mut Renderer,
    surface: &mut Surface,
    viewport: &Viewport,
    background: Color,
    on_pre_present: impl FnOnce(),
) -> Result<(), SurfaceError>
```

**Source:** [compositor.rs](../../../src/iced_tiny_skia/window/compositor.rs.md) (lines 133-203)
