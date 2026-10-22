---
title: "Zero"
kind: trait
path: "iced::widget::canvas::path::lyon_path::geom::euclid::num::Zero"
crate: "iced"
module: "iced::widget::canvas::path::lyon_path::geom::euclid::num"
file: "iced/widget/canvas/path/lyon_path/geom/euclid/num/trait.Zero.md"
docs_url: "https://docs.iced.rs/iced/widget/canvas/path/lyon_path/geom/euclid/num/trait.Zero.html"
---

# `iced::widget::canvas::path::lyon_path::geom::euclid::num::Zero`

```rust
pub trait Zero {
    // Required method
    fn zero() -> Self;
}
```

## Required Methods

### `zero`

```rust
fn zero() -> Self
```

## Implementors

- `impl<T> Zero for T where T: Zero,`
- `impl<T, U> Zero for Length<T, U> where T: Zero,`
- `impl<T, U> Zero for Point2D<T, U> where T: Zero,`
- `impl<T, U> Zero for Point3D<T, U> where T: Zero,`
- `impl<T, U> Zero for Size2D<T, U> where T: Zero,`
- `impl<T, U> Zero for Size3D<T, U> where T: Zero,`
- `impl<T, U> Zero for Vector2D<T, U> where T: Zero,`
- `impl<T, U> Zero for Vector3D<T, U> where T: Zero,`
