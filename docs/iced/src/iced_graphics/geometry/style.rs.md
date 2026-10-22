---
title: "src/iced_graphics/geometry/style.rs"
kind: source
path: "src/iced_graphics/geometry/style.rs"
crate: "iced_graphics"
file: "src/iced_graphics/geometry/style.rs.md"
docs_url: "https://docs.iced.rs/src/iced_graphics/geometry/style.rs.html"
source_file: "src/iced_graphics/geometry/style.rs"
---

# `src/iced_graphics/geometry/style.rs`

```rust
use crate::core::Color;
use crate::geometry::Gradient;

/// The coloring style of some drawing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Style {
    /// A solid [`Color`].
    Solid(Color),

    /// A [`Gradient`] color.
    Gradient(Gradient),
}

impl From<Color> for Style {
    fn from(color: Color) -> Self {
        Self::Solid(color)
    }
}

impl From<Gradient> for Style {
    fn from(gradient: Gradient) -> Self {
        Self::Gradient(gradient)
    }
}
```
