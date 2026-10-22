---
title: "draw"
kind: function
path: "iced_widget::image::draw"
crate: "iced_widget"
module: "iced_widget::image"
file: "iced_widget/image/fn.draw.md"
docs_url: "https://docs.iced.rs/iced_widget/image/fn.draw.html"
source_file: "src/iced_widget/image.rs"
source_lines: "303-333"
---

# `iced_widget::image::draw`

```rust
pub fn draw<Renderer, Handle>(
    renderer: &mut Renderer,
    layout: Layout,
    handle: &Handle,
    crop: Option<Rectangle<u32>>,
    border_radius: Radius,
    content_fit: ContentFit,
    filter_method: FilterMethod,
    rotation: Rotation,
    opacity: f32,
    scale: f32,
)where
    Renderer: Renderer<Handle = Handle>,
    Handle: Clone,
```

Draws an [`Image`](struct.Image.md)

**Source:** [image.rs](../../src/iced_widget/image.rs.md) (lines 303-333)
