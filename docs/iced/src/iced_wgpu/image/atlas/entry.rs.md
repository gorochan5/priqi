---
title: "src/iced_wgpu/image/atlas/entry.rs"
kind: source
path: "src/iced_wgpu/image/atlas/entry.rs"
crate: "iced_wgpu"
file: "src/iced_wgpu/image/atlas/entry.rs.md"
docs_url: "https://docs.iced.rs/src/iced_wgpu/image/atlas/entry.rs.html"
source_file: "src/iced_wgpu/image/atlas/entry.rs"
---

# `src/iced_wgpu/image/atlas/entry.rs`

```rust
use crate::core::Size;
use crate::image::atlas;

#[derive(Debug)]
pub enum Entry {
    Contiguous(atlas::Allocation),
    Fragmented {
        size: Size<u32>,
        fragments: Vec<Fragment>,
    },
}

impl Entry {
    #[cfg(feature = "image")]
    pub fn size(&self) -> Size<u32> {
        match self {
            Entry::Contiguous(allocation) => allocation.size(),
            Entry::Fragmented { size, .. } => *size,
        }
    }
}

#[derive(Debug)]
pub struct Fragment {
    pub position: (u32, u32),
    pub allocation: atlas::Allocation,
}
```
