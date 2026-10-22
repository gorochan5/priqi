---
title: "Renderer"
kind: trait
path: "iced_graphics::geometry::Renderer"
crate: "iced_graphics"
module: "iced_graphics::geometry"
file: "iced_graphics/geometry/trait.Renderer.md"
docs_url: "https://docs.iced.rs/iced_graphics/geometry/trait.Renderer.html"
source_file: "src/iced_graphics/geometry.rs"
source_lines: "26-38"
---

# `iced_graphics::geometry::Renderer`

```rust
pub trait Renderer: Renderer {
    type Geometry: Cached;
    type Frame: Backend<Geometry = Self::Geometry>;

    // Required methods
    fn new_frame(&self, bounds: Rectangle) -> Self::Frame;
    fn draw_geometry(&mut self, geometry: Self::Geometry);
}
```

A renderer capable of drawing some [`Self::Geometry`](trait.Renderer.md).

**Source:** [geometry.rs](../../src/iced_graphics/geometry.rs.md) (lines 26-38)

## Required Associated Types

### `Geometry`

```rust
type Geometry: Cached
```

The kind of geometry this renderer can draw.

### `Frame`

```rust
type Frame: Backend<Geometry = Self::Geometry>
```

The kind of [`Frame`](frame/struct.Frame.md) this renderer supports.

## Required Methods

### `new_frame`

```rust
fn new_frame(&self, bounds: Rectangle) -> Self::Frame
```

Creates a new [`Self::Frame`](trait.Renderer.md).

### `draw_geometry`

```rust
fn draw_geometry(&mut self, geometry: Self::Geometry)
```

Draws the given [`Self::Geometry`](trait.Renderer.md).

## Implementors
