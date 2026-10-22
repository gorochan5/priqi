---
title: "layout"
kind: function
path: "iced_widget::image::layout"
crate: "iced_widget"
module: "iced_widget::image"
file: "iced_widget/image/fn.layout.md"
docs_url: "https://docs.iced.rs/iced_widget/image/fn.layout.html"
source_file: "src/iced_widget/image.rs"
source_lines: "179-220"
---

# `iced_widget::image::layout`

```rust
pub fn layout<Renderer, Handle>(
    renderer: &Renderer,
    limits: &Limits,
    handle: &Handle,
    width: Length,
    height: Length,
    region: Option<Rectangle<u32>>,
    content_fit: ContentFit,
    rotation: Rotation,
    expand: bool,
) -> Sizewhere
    Renderer: Renderer<Handle = Handle>,
```

Computes the layout of an [`Image`](struct.Image.md).

**Source:** [image.rs](../../src/iced_widget/image.rs.md) (lines 179-220)
