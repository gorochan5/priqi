---
title: "geometry"
kind: module
path: "iced_renderer::geometry"
crate: "iced_renderer"
module: "iced_renderer"
file: "iced_renderer/geometry/index.md"
docs_url: "https://docs.iced.rs/iced_renderer/geometry/index.html"
---

# `iced_renderer::geometry`

Build and draw geometry.

## Modules

- [`fill`](fill/index.md) — Fill [`Geometry`](trait.Renderer.md) with a certain style.
- [`frame`](frame/index.md) — Draw and generate geometry.
- [`gradient`](gradient/index.md) — A gradient that can be used as a fill for some geometry.
- [`path`](path/index.md) — Build different kinds of 2D shapes.
- [`stroke`](stroke/index.md) — Create lines from a [`Path`](struct.Path.md) and assigns them various attributes/styles.

## Structs

- [`Cache`](struct.Cache.md) — A simple cache that stores generated geometry to avoid recomputation.
- [`Fill`](struct.Fill.md) — The style used to fill geometry.
- [`Frame`](struct.Frame.md) — The region of a surface that can be used to draw geometry.
- [`Image`](struct.Image.md) — A raster image that can be drawn.
- [`LineDash`](struct.LineDash.md) — The dash pattern used when stroking the line.
- [`Path`](struct.Path.md) — An immutable set of points that may or may not be connected.
- [`Stroke`](struct.Stroke.md) — The style of a stroke.
- [`Svg`](struct.Svg.md) — A raster image that can be drawn.
- [`Text`](struct.Text.md) — A bunch of text that can be drawn to a canvas

## Enums

- [`Gradient`](enum.Gradient.md) — A fill which linearly interpolates colors along a direction.
- [`LineCap`](enum.LineCap.md) — The shape used at the end of open subpaths when they are stroked.
- [`LineJoin`](enum.LineJoin.md) — The shape used at the corners of paths or basic shapes when they are stroked.
- [`Style`](enum.Style.md) — The coloring style of some drawing.

## Traits

- [`Renderer`](trait.Renderer.md) — A renderer capable of drawing some [`Self::Geometry`](trait.Renderer.md).
