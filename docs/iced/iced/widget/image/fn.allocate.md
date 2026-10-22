---
title: "allocate"
kind: function
path: "iced::widget::image::allocate"
crate: "iced"
module: "iced::widget::image"
file: "iced/widget/image/fn.allocate.md"
docs_url: "https://docs.iced.rs/iced/widget/image/fn.allocate.html"
---

# `iced::widget::image::allocate`

```rust
pub fn allocate(handle: impl Into<Handle>) -> Task<Result<Allocation, Error>>
```

Allocates an image [`Handle`](../../advanced/image/enum.Handle.md).

When you obtain an [`Allocation`](../../advanced/image/struct.Allocation.md) explicitly, you get the guarantee that using a [`Handle`](../../advanced/image/enum.Handle.md) will draw the corresponding image immediately in the next frame.
