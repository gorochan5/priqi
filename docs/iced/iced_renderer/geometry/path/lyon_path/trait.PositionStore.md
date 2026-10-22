---
title: "PositionStore"
kind: trait
path: "iced_renderer::geometry::path::lyon_path::PositionStore"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path"
file: "iced_renderer/geometry/path/lyon_path/trait.PositionStore.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/trait.PositionStore.html"
---

# `iced_renderer::geometry::path::lyon_path::PositionStore`

```rust
pub trait PositionStore {
    // Required methods
    fn get_endpoint(&self, id: EndpointId) -> Point2D<f32, UnknownUnit>;
    fn get_control_point(&self, id: ControlPointId) -> Point2D<f32, UnknownUnit>;
}
```

Interface for objects storing endpoints and control points positions.

This interface can be implemented by path objects themselves or via external data structures.

## Required Methods

### `get_endpoint`

```rust
fn get_endpoint(&self, id: EndpointId) -> Point2D<f32, UnknownUnit>
```

### `get_control_point`

```rust
fn get_control_point(&self, id: ControlPointId) -> Point2D<f32, UnknownUnit>
```

## Implementors

- `impl PositionStore for Path`
- `impl<'l> PositionStore for PathSlice<'l>`
- `impl<'l, Endpoint> PositionStore for Polygon<'l, Endpoint> where Endpoint: Position,`
- `impl<'l, Endpoint, ControlPoint> PositionStore for CommandsPathSlice<'l, Endpoint, ControlPoint> where Endpoint: Position, ControlPoint: Position,`
