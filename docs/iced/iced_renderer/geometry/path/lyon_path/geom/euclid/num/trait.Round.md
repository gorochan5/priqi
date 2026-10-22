---
title: "Round"
kind: trait
path: "iced_renderer::geometry::path::lyon_path::geom::euclid::num::Round"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::euclid::num"
file: "iced_renderer/geometry/path/lyon_path/geom/euclid/num/trait.Round.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/euclid/num/trait.Round.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::euclid::num::Round`

```rust
pub trait Round: Copy {
    // Required method
    fn round(self) -> Self;
}
```

Defines the nearest integer value to the original value.

## Required Methods

### `round`

```rust
fn round(self) -> Self
```

Rounds to the nearest integer value.

This behavior is preserved for negative values (unlike the basic cast).

## Implementors

- `impl<T, U> Round for Point2D<T, U> where T: Round,`
- `impl<T, U> Round for Point3D<T, U> where T: Round,`
- `impl<T, U> Round for Size2D<T, U> where T: Round,`
- `impl<T, U> Round for Size3D<T, U> where T: Round,`
- `impl<T, U> Round for Vector2D<T, U> where T: Round,`
- `impl<T, U> Round for Vector3D<T, U> where T: Round,`
