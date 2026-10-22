---
title: "PathEvent"
kind: type
path: "iced_renderer::geometry::path::lyon_path::PathEvent"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path"
file: "iced_renderer/geometry/path/lyon_path/type.PathEvent.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/type.PathEvent.html"
---

# `iced_renderer::geometry::path::lyon_path::PathEvent`

```rust
pub type PathEvent = Event<Point2D<f32, UnknownUnit>, Point2D<f32, UnknownUnit>>;
```

A path event representing endpoints and control points as positions.

## Variants

### `Begin`

**Fields**

- `at: Point2D<f32, UnknownUnit>`

### `Line`

**Fields**

- `from: Point2D<f32, UnknownUnit>`
- `to: Point2D<f32, UnknownUnit>`

### `Quadratic`

**Fields**

- `from: Point2D<f32, UnknownUnit>`
- `ctrl: Point2D<f32, UnknownUnit>`
- `to: Point2D<f32, UnknownUnit>`

### `Cubic`

**Fields**

- `from: Point2D<f32, UnknownUnit>`
- `ctrl1: Point2D<f32, UnknownUnit>`
- `ctrl2: Point2D<f32, UnknownUnit>`
- `to: Point2D<f32, UnknownUnit>`

### `End`

**Fields**

- `last: Point2D<f32, UnknownUnit>`
- `first: Point2D<f32, UnknownUnit>`
- `close: bool`
