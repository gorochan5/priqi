---
title: "ApproxEq"
kind: trait
path: "iced_renderer::geometry::path::lyon_path::geom::euclid::approxeq::ApproxEq"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom::euclid::approxeq"
file: "iced_renderer/geometry/path/lyon_path/geom/euclid/approxeq/trait.ApproxEq.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/euclid/approxeq/trait.ApproxEq.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::euclid::approxeq::ApproxEq`

```rust
pub trait ApproxEq<Eps> {
    // Required methods
    fn approx_epsilon() -> Eps;
    fn approx_eq_eps(&self, other: &Self, approx_epsilon: &Eps) -> bool;

    // Provided method
    fn approx_eq(&self, other: &Self) -> bool { ... }
}
```

Trait for testing approximate equality

## Required Methods

### `approx_epsilon`

```rust
fn approx_epsilon() -> Eps
```

Default epsilon value

### `approx_eq_eps`

```rust
fn approx_eq_eps(&self, other: &Self, approx_epsilon: &Eps) -> bool
```

Returns `true` if this object is approximately equal to the other one, using a provided epsilon value.

## Provided Methods

### `approx_eq`

```rust
fn approx_eq(&self, other: &Self) -> bool
```

Returns `true` if this object is approximately equal to the other one, using the [`approx_epsilon`](trait.ApproxEq.md) epsilon value.

## Implementors

- `impl<T> ApproxEq<T> for Angle<T> where T: ApproxEq<T>,`
- `impl<T, Src, Dst> ApproxEq<T> for Rotation2D<T, Src, Dst> where T: Copy + Neg<Output = T> + ApproxEq<T>,`
- `impl<T, Src, Dst> ApproxEq<T> for Rotation3D<T, Src, Dst> where T: Copy + Neg<Output = T> + ApproxEq<T>,`
- `impl<T, Src, Dst> ApproxEq<T> for ScaleOffset2D<T, Src, Dst> where T: ApproxEq<T>,`
- `impl<T, Src, Dst> ApproxEq<T> for Transform2D<T, Src, Dst> where T: ApproxEq<T>,`
- `impl<T, Src, Dst> ApproxEq<T> for Transform3D<T, Src, Dst> where T: ApproxEq<T>,`
- `impl<T, U> ApproxEq<Point2D<T, U>> for Point2D<T, U> where T: ApproxEq<T>,`
- `impl<T, U> ApproxEq<Point3D<T, U>> for Point3D<T, U> where T: ApproxEq<T>,`
- `impl<T, U> ApproxEq<Vector2D<T, U>> for Vector2D<T, U> where T: ApproxEq<T>,`
- `impl<T, U> ApproxEq<Vector3D<T, U>> for Vector3D<T, U> where T: ApproxEq<T>,`
- `impl<U, T> ApproxEq<T> for Length<T, U> where T: ApproxEq<T>,`
