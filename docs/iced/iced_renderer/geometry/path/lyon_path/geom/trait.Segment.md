---
title: "Segment"
kind: trait
path: "iced_renderer::geometry::path::lyon_path::geom::Segment"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom"
file: "iced_renderer/geometry/path/lyon_path/geom/trait.Segment.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/trait.Segment.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::Segment`

```rust
pub trait Segment: Sized + Copy {
    type Scalar: Scalar;

Show 15 methods    // Required methods
    fn from(&self) -> Point2D<Self::Scalar, UnknownUnit>;
    fn to(&self) -> Point2D<Self::Scalar, UnknownUnit>;
    fn sample(&self, t: Self::Scalar) -> Point2D<Self::Scalar, UnknownUnit>;
    fn derivative(&self, t: Self::Scalar) -> Vector2D<Self::Scalar, UnknownUnit>;
    fn split(&self, t: Self::Scalar) -> (Self, Self);
    fn before_split(&self, t: Self::Scalar) -> Self;
    fn after_split(&self, t: Self::Scalar) -> Self;
    fn split_range(&self, t_range: Range<Self::Scalar>) -> Self;
    fn flip(&self) -> Self;
    fn approximate_length(&self, tolerance: Self::Scalar) -> Self::Scalar;
    fn for_each_flattened_with_t(
        &self,
        tolerance: Self::Scalar,
        callback: &mut dyn FnMut(&LineSegment<Self::Scalar>, Range<Self::Scalar>),
    );

    // Provided methods
    fn x(&self, t: Self::Scalar) -> Self::Scalar { ... }
    fn y(&self, t: Self::Scalar) -> Self::Scalar { ... }
    fn dx(&self, t: Self::Scalar) -> Self::Scalar { ... }
    fn dy(&self, t: Self::Scalar) -> Self::Scalar { ... }
}
```

Common APIs to segment types.

## Required Associated Types

### `Scalar`

```rust
type Scalar: Scalar
```

## Required Methods

### `from`

```rust
fn from(&self) -> Point2D<Self::Scalar, UnknownUnit>
```

Start of the curve.

### `to`

```rust
fn to(&self) -> Point2D<Self::Scalar, UnknownUnit>
```

End of the curve.

### `sample`

```rust
fn sample(&self, t: Self::Scalar) -> Point2D<Self::Scalar, UnknownUnit>
```

Sample the curve at t (expecting t between 0 and 1).

### `derivative`

```rust
fn derivative(&self, t: Self::Scalar) -> Vector2D<Self::Scalar, UnknownUnit>
```

Sample the derivative at t (expecting t between 0 and 1).

### `split`

```rust
fn split(&self, t: Self::Scalar) -> (Self, Self)
```

Split this curve into two sub-curves.

### `before_split`

```rust
fn before_split(&self, t: Self::Scalar) -> Self
```

Return the curve before the split point.

### `after_split`

```rust
fn after_split(&self, t: Self::Scalar) -> Self
```

Return the curve after the split point.

### `split_range`

```rust
fn split_range(&self, t_range: Range<Self::Scalar>) -> Self
```

Return the curve inside a given range of t.

This is equivalent splitting at the range’s end points.

### `flip`

```rust
fn flip(&self) -> Self
```

Swap the direction of the segment.

### `approximate_length`

```rust
fn approximate_length(&self, tolerance: Self::Scalar) -> Self::Scalar
```

Compute the length of the segment using a flattened approximation.

### `for_each_flattened_with_t`

```rust
fn for_each_flattened_with_t( &self, tolerance: Self::Scalar, callback: &mut dyn FnMut(&LineSegment<Self::Scalar>, Range<Self::Scalar>), )
```

Approximates the curve with sequence of line segments.

The `tolerance` parameter defines the maximum distance between the curve and its approximation.

The parameter `t` at the final segment is guaranteed to be equal to `1.0`.

## Provided Methods

### `x`

```rust
fn x(&self, t: Self::Scalar) -> Self::Scalar
```

Sample x at t (expecting t between 0 and 1).

### `y`

```rust
fn y(&self, t: Self::Scalar) -> Self::Scalar
```

Sample y at t (expecting t between 0 and 1).

### `dx`

```rust
fn dx(&self, t: Self::Scalar) -> Self::Scalar
```

Sample x derivative at t (expecting t between 0 and 1).

### `dy`

```rust
fn dy(&self, t: Self::Scalar) -> Self::Scalar
```

Sample y derivative at t (expecting t between 0 and 1).

## Implementors

- `impl<S> Segment for Arc<S> where S: Scalar,`
  - `type Scalar = S`
- `impl<S> Segment for CubicBezierSegment<S> where S: Scalar,`
  - `type Scalar = S`
- `impl<S> Segment for LineSegment<S> where S: Scalar,`
  - `type Scalar = S`
- `impl<S> Segment for QuadraticBezierSegment<S> where S: Scalar,`
  - `type Scalar = S`
