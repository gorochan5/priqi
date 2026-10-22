---
title: "iced_wgpu"
kind: crate
path: "iced_wgpu"
crate: "iced_wgpu"
file: "iced_wgpu/index.md"
docs_url: "https://docs.iced.rs/iced_wgpu/index.html"
source_file: "src/iced_wgpu/lib.rs"
source_lines: "1-966"
---

# `iced_wgpu`

A [`wgpu`](https://github.com/gfx-rs/wgpu-rs) renderer for [Iced](https://github.com/iced-rs/iced).

![The native path of the Iced ecosystem](https://github.com/iced-rs/iced/blob/0525d76ff94e828b7b21634fa94a747022001c83/docs/graphs/native.png?raw=true)

[`wgpu`](https://github.com/gfx-rs/wgpu-rs) supports most modern graphics backends: Vulkan, Metal, DX11, and DX12 (OpenGL and WebGL are still WIP). Additionally, it will support the incoming [WebGPU API](https://gpuweb.github.io/gpuweb/).

Currently, `iced_wgpu` supports the following primitives:

- Text, which is rendered using [`glyphon`](https://github.com/grovesNL/glyphon).
- Quads or rectangles, with rounded borders and a solid background color.
- Clip areas, useful to implement scrollables or hide overflowing content.
- Images and SVG, loaded from memory or the file system.
- Meshes of triangles, useful to draw geometry freely.

## Re-exports

- [`Layer`](layer/struct.Layer.md)
- [`Primitive`](primitive/trait.Primitive.md)
- [`Geometry`](geometry/enum.Geometry.md) *(geometry)*
- `pub use iced_graphics as graphics;`
- `pub use iced_graphics::core;`
- `pub use wgpu;`

## Modules

- [`geometry`](geometry/index.md) *(geometry)* — Build and draw geometry.
- [`layer`](layer/index.md)
- [`primitive`](primitive/index.md) — Draw custom primitives.
- [`window`](window/index.md) — Display rendering results on windows.

## Structs

- [`Engine`](struct.Engine.md)
- [`Renderer`](struct.Renderer.md) — A [`wgpu`](https://github.com/gfx-rs/wgpu-rs) graphics renderer for [`iced`](https://github.com/iced-rs/iced).
