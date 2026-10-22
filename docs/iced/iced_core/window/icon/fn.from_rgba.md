---
title: "from_rgba"
kind: function
path: "iced_core::window::icon::from_rgba"
crate: "iced_core"
module: "iced_core::window::icon"
file: "iced_core/window/icon/fn.from_rgba.md"
docs_url: "https://docs.iced.rs/iced_core/window/icon/fn.from_rgba.html"
source_file: "src/iced_core/window/icon.rs"
source_lines: "7-31"
---

# `iced_core::window::icon::from_rgba`

```rust
pub fn from_rgba(rgba: Vec<u8>, width: u32, height: u32) -> Result<Icon, Error>
```

Builds an [`Icon`](struct.Icon.md) from its RGBA pixels in the `sRGB` color space.

**Source:** [icon.rs](../../../src/iced_core/window/icon.rs.md) (lines 7-31)
