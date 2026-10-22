---
title: "iced_graphics"
kind: crate
path: "iced_graphics"
crate: "iced_graphics"
file: "iced_graphics/index.md"
docs_url: "https://docs.iced.rs/iced_graphics/index.html"
source_file: "src/iced_graphics/lib.rs"
source_lines: "1-40"
---

# `iced_graphics`

A bunch of backend-agnostic types that can be leveraged to build a renderer for [`iced`](https://github.com/iced-rs/iced).

![The native path of the Iced ecosystem](https://github.com/iced-rs/iced/blob/0525d76ff94e828b7b21634fa94a747022001c83/docs/graphs/native.png?raw=true)

## Re-exports

- [`Cache`](cache/struct.Cache.md)
- [`Compositor`](compositor/trait.Compositor.md)
- [`Gradient`](gradient/enum.Gradient.md)
- [`Image`](image/enum.Image.md)
- [`Layer`](layer/trait.Layer.md)
- [`Mesh`](mesh/enum.Mesh.md)
- [`Shell`](shell/struct.Shell.md)
- [`Text`](text/enum.Text.md)
- `pub use iced_core as core;`
- `pub use iced_futures as futures;`

## Modules

- [`cache`](cache/index.md) — Cache computations and efficiently reuse them.
- [`color`](color/index.md) — Manage colors for shaders.
- [`compositor`](compositor/index.md) — A compositor is responsible for initializing a renderer and managing window surfaces.
- [`damage`](damage/index.md) — Compute the damage between frames.
- [`geometry`](geometry/index.md) *(geometry)* — Build and draw geometry.
- [`gradient`](gradient/index.md) — A gradient that can be used as a fill for some geometry.
- [`image`](image/index.md) — Load and operate on images.
- [`layer`](layer/index.md) — Draw and stack layers of graphical primitives.
- [`mesh`](mesh/index.md) — Draw triangles!
- [`shell`](shell/index.md) — Control the windowing runtime from a renderer.
- [`text`](text/index.md) — Draw text.

## Structs

- [`Viewport`](struct.Viewport.md) — A viewing region for displaying computer graphics.

## Enums

- [`Antialiasing`](enum.Antialiasing.md) — An antialiasing strategy.
