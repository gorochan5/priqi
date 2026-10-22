---
title: "scaled_image_size"
kind: function
path: "iced_widget::image::viewer::scaled_image_size"
crate: "iced_widget"
module: "iced_widget::image::viewer"
file: "iced_widget/image/viewer/fn.scaled_image_size.md"
docs_url: "https://docs.iced.rs/iced_widget/image/viewer/fn.scaled_image_size.html"
source_file: "src/iced_widget/image/viewer.rs"
source_lines: "394-414"
---

# `iced_widget::image::viewer::scaled_image_size`

```rust
pub fn scaled_image_size<Renderer>(
    renderer: &Renderer,
    handle: &<Renderer as Renderer>::Handle,
    state: &State,
    bounds: Size,
    content_fit: ContentFit,
) -> Sizewhere
    Renderer: Renderer,
```

Returns the bounds of the underlying image, given the bounds of the [`Viewer`](struct.Viewer.md). Scaling will be applied and original aspect ratio will be respected.

**Source:** [viewer.rs](../../../src/iced_widget/image/viewer.rs.md) (lines 394-414)
