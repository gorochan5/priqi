---
title: "allocate"
kind: function
path: "iced_runtime::image::allocate"
crate: "iced_runtime"
module: "iced_runtime::image"
file: "iced_runtime/image/fn.allocate.md"
docs_url: "https://docs.iced.rs/iced_runtime/image/fn.allocate.html"
source_file: "src/iced_runtime/image.rs"
source_lines: "20-22"
---

# `iced_runtime::image::allocate`

```rust
pub fn allocate(handle: impl Into<Handle>) -> Task<Result<Allocation, Error>>
```

Allocates an image [`Handle`].

When you obtain an [`Allocation`](struct.Allocation.md) explicitly, you get the guarantee that using a [`Handle`] will draw the corresponding image immediately in the next frame.

**Source:** [image.rs](../../src/iced_runtime/image.rs.md) (lines 20-22)
