---
title: "directed_angle"
kind: function
path: "iced_renderer::geometry::path::lyon_path::geom::utils::directed_angle"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::utils"
file: "iced_renderer/geometry/path/lyon_path/geom/utils/fn.directed_angle.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/utils/fn.directed_angle.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::utils::directed_angle`

```rust
pub fn directed_angle<S>(
    v1: Vector2D<S, UnknownUnit>,
    v2: Vector2D<S, UnknownUnit>,
) -> Swhere
    S: Scalar,
```

Angle between vectors v1 and v2 (oriented clockwise assuming y points downwards). The result is a number between `0` and `2 * PI`.

ex: `directed_angle([0,1], [1,0]) = 3/2 Pi rad`

```
    x       __
  0-->     /  \
 y|       |  x--> v2
  v        \ |v1
             v
```

Or, assuming y points upwards: `directed_angle([0,-1], [1,0]) = 1/2 Pi rad`

```
  ^           v2
 y|          x-->
  0-->    v1 | /
    x        v-
```
