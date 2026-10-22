---
title: "builder"
kind: module
path: "iced::widget::canvas::path::lyon_path::builder"
crate: "iced"
module: "iced::widget::canvas::path::lyon_path"
file: "iced/widget/canvas/path/lyon_path/builder/index.md"
docs_url: "https://docs.iced.rs/iced/widget/canvas/path/lyon_path/builder/index.html"
---

# `iced::widget::canvas::path::lyon_path::builder`

Path building utilities.

### `PathBuilder` or `SvgPathBuilder`

Path can be built via either of two abstractions:

- [PathBuilder](trait.PathBuilder.md) is a simple and efficient interface which does not deal with any ambiguous cases.
- [SvgPathBuilder](trait.SvgPathBuilder.md) is a higher-level interface that follows SVG’s specification, removing the the burden of dealing with special cases from the user at a run-time cost.

`SvgPathBuilder` may be a better choice when interactive with SVG, or dealing with arbitrary input. `PathBuilder`. `PathBuilder` is probably a more useful trait to implement when creating a new path data structure since all `PathBuilder` implementations automatically get an `SvgPathBuilder` adapter (see the `with_svg` method). It may also make sense to use the `PathBuilder` API when following a specification that behaves like SVG paths or when no performance can be traded for convenience.

### Examples

The following example shows how to create a simple path using the [PathBuilder](trait.PathBuilder.md) interface.

```rust
use lyon_path::{Path, geom::point};

let mut builder = Path::builder();

// All sub-paths *must* have be contained in a begin/end pair.
builder.begin(point(0.0, 0.0));
builder.line_to(point(1.0, 0.0));
builder.quadratic_bezier_to(point(2.0, 0.0), point(2.0, 1.0));
builder.end(false);

builder.begin(point(10.0, 0.0));
builder.cubic_bezier_to(point(12.0, 2.0), point(11.0, 2.0), point(5.0, 0.0));
builder.close(); // close() is equivalent to end(true).

let path = builder.build();
```

The same path can be built using the `SvgPathBuilder` API:

```rust
use lyon_path::{Path, geom::{point, vector}, builder::SvgPathBuilder};

// Use the SVG adapter.
let mut builder = Path::builder().with_svg();

// All sub-paths *must* have be contained in a begin/end pair.
builder.move_to(point(0.0, 0.0));
builder.line_to(point(1.0, 0.0));
builder.quadratic_bezier_to(point(2.0, 0.0), point(2.0, 1.0));
// No need to explicitly end a sub-path.

builder.move_to(point(10.0, 0.0));
builder.relative_cubic_bezier_to(vector(2.0, 2.0), vector(1.0, 2.0), vector(-5.0, 0.0));
builder.close();

let path = builder.build();
```

Implementors of the `PathBuilder` trait automatically gain access to a few other adapters. For example a builder that approximates curves with a sequence of line segments:

```rust
use lyon_path::{Path, geom::point};

let tolerance = 0.05;// maximum distance between a curve and its approximation.
let mut builder = Path::builder().flattened(tolerance);

builder.begin(point(0.0, 0.0));
builder.quadratic_bezier_to(point(1.0, 0.0), point(1.0, 1.0));
builder.end(true);

// The resulting path contains only Begin, Line and End events.
let path = builder.build();
```

## Structs

- [`BorderRadii`](struct.BorderRadii.md) — The radius of each corner of a rounded rectangle.
- [`Flattened`](struct.Flattened.md) — A Builder that approximates curves with successions of line segments.
- [`NoAttributes`](struct.NoAttributes.md) — A convenience wrapper for `PathBuilder` without custom attributes.
- [`Transformed`](struct.Transformed.md) — Builds a path with a transformation applied.
- [`WithSvg`](struct.WithSvg.md) — Implements an SVG-like building interface on top of a PathBuilder.

## Traits

- [`Build`](trait.Build.md) — Builds a path.
- [`PathBuilder`](trait.PathBuilder.md) — The base path building interface.
- [`SvgPathBuilder`](trait.SvgPathBuilder.md) — A path building interface that tries to stay close to SVG’s path specification. [https://svgwg.org/specs/paths/](https://svgwg.org/specs/paths/)
