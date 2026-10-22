---
title: "viewer"
kind: module
path: "iced::widget::image::viewer"
crate: "iced"
module: "iced::widget::image"
file: "iced/widget/image/viewer/index.md"
docs_url: "https://docs.iced.rs/iced/widget/image/viewer/index.html"
source_file: "src/iced_widget/image.rs"
source_lines: "19"
---

# `iced::widget::image::viewer`

Zoom and pan on an image.

## Structs

- [`State`](struct.State.md) — The local state of a [`Viewer`](../struct.Viewer.md).
- [`Viewer`](struct.Viewer.md) — A frame that displays an image with the ability to zoom in/out and pan.

## Functions

- [`scaled_image_size`](fn.scaled_image_size.md) — Returns the bounds of the underlying image, given the bounds of the [`Viewer`](../struct.Viewer.md). Scaling will be applied and original aspect ratio will be respected.
