---
title: "geometry"
kind: module
path: "iced_graphics::geometry"
crate: "iced_graphics"
module: "iced_graphics"
file: "iced_graphics/geometry/index.md"
docs_url: "https://docs.iced.rs/iced_graphics/geometry/index.html"
source_file: "src/iced_graphics/geometry.rs"
source_lines: "1-48"
---

# `iced_graphics::geometry`

Build and draw geometry.

## Re-exports

- [`Fill`](fill/struct.Fill.md)
- [`Frame`](frame/struct.Frame.md)
- [`Path`](path/struct.Path.md)
- [`LineCap`](stroke/enum.LineCap.md)
- [`LineDash`](stroke/struct.LineDash.md)
- [`LineJoin`](stroke/enum.LineJoin.md)
- [`Stroke`](stroke/struct.Stroke.md)
- [`gradient`](../gradient/index.md)
- [`Gradient`](../gradient/enum.Gradient.md)

## Modules

- [`fill`](fill/index.md) — Fill [`Geometry`](trait.Renderer.md) with a certain style.
- [`frame`](frame/index.md) — Draw and generate geometry.
- [`path`](path/index.md) — Build different kinds of 2D shapes.
- [`stroke`](stroke/index.md) — Create lines from a [`Path`](path/struct.Path.md) and assigns them various attributes/styles.

## Structs

- [`Cache`](struct.Cache.md) — A simple cache that stores generated geometry to avoid recomputation.
- [`Image`](struct.Image.md) — A raster image that can be drawn.
- [`Svg`](struct.Svg.md) — A raster image that can be drawn.
- [`Text`](struct.Text.md) — A bunch of text that can be drawn to a canvas

## Enums

- [`Style`](enum.Style.md) — The coloring style of some drawing.

## Traits

- [`Renderer`](trait.Renderer.md) — A renderer capable of drawing some [`Self::Geometry`](trait.Renderer.md).
