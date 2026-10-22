---
title: "euclid"
kind: module
path: "iced_renderer::geometry::path::lyon_path::geom::euclid"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path::geom"
file: "iced_renderer/geometry/path/lyon_path/geom/euclid/index.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/euclid/index.html"
---

# `iced_renderer::geometry::path::lyon_path::geom::euclid`

A collection of strongly typed math tools for computer graphics with an inclination towards 2d graphics and layout.

All types are generic over the scalar type of their component (`f32`, `i32`, etc.), and tagged with a generic Unit parameter which is useful to prevent mixing values from different spaces. For example it should not be legal to translate a screen-space position by a world-space vector and this can be expressed using the generic Unit parameter.

This unit system is not mandatory and all structures have an alias with the default unit: `UnknownUnit`. for example `default::Point2D<T>` is equivalent to `Point2D<T, UnknownUnit>`. Client code typically creates a set of aliases for each type and doesn’t need to deal with the specifics of typed units further. For example:

```rust
use euclid::*;
pub struct ScreenSpace;
pub type ScreenPoint = Point2D<f32, ScreenSpace>;
pub type ScreenSize = Size2D<f32, ScreenSpace>;
pub struct WorldSpace;
pub type WorldPoint = Point3D<f32, WorldSpace>;
pub type ProjectionMatrix = Transform3D<f32, WorldSpace, ScreenSpace>;
// etc...
```

All euclid types are marked `#[repr(C)]` in order to facilitate exposing them to foreign function interfaces (provided the underlying scalar type is also `repr(C)`).

## Modules

- [`approxeq`](approxeq/index.md) *(std or libm)*
- [`approxord`](approxord/index.md) — Utilities for testing approximate ordering - especially true for floating point types, where NaN’s cannot be ordered.
- [`default`](default/index.md) — A set of aliases for all types, tagged with the default unknown unit.
- [`num`](num/index.md) — A one-dimensional length, tagged with its units.

## Structs

- [`Angle`](struct.Angle.md) — An angle in radians
- [`BoolVector2D`](struct.BoolVector2D.md) — A 2d vector of booleans, useful for component-wise logic operations.
- [`BoolVector3D`](struct.BoolVector3D.md) — A 3d vector of booleans, useful for component-wise logic operations.
- [`Box2D`](struct.Box2D.md) — A 2d axis aligned rectangle represented by its minimum and maximum coordinates.
- [`Box3D`](struct.Box3D.md) — An axis aligned 3D box represented by its minimum and maximum coordinates.
- [`HomogeneousVector`](struct.HomogeneousVector.md) — Homogeneous vector in 3D space.
- [`Length`](struct.Length.md) — A one-dimensional distance, with value represented by `T` and unit of measurement `Unit`.
- [`Point2D`](struct.Point2D.md) — A 2d Point tagged with a unit.
- [`Point3D`](struct.Point3D.md) — A 3d Point tagged with a unit.
- [`Rect`](struct.Rect.md) — A 2d Rectangle optionally tagged with a unit.
- [`RigidTransform3D`](struct.RigidTransform3D.md) — A rigid transformation. All lengths are preserved under such a transformation.
- [`Rotation2D`](struct.Rotation2D.md) — A transform that can represent rotations in 2d, represented as an angle in radians.
- [`Rotation3D`](struct.Rotation3D.md) — A transform that can represent rotations in 3d, represented as a quaternion.
- [`Scale`](struct.Scale.md) — A scaling factor between two different units of measurement.
- [`ScaleOffset2D`](struct.ScaleOffset2D.md)
- [`SideOffsets2D`](struct.SideOffsets2D.md) — A group of 2D side offsets, which correspond to top/right/bottom/left for borders, padding, and margins in CSS, optionally tagged with a unit.
- [`Size2D`](struct.Size2D.md) — A 2d size tagged with a unit.
- [`Size3D`](struct.Size3D.md) — A 3d size tagged with a unit.
- [`Transform2D`](struct.Transform2D.md) — A 2d transform represented by a column-major 3 by 3 matrix, compressed down to 3 by 2.
- [`Transform3D`](struct.Transform3D.md) — A 3d transform stored as a column-major 4 by 4 matrix.
- [`Translation2D`](struct.Translation2D.md) — A 2d transformation from a space to another that can only express translations.
- [`Translation3D`](struct.Translation3D.md) — A 3d transformation from a space to another that can only express translations.
- [`UnknownUnit`](struct.UnknownUnit.md) — The default unit.
- [`Vector2D`](struct.Vector2D.md) — A 2d Vector tagged with a unit.
- [`Vector3D`](struct.Vector3D.md) — A 3d Vector tagged with a unit.

## Traits

- [`Trig`](trait.Trig.md) — Trait for basic trigonometry functions, so they can be used on generic numeric types

## Functions

- [`box3d`](fn.box3d.md) — Shorthand for `Box3D::new(Point3D::new(x1, y1, z1), Point3D::new(x2, y2, z2))`.
- [`bvec2`](fn.bvec2.md) — Shorthand for `BoolVector2D { x, y }`.
- [`bvec3`](fn.bvec3.md) — Shorthand for `BoolVector3D { x, y, z }`.
- [`point2`](fn.point2.md) — Shorthand for `Point2D::new(x, y)`.
- [`point3`](fn.point3.md) — Shorthand for `Point3D::new(x, y)`.
- [`rect`](fn.rect.md) — Shorthand for `Rect::new(Point2D::new(x, y), Size2D::new(w, h))`.
- [`size2`](fn.size2.md) — Shorthand for `Size2D::new(w, h)`.
- [`size3`](fn.size3.md) — Shorthand for `Size3D::new(w, h, d)`.
- [`vec2`](fn.vec2.md) — Convenience constructor.
- [`vec3`](fn.vec3.md) — Convenience constructor.
