---
title: "fullscreen"
kind: function
path: "iced_winit::conversion::fullscreen"
crate: "iced_winit"
module: "iced_winit::conversion"
file: "iced_winit/conversion/fn.fullscreen.md"
docs_url: "https://docs.iced.rs/iced_winit/conversion/fn.fullscreen.html"
source_file: "src/iced_winit/conversion.rs"
source_lines: "393-401"
---

# `iced_winit::conversion::fullscreen`

```rust
pub fn fullscreen(
    monitor: Option<MonitorHandle>,
    mode: Mode,
) -> Option<Fullscreen>
```

Converts a [`window::Mode`] into a [`winit`](https://github.com/rust-windowing/winit) fullscreen mode.

**Source:** [conversion.rs](../../src/iced_winit/conversion.rs.md) (lines 393-401)
