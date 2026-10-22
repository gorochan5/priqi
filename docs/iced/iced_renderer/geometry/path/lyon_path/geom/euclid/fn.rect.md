---
title: "rect"
kind: function
path: "iced_renderer::geometry::path::lyon_path::geom::euclid::rect"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::euclid"
file: "iced_renderer/geometry/path/lyon_path/geom/euclid/fn.rect.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/euclid/fn.rect.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::euclid::rect`

```rust
pub const fn rect<T, U>(x: T, y: T, w: T, h: T) -> Rect<T, U>
```

Shorthand for `Rect::new(Point2D::new(x, y), Size2D::new(w, h))`.
