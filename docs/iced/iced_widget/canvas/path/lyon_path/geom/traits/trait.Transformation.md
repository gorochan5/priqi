---
title: "Transformation"
kind: trait
path: "iced_widget::canvas::path::lyon_path::geom::traits::Transformation"
crate: "iced_widget"
module: "iced_widget::canvas::path::lyon_path::geom::traits"
file: "iced_widget/canvas/path/lyon_path/geom/traits/trait.Transformation.md"
docs_url: "https://docs.iced.rs/iced_widget/canvas/path/lyon_path/geom/traits/trait.Transformation.html"
---

# `iced_widget::canvas::path::lyon_path::geom::traits::Transformation`

```rust
pub trait Transformation<S> {
    // Required methods
    fn transform_point(
        &self,
        p: Point2D<S, UnknownUnit>,
    ) -> Point2D<S, UnknownUnit>;
    fn transform_vector(
        &self,
        v: Vector2D<S, UnknownUnit>,
    ) -> Vector2D<S, UnknownUnit>;
}
```

## Required Methods

### `transform_point`

```rust
fn transform_point(&self, p: Point2D<S, UnknownUnit>) -> Point2D<S, UnknownUnit>
```

### `transform_vector`

```rust
fn transform_vector( &self, v: Vector2D<S, UnknownUnit>, ) -> Vector2D<S, UnknownUnit>
```

## Implementors

- `impl<S> Transformation<S> for Rotation2D<S, UnknownUnit, UnknownUnit> where S: Scalar,`
- `impl<S> Transformation<S> for Scale<S, UnknownUnit, UnknownUnit> where S: Scalar,`
- `impl<S> Transformation<S> for Transform2D<S, UnknownUnit, UnknownUnit> where S: Scalar,`
- `impl<S> Transformation<S> for Translation2D<S, UnknownUnit, UnknownUnit> where S: Scalar,`
