---
title: "position"
kind: function
path: "iced_winit::conversion::position"
crate: "iced_winit"
module: "iced_winit::conversion"
file: "iced_winit/conversion/fn.position.md"
docs_url: "https://docs.iced.rs/iced_winit/conversion/fn.position.html"
source_file: "src/iced_winit/conversion.rs"
source_lines: "326-388"
---

# `iced_winit::conversion::position`

```rust
pub fn position(
    monitor: Option<&MonitorHandle>,
    size: Size,
    position: Position,
) -> Option<Position>
```

Converts a [`window::Position`] into a [`winit`](https://github.com/rust-windowing/winit) logical position for a given monitor.

**Source:** [conversion.rs](../../src/iced_winit/conversion.rs.md) (lines 326-388)
