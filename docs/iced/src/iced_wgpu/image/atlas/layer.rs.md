---
title: "src/iced_wgpu/image/atlas/layer.rs"
kind: source
path: "src/iced_wgpu/image/atlas/layer.rs"
crate: "iced_wgpu"
file: "src/iced_wgpu/image/atlas/layer.rs.md"
docs_url: "https://docs.iced.rs/src/iced_wgpu/image/atlas/layer.rs.html"
source_file: "src/iced_wgpu/image/atlas/layer.rs"
---

# `src/iced_wgpu/image/atlas/layer.rs`

```rust
use crate::image::atlas::Allocator;

#[derive(Debug)]
pub enum Layer {
    Empty,
    Busy(Allocator),
    Full,
}

impl Layer {
    pub fn is_empty(&self) -> bool {
        matches!(self, Layer::Empty)
    }

    pub fn allocations(&self) -> usize {
        match self {
            Layer::Empty => 0,
            Layer::Busy(allocator) => allocator.allocations(),
            Layer::Full => 1,
        }
    }
}
```
