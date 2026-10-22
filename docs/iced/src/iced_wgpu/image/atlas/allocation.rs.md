---
title: "src/iced_wgpu/image/atlas/allocation.rs"
kind: source
path: "src/iced_wgpu/image/atlas/allocation.rs"
crate: "iced_wgpu"
file: "src/iced_wgpu/image/atlas/allocation.rs.md"
docs_url: "https://docs.iced.rs/src/iced_wgpu/image/atlas/allocation.rs.html"
source_file: "src/iced_wgpu/image/atlas/allocation.rs"
---

# `src/iced_wgpu/image/atlas/allocation.rs`

```rust
use crate::core::Size;
use crate::image::atlas::allocator;

#[derive(Debug)]
pub enum Allocation {
    Partial {
        layer: usize,
        region: allocator::Region,
        atlas_size: u32,
    },
    Full {
        layer: usize,
        size: u32,
    },
}

impl Allocation {
    pub fn position(&self) -> (u32, u32) {
        match self {
            Allocation::Partial { region, .. } => region.position(),
            Allocation::Full { .. } => (0, 0),
        }
    }

    pub fn size(&self) -> Size<u32> {
        match self {
            Allocation::Partial { region, .. } => region.size(),
            Allocation::Full { size, .. } => Size::new(*size, *size),
        }
    }

    pub fn padding(&self) -> Size<u32> {
        match self {
            Allocation::Partial { region, .. } => region.padding(),
            Allocation::Full { .. } => Size::new(0, 0),
        }
    }

    pub fn layer(&self) -> usize {
        match self {
            Allocation::Partial { layer, .. } => *layer,
            Allocation::Full { layer, .. } => *layer,
        }
    }

    pub fn atlas_size(&self) -> u32 {
        match self {
            Allocation::Partial { atlas_size, .. } => *atlas_size,
            Allocation::Full { size, .. } => *size,
        }
    }
}
```
