---
title: "compositor"
kind: module
path: "iced_graphics::compositor"
crate: "iced_graphics"
module: "iced_graphics"
file: "iced_graphics/compositor/index.md"
docs_url: "https://docs.iced.rs/iced_graphics/compositor/index.html"
source_file: "src/iced_graphics/compositor.rs"
source_lines: "1-221"
---

# `iced_graphics::compositor`

A compositor is responsible for initializing a renderer and managing window surfaces.

## Structs

- [`Information`](struct.Information.md) — Contains information about the graphics (e.g. graphics adapter, graphics backend).

## Enums

- [`SurfaceError`](enum.SurfaceError.md) — Result of an unsuccessful call to [`Compositor::present`](trait.Compositor.md).

## Traits

- [`Compositor`](trait.Compositor.md) — A graphics compositor that can draw to windows.
- [`Default`](trait.Default.md) — Defines the default compositor of a renderer.
- [`Display`](trait.Display.md) — An owned display handle that can be used in a [`Compositor`](trait.Compositor.md).
- [`Window`](trait.Window.md) — A window that can be used in a [`Compositor`](trait.Compositor.md).
