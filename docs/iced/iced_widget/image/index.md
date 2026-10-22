---
title: "image"
kind: module
path: "iced_widget::image"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/image/index.md"
docs_url: "https://docs.iced.rs/iced_widget/image/index.html"
source_file: "src/iced_widget/image.rs"
source_lines: "1-386"
---

# `iced_widget::image`

Images display raster graphics in different formats (PNG, JPG, etc.).

## Example

```rust
use iced::widget::image;

enum Message {
    // ...
}

fn view(state: &State) -> impl Widget<Message> {
    image("ferris.png")
}
```

![](https://github.com/iced-rs/iced/blob/9712b319bb7a32848001b96bd84977430f14b623/examples/resources/ferris.png?raw=true)

## Re-exports

- [`Viewer`](viewer/struct.Viewer.md)

## Modules

- [`viewer`](viewer/index.md) — Zoom and pan on an image.

## Structs

- [`Image`](struct.Image.md) — A frame that displays an image while keeping aspect ratio.

## Enums

- [`FilterMethod`](enum.FilterMethod.md) — Image filtering strategy.
- [`Handle`](enum.Handle.md) — A handle of some image data.

## Functions

- [`draw`](fn.draw.md) — Draws an [`Image`](struct.Image.md)
- [`layout`](fn.layout.md) — Computes the layout of an [`Image`](struct.Image.md).
- [`viewer`](fn.viewer.md) — Creates a new [`Viewer`](viewer/struct.Viewer.md) with the given image `Handle`.
