---
title: "Position"
kind: trait
path: "iced_renderer::geometry::path::lyon_path::Position"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path"
file: "iced_renderer/geometry/path/lyon_path/trait.Position.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/trait.Position.html"
---

# `iced_renderer::geometry::path::lyon_path::Position`

```rust
pub trait Position {
    // Required method
    fn position(&self) -> Point2D<f32, UnknownUnit>;
}
```

Interface for types types (typically endpoints and control points) that have a 2D position.

## Required Methods

### `position`

```rust
fn position(&self) -> Point2D<f32, UnknownUnit>
```

## Implementors

- `impl<U> Position for Point2D<f32, U>`
