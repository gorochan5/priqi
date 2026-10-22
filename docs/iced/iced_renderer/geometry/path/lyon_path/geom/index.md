---
title: "geom"
kind: module
path: "iced_renderer::geometry::path::lyon_path::geom"
crate: "iced_renderer"
module: "iced_renderer::geometry::path::lyon_path"
file: "iced_renderer/geometry/path/lyon_path/geom/index.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/geom/index.html"
---

# `iced_renderer::geometry::path::lyon_path::geom`

Simple 2D geometric primitives on top of euclid.

This crate is reexported in [lyon](https://docs.rs/lyon/).

## Overview.

This crate implements some of the maths to work with:

- lines and line segments,
- quadratic and cubic bézier curves,
- elliptic arcs,
- triangles.

## Flattening

Flattening is the action of approximating a curve with a succession of line segments.

→

The tolerance threshold taken as input by the flattening algorithms corresponds to the maximum distance between the curve and its linear approximation. The smaller the tolerance is, the more precise the approximation and the more segments are generated. This value is typically chosen in function of the zoom level.

The figure above shows a close up on a curve (the dotted line) and its linear approximation (the black segments). The tolerance threshold is represented by the light green area and the orange arrow.

## Modules

- [`arc`](arc/index.md) — Elliptic arc related maths and tools.
- [`arrayvec`](arrayvec/index.md) — **arrayvec** provides the types [`ArrayVec`](arrayvec/struct.ArrayVec.md) and [`ArrayString`](arrayvec/struct.ArrayString.md): array-backed vector and string types, which store their contents inline.
- [`cubic_bezier`](cubic_bezier/index.md)
- [`euclid`](euclid/index.md) — A collection of strongly typed math tools for computer graphics with an inclination towards 2d graphics and layout.
- [`quadratic_bezier`](quadratic_bezier/index.md)
- [`traits`](traits/index.md)
- [`utils`](utils/index.md)

## Structs

- [`Angle`](struct.Angle.md) — An angle in radians
- [`Arc`](struct.Arc.md) — An elliptic arc curve segment.
- [`ArcFlags`](struct.ArcFlags.md) — Flag parameters for arcs as described by the SVG specification.
- [`CubicBezierSegment`](struct.CubicBezierSegment.md) — A 2d curve segment defined by four points: the beginning of the segment, two control points and the end of the segment.
- [`Line`](struct.Line.md) — An infinite line defined by a point and a vector.
- [`LineEquation`](struct.LineEquation.md) — A line defined by the equation `a * x + b * y + c = 0; a * a + b * b = 1`.
- [`LineSegment`](struct.LineSegment.md) — A linear segment.
- [`QuadraticBezierSegment`](struct.QuadraticBezierSegment.md) — A 2d curve segment defined by three points: the beginning of the segment, a control point and the end of the segment.
- [`SvgArc`](struct.SvgArc.md) — An elliptic arc curve segment using the SVG’s end-point notation.
- [`Triangle`](struct.Triangle.md) — A 2D triangle defined by three points `a`, `b` and `c`.

## Traits

- [`Scalar`](trait.Scalar.md)
- [`Segment`](trait.Segment.md) — Common APIs to segment types.

## Functions

- [`point`](fn.point.md) — Shorthand for `Point::new(x, y)`.
- [`size`](fn.size.md) — Shorthand for `Size::new(x, y)`.
- [`vector`](fn.vector.md) — Shorthand for `Vector::new(x, y)`.

## Type Aliases

- [`Box2D`](type.Box2D.md)
- [`Point`](type.Point.md)
- [`Rotation`](type.Rotation.md) — Alias for `euclid::default::Rotation2D`
- [`Scale`](type.Scale.md)
- [`Size`](type.Size.md)
- [`Transform`](type.Transform.md) — Alias for `euclid::default::Transform2D`
- [`Translation`](type.Translation.md) — Alias for `euclid::default::Translation2D`
- [`Vector`](type.Vector.md)
