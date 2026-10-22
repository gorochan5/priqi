---
title: "box3d"
kind: function
path: "iced_renderer::geometry::path::lyon_path::geom::euclid::box3d"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::euclid"
file: "iced_renderer/geometry/path/lyon_path/geom/euclid/fn.box3d.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/euclid/fn.box3d.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::euclid::box3d`

```rust
pub fn box3d<T, U>(
    min_x: T,
    min_y: T,
    min_z: T,
    max_x: T,
    max_y: T,
    max_z: T,
) -> Box3D<T, U> where
    T: Copy,
```

Shorthand for `Box3D::new(Point3D::new(x1, y1, z1), Point3D::new(x2, y2, z2))`.
