---
title: "window_attributes"
kind: function
path: "iced_winit::conversion::window_attributes"
crate: "iced_winit"
module: "iced_winit::conversion"
file: "iced_winit/conversion/fn.window_attributes.md"
docs_url: "https://docs.iced.rs/iced_winit/conversion/fn.window_attributes.html"
source_file: "src/iced_winit/conversion.rs"
source_lines: "14-146"
---

# `iced_winit::conversion::window_attributes`

```rust
pub fn window_attributes(
    settings: Settings,
    title: &str,
    scale_factor: f32,
    primary_monitor: Option<MonitorHandle>,
    _id: Option<String>,
) -> WindowAttributes
```

Converts some [`window::Settings`] into some `WindowAttributes` from `winit`.

**Source:** [conversion.rs](../../src/iced_winit/conversion.rs.md) (lines 14-146)
