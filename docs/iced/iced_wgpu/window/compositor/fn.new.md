---
title: "new"
kind: function
path: "iced_wgpu::window::compositor::new"
crate: "iced_wgpu"
module: "iced_wgpu::window::compositor"
file: "iced_wgpu/window/compositor/fn.new.md"
docs_url: "https://docs.iced.rs/iced_wgpu/window/compositor/fn.new.html"
source_file: "src/iced_wgpu/window/compositor.rs"
source_lines: "220-227"
---

# `iced_wgpu::window::compositor::new`

```rust
pub async fn new(
    settings: Settings,
    display: impl Display,
    compatible_window: impl Window,
    shell: Shell,
) -> Result<Compositor, Error>
```

Creates a [`Compositor`](struct.Compositor.md) with the given [`Settings`](struct.Settings.md) and window.

**Source:** [compositor.rs](../../../src/iced_wgpu/window/compositor.rs.md) (lines 220-227)
