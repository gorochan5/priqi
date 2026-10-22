---
title: "image"
kind: module
path: "iced_core::image"
crate: "iced_core"
module: "iced_core"
file: "iced_core/image/index.md"
docs_url: "https://docs.iced.rs/iced_core/image/index.html"
source_file: "src/iced_core/image.rs"
source_lines: "1-339"
---

# `iced_core::image`

Load and draw raster graphics.

## Structs

- [`Allocation`](struct.Allocation.md) — A memory allocation of a [`Handle`](enum.Handle.md), often in GPU memory.
- [`Id`](struct.Id.md) — The unique identifier of some [`Handle`](enum.Handle.md) data.
- [`Image`](struct.Image.md) — A raster image that can be drawn.
- [`Memory`](struct.Memory.md) — Some memory taken by an [`Allocation`](struct.Allocation.md).

## Enums

- [`Error`](enum.Error.md) — An image loading error.
- [`FilterMethod`](enum.FilterMethod.md) — Image filtering strategy.
- [`Handle`](enum.Handle.md) — A handle of some image data.

## Traits

- [`Renderer`](trait.Renderer.md) — A [`Renderer`](trait.Renderer.md) that can render raster graphics.

## Functions

- [`allocate`](fn.allocate.md) — Creates a new [`Allocation`](struct.Allocation.md) for the given handle.
