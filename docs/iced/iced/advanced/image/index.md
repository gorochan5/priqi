---
title: "image"
kind: module
path: "iced::advanced::image"
crate: "iced"
module: "iced::advanced"
file: "iced/advanced/image/index.md"
docs_url: "https://docs.iced.rs/iced/advanced/image/index.html"
---

# `iced::advanced::image`

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
