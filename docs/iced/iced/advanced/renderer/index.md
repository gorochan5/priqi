---
title: "renderer"
kind: module
path: "iced::advanced::renderer"
crate: "iced"
module: "iced::advanced"
file: "iced/advanced/renderer/index.md"
docs_url: "https://docs.iced.rs/iced/advanced/renderer/index.html"
---

# `iced::advanced::renderer`

Write your own renderer.

## Structs

- [`Quad`](struct.Quad.md) — A polygon with four sides.
- [`Scale`](struct.Scale.md) — The scale factor of a [`Renderer`](../trait.Renderer.md).
- [`Settings`](struct.Settings.md) — The settings of a [`Renderer`](../trait.Renderer.md).
- [`Style`](struct.Style.md) — The styling attributes of a [`Renderer`](../trait.Renderer.md).

## Constants

- [`CRISP`](constant.CRISP.md) — Whether anti-aliasing should be avoided by snapping primitive coordinates to the pixel grid.

## Traits

- [`Headless`](trait.Headless.md) — A headless renderer is a renderer that can render offscreen without a window nor a compositor.
- [`Renderer`](trait.Renderer.md) — A component that can be used by widgets to draw themselves on a screen.
