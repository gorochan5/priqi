---
title: "Ceil"
kind: trait
path: "iced_renderer::geometry::path::lyon_path::geom::euclid::num::Ceil"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::euclid::num"
file: "iced_renderer/geometry/path/lyon_path/geom/euclid/num/trait.Ceil.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/euclid/num/trait.Ceil.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::euclid::num::Ceil`

```rust
pub trait Ceil: Copy {
    // Required method
    fn ceil(self) -> Self;
}
```

Defines the smallest integer equal or greater than the original value.

## Required Methods

### `ceil`

```rust
fn ceil(self) -> Self
```

Rounds to the smallest integer equal or greater than the original value.

This behavior is preserved for negative values (unlike the basic cast).

## Implementors

- `impl<T, U> Ceil for Point2D<T, U> where T: Ceil,`
- `impl<T, U> Ceil for Point3D<T, U> where T: Ceil,`
- `impl<T, U> Ceil for Size2D<T, U> where T: Ceil,`
- `impl<T, U> Ceil for Size3D<T, U> where T: Ceil,`
- `impl<T, U> Ceil for Vector2D<T, U> where T: Ceil,`
- `impl<T, U> Ceil for Vector3D<T, U> where T: Ceil,`
