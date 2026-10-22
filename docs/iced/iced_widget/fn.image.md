---
title: "image"
kind: function
path: "iced_widget::image"
crate: "iced_widget"
module: "iced_widget"
file: "iced_widget/fn.image.md"
docs_url: "https://docs.iced.rs/iced_widget/fn.image.html"
source_file: "src/iced_widget/helpers.rs"
source_lines: "1467-1469"
---

# `iced_widget::image`

```rust
pub fn image<Handle>(handle: impl Into<Handle>) -> Image<Handle>
```

Creates a new [`Image`](image/struct.Image.md).

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

**Source:** [helpers.rs](../src/iced_widget/helpers.rs.md) (lines 1467-1469)
