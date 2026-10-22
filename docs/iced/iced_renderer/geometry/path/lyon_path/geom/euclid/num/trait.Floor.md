---
title: "Floor"
kind: trait
path: "iced_renderer::geometry::path::lyon_path::geom::euclid::num::Floor"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::euclid::num"
file: "iced_renderer/geometry/path/lyon_path/geom/euclid/num/trait.Floor.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/euclid/num/trait.Floor.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::euclid::num::Floor`

```rust
pub trait Floor: Copy {
    // Required method
    fn floor(self) -> Self;
}
```

Defines the biggest integer equal or lower than the original value.

## Required Methods

### `floor`

```rust
fn floor(self) -> Self
```

Rounds to the biggest integer equal or lower than the original value.

This behavior is preserved for negative values (unlike the basic cast).

## Implementors

- `impl<T, U> Floor for Point2D<T, U> where T: Floor,`
- `impl<T, U> Floor for Point3D<T, U> where T: Floor,`
- `impl<T, U> Floor for Size2D<T, U> where T: Floor,`
- `impl<T, U> Floor for Size3D<T, U> where T: Floor,`
- `impl<T, U> Floor for Vector2D<T, U> where T: Floor,`
- `impl<T, U> Floor for Vector3D<T, U> where T: Floor,`
