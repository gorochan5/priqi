---
title: "src/iced_graphics/geometry.rs"
kind: source
path: "src/iced_graphics/geometry.rs"
crate: "iced_graphics"
file: "src/iced_graphics/geometry.rs.md"
docs_url: "https://docs.iced.rs/src/iced_graphics/geometry.rs.html"
source_file: "src/iced_graphics/geometry.rs"
---

# `src/iced_graphics/geometry.rs`

```rust
//! Build and draw geometry.
pub mod fill;
pub mod frame;
pub mod path;
pub mod stroke;

mod cache;
mod style;
mod text;

pub use cache::Cache;
pub use fill::Fill;
pub use frame::Frame;
pub use path::Path;
pub use stroke::{LineCap, LineDash, LineJoin, Stroke};
pub use style::Style;
pub use text::Text;

pub use crate::core::{Image, Svg};
pub use crate::gradient::{self, Gradient};

use crate::cache::Cached;
use crate::core::{self, Rectangle};

/// A renderer capable of drawing some [`Self::Geometry`].
pub trait Renderer: core::Renderer {
    /// The kind of geometry this renderer can draw.
    type Geometry: Cached;

    /// The kind of [`Frame`] this renderer supports.
    type Frame: frame::Backend<Geometry = Self::Geometry>;

    /// Creates a new [`Self::Frame`].
    fn new_frame(&self, bounds: Rectangle) -> Self::Frame;

    /// Draws the given [`Self::Geometry`].
    fn draw_geometry(&mut self, geometry: Self::Geometry);
}

#[cfg(debug_assertions)]
impl Renderer for () {
    type Geometry = ();
    type Frame = ();

    fn new_frame(&self, _bounds: Rectangle) -> Self::Frame {}

    fn draw_geometry(&mut self, _geometry: Self::Geometry) {}
}
```
