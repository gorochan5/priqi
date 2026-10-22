---
title: "Backend"
kind: trait
path: "iced_graphics::geometry::frame::Backend"
crate: "iced_graphics"
module: "iced_graphics::geometry::frame"
file: "iced_graphics/geometry/frame/trait.Backend.md"
docs_url: "https://docs.iced.rs/iced_graphics/geometry/frame/trait.Backend.html"
source_file: "src/iced_graphics/geometry/frame.rs"
source_lines: "192-223"
---

# `iced_graphics::geometry::frame::Backend`

```rust
pub trait Backend: Sized {
    type Geometry;

Show 21 methods    // Required methods
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn size(&self) -> Size;
    fn center(&self) -> Point;
    fn push_transform(&mut self);
    fn pop_transform(&mut self);
    fn translate(&mut self, translation: Vector);
    fn rotate(&mut self, angle: impl Into<Radians>);
    fn scale(&mut self, scale: f32);
    fn scale_nonuniform(&mut self, scale: impl Into<Vector>);
    fn draft(&mut self, clip_bounds: Rectangle) -> Self;
    fn paste(&mut self, frame: Self);
    fn stroke<'a>(&mut self, path: &Path, stroke: impl Into<Stroke<'a>>);
    fn stroke_rectangle<'a>(
        &mut self,
        top_left: Point,
        size: Size,
        stroke: impl Into<Stroke<'a>>,
    );
    fn stroke_text<'a>(
        &mut self,
        text: impl Into<Text>,
        stroke: impl Into<Stroke<'a>>,
    );
    fn fill(&mut self, path: &Path, fill: impl Into<Fill>);
    fn fill_text(&mut self, text: impl Into<Text>);
    fn fill_rectangle(
        &mut self,
        top_left: Point,
        size: Size,
        fill: impl Into<Fill>,
    );
    fn draw_image(&mut self, bounds: Rectangle, image: impl Into<Image>);
    fn draw_svg(&mut self, bounds: Rectangle, svg: impl Into<Svg>);
    fn into_geometry(self) -> Self::Geometry;
}
```

The internal implementation of a [`Frame`](struct.Frame.md).

Analogous to [`Frame`](struct.Frame.md). See [`Frame`](struct.Frame.md) for the documentation of each method.

**Source:** [frame.rs](../../../src/iced_graphics/geometry/frame.rs.md) (lines 192-223)

## Required Associated Types

### `Geometry`

```rust
type Geometry
```

## Required Methods

### `width`

```rust
fn width(&self) -> f32
```

### `height`

```rust
fn height(&self) -> f32
```

### `size`

```rust
fn size(&self) -> Size
```

### `center`

```rust
fn center(&self) -> Point
```

### `push_transform`

```rust
fn push_transform(&mut self)
```

### `pop_transform`

```rust
fn pop_transform(&mut self)
```

### `translate`

```rust
fn translate(&mut self, translation: Vector)
```

### `rotate`

```rust
fn rotate(&mut self, angle: impl Into<Radians>)
```

### `scale`

```rust
fn scale(&mut self, scale: f32)
```

### `scale_nonuniform`

```rust
fn scale_nonuniform(&mut self, scale: impl Into<Vector>)
```

### `draft`

```rust
fn draft(&mut self, clip_bounds: Rectangle) -> Self
```

### `paste`

```rust
fn paste(&mut self, frame: Self)
```

### `stroke`

```rust
fn stroke<'a>(&mut self, path: &Path, stroke: impl Into<Stroke<'a>>)
```

### `stroke_rectangle`

```rust
fn stroke_rectangle<'a>( &mut self, top_left: Point, size: Size, stroke: impl Into<Stroke<'a>>, )
```

### `stroke_text`

```rust
fn stroke_text<'a>( &mut self, text: impl Into<Text>, stroke: impl Into<Stroke<'a>>, )
```

### `fill`

```rust
fn fill(&mut self, path: &Path, fill: impl Into<Fill>)
```

### `fill_text`

```rust
fn fill_text(&mut self, text: impl Into<Text>)
```

### `fill_rectangle`

```rust
fn fill_rectangle(&mut self, top_left: Point, size: Size, fill: impl Into<Fill>)
```

### `draw_image`

```rust
fn draw_image(&mut self, bounds: Rectangle, image: impl Into<Image>)
```

### `draw_svg`

```rust
fn draw_svg(&mut self, bounds: Rectangle, svg: impl Into<Svg>)
```

### `into_geometry`

```rust
fn into_geometry(self) -> Self::Geometry
```

## Implementors
