---
title: "viewer"
kind: module
path: "iced_widget::image::viewer"
crate: "iced_widget"
module: "iced_widget::image"
file: "iced_widget/image/viewer/index.md"
docs_url: "https://docs.iced.rs/iced_widget/image/viewer/index.html"
source_file: "src/iced_widget/image/viewer.rs"
source_lines: "1-414"
---

# `iced_widget::image::viewer`

Zoom and pan on an image.

## Structs

- [`State`](struct.State.md) — The local state of a [`Viewer`](struct.Viewer.md).
- [`Viewer`](struct.Viewer.md) — A frame that displays an image with the ability to zoom in/out and pan.

## Functions

- [`scaled_image_size`](fn.scaled_image_size.md) — Returns the bounds of the underlying image, given the bounds of the [`Viewer`](struct.Viewer.md). Scaling will be applied and original aspect ratio will be respected.
