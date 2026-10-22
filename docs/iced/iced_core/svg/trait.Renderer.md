---
title: "Renderer"
kind: trait
path: "iced_core::svg::Renderer"
crate: "iced_core"
module: "iced_core::svg"
file: "iced_core/svg/trait.Renderer.md"
docs_url: "https://docs.iced.rs/iced_core/svg/trait.Renderer.html"
source_file: "src/iced_core/svg.rs"
source_lines: "153-159"
---

# `iced_core::svg::Renderer`

```rust
pub trait Renderer: Renderer {
    // Required methods
    fn measure_svg(&self, handle: &Handle) -> Size<u32>;
    fn draw_svg(&mut self, svg: Svg, bounds: Rectangle, clip_bounds: Rectangle);
}
```

A [`Renderer`](trait.Renderer.md) that can render vector graphics.

**Source:** [svg.rs](../../src/iced_core/svg.rs.md) (lines 153-159)

## Required Methods

### `measure_svg`

```rust
fn measure_svg(&self, handle: &Handle) -> Size<u32>
```

Returns the default dimensions of an SVG for the given [`Handle`](struct.Handle.md).

### `draw_svg`

```rust
fn draw_svg(&mut self, svg: Svg, bounds: Rectangle, clip_bounds: Rectangle)
```

Draws an SVG with the given [`Handle`](struct.Handle.md), an optional [`Color`](../color/struct.Color.md) filter, and inside the provided `bounds`.

## Implementors
