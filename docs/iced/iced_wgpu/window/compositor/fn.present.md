---
title: "present"
kind: function
path: "iced_wgpu::window::compositor::present"
crate: "iced_wgpu"
module: "iced_wgpu::window::compositor"
file: "iced_wgpu/window/compositor/fn.present.md"
docs_url: "https://docs.iced.rs/iced_wgpu/window/compositor/fn.present.html"
source_file: "src/iced_wgpu/window/compositor.rs"
source_lines: "230-264"
---

# `iced_wgpu::window::compositor::present`

```rust
pub fn present(
    renderer: &mut Renderer,
    surface: &mut Surface<'static>,
    viewport: &Viewport,
    background_color: Color,
    on_pre_present: impl FnOnce(),
) -> Result<(), SurfaceError>
```

Presents the given primitives with the given [`Compositor`](struct.Compositor.md).

**Source:** [compositor.rs](../../../src/iced_wgpu/window/compositor.rs.md) (lines 230-264)
