---
title: "lyon_path"
kind: module
path: "iced_renderer::geometry::path::lyon_path"
crate: "iced_renderer"
module: "iced_renderer::geometry::path"
file: "iced_renderer/geometry/path/lyon_path/index.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/path/lyon_path/index.html"
---

# `iced_renderer::geometry::path::lyon_path`

Data structures and traits to work with paths (vector graphics).

To build and consume paths, see the [builder](builder/index.md) and [iterator](iterator/index.md) modules.

This crate is reexported in [lyon](https://docs.rs/lyon/).

## Examples

```rust
use lyon_path::Path;
use lyon_path::math::{point};
use lyon_path::builder::*;

// Create a builder object to build the path.
let mut builder = Path::builder();

// Build a simple path.
let mut builder = Path::builder();
builder.begin(point(0.0, 0.0));
builder.line_to(point(1.0, 2.0));
builder.line_to(point(2.0, 0.0));
builder.line_to(point(1.0, 1.0));
builder.close();

// Generate the actual path object.
let path = builder.build();

for event in &path {
    println!("{:?}", event);
}
```

## Modules

- [`builder`](builder/index.md) — Path building utilities.
- [`commands`](commands/index.md) — A generic representation for paths that allow more control over how endpoints and control points are stored.
- [`geom`](geom/index.md) — Simple 2D geometric primitives on top of euclid.
- [`iterator`](iterator/index.md) — Tools to iterate over paths.
- [`math`](math/index.md) — f32 version of the lyon_geom types used everywhere. Most other lyon crates reexport them.
- [`path`](path/index.md) — The default path data structure.
- [`path_buffer`](path_buffer/index.md) — A container to store multiple paths contiguously.
- [`polygon`](polygon/index.md) — Specific path types for polygons.
- [`traits`](traits/index.md) — `lyon_path` traits reexported here for convenience.

## Structs

- [`ArcFlags`](struct.ArcFlags.md) — Flag parameters for arcs as described by the SVG specification.
- [`AttributeSlice`](struct.AttributeSlice.md) — A view over a contiguous storage of custom attributes.
- [`BuilderImpl`](struct.BuilderImpl.md) — The default builder for `Path`.
- [`BuilderWithAttributes`](struct.BuilderWithAttributes.md) — A builder for `Path` with custom attributes.
- [`ControlPointId`](struct.ControlPointId.md) — ID of a control point in a path.
- [`EndpointId`](struct.EndpointId.md) — ID of an endpoint point in a path.
- [`EventId`](struct.EventId.md) — Refers to an event in a path.
- [`IdIter`](struct.IdIter.md) — An iterator of endpoint and control point ids for `Path` and `PathSlice`.
- [`IdPolygon`](struct.IdPolygon.md) — A view over a sequence of endpoint IDs forming a polygon.
- [`Iter`](struct.Iter.md) — An iterator for `Path` and `PathSlice`.
- [`IterWithAttributes`](struct.IterWithAttributes.md) — An iterator for `Path` and `PathSlice`.
- [`Path`](struct.Path.md) — A simple path data structure.
- [`PathBuffer`](struct.PathBuffer.md) — An object that stores multiple paths contiguously.
- [`PathBufferSlice`](struct.PathBufferSlice.md) — A view on a `PathBuffer`.
- [`PathCommands`](struct.PathCommands.md) — The commands of a path encoded in a single array using IDs to refer to endpoints and control points externally.
- [`PathCommandsSlice`](struct.PathCommandsSlice.md) — A view over [`PathCommands`](struct.PathCommands.md).
- [`PathSlice`](struct.PathSlice.md) — A view on a `Path`.
- [`Polygon`](struct.Polygon.md) — A view over a sequence of endpoints forming a polygon.
- [`Reversed`](struct.Reversed.md) — An iterator of over a `Path` traversing the path in reverse.

## Enums

- [`Event`](enum.Event.md) — Represents an event or edge of path.
- [`FillRule`](enum.FillRule.md) — The fill rule defines how to determine what is inside and what is outside of the shape.
- [`LineCap`](enum.LineCap.md) — Line cap as defined by the SVG specification.
- [`LineJoin`](enum.LineJoin.md) — Line join as defined by the SVG specification.
- [`Side`](enum.Side.md) — The positive or negative side of a vector or segment.
- [`Winding`](enum.Winding.md) — The two possible orientations for the edges of a shape to be built in.

## Constants

- [`NO_ATTRIBUTES`](constant.NO_ATTRIBUTES.md) — An empty attribute slice.

## Traits

- [`AttributeStore`](trait.AttributeStore.md) — Interface for objects storing custom attributes associated with endpoints.
- [`Position`](trait.Position.md) — Interface for types types (typically endpoints and control points) that have a 2D position.
- [`PositionStore`](trait.PositionStore.md) — Interface for objects storing endpoints and control points positions.

## Type Aliases

- [`AttributeIndex`](type.AttributeIndex.md) — An alias for `usize`.
- [`Attributes`](type.Attributes.md) — An alias for a slice of `f32` values.
- [`Builder`](type.Builder.md)
- [`IdEvent`](type.IdEvent.md) — A path event representing endpoints and control points as IDs.
- [`PathEvent`](type.PathEvent.md) — A path event representing endpoints and control points as positions.
