---
title: "image"
kind: module
path: "iced::widget::image"
crate: "iced"
module: "iced::widget"
file: "iced/widget/image/index.md"
docs_url: "https://docs.iced.rs/iced/widget/image/index.html"
source_file: "src/iced/lib.rs"
source_lines: "641"
---

# `iced::widget::image`

Images display raster graphics in different formats (PNG, JPG, etc.).

## Modules

- [`viewer`](viewer/index.md) — Zoom and pan on an image.

## Structs

- [`Allocation`](struct.Allocation.md) — A memory allocation of a [`Handle`](../../advanced/image/enum.Handle.md), often in GPU memory.
- [`Image`](struct.Image.md) — A frame that displays an image while keeping aspect ratio.
- [`Viewer`](struct.Viewer.md) — A frame that displays an image with the ability to zoom in/out and pan.

## Enums

- [`Error`](enum.Error.md) — An image loading error.
- [`FilterMethod`](enum.FilterMethod.md) — Image filtering strategy.
- [`Handle`](enum.Handle.md) — A handle of some image data.

## Functions

- [`allocate`](fn.allocate.md) — Allocates an image [`Handle`](../../advanced/image/enum.Handle.md).
- [`draw`](fn.draw.md) — Draws an [`Image`](../struct.Image.md)
- [`layout`](fn.layout.md) — Computes the layout of an [`Image`](../struct.Image.md).
- [`viewer`](fn.viewer.md) — Creates a new [`Viewer`](struct.Viewer.md) with the given image `Handle`.
