---
title: "allocate"
kind: function
path: "iced_core::image::allocate"
crate: "iced_core"
module: "iced_core::image"
file: "iced_core/image/fn.allocate.md"
docs_url: "https://docs.iced.rs/iced_core/image/fn.allocate.html"
source_file: "src/iced_core/image.rs"
source_lines: "275-280"
---

# `iced_core::image::allocate`

```rust
pub unsafe fn allocate(handle: &Handle, size: Size<u32>) -> Allocation
```

Creates a new [`Allocation`](struct.Allocation.md) for the given handle.

This should only be used internally by renderer implementations.

## Safety

Must only be created once the [`Handle`](enum.Handle.md) is allocated in memory.

**Source:** [image.rs](../../src/iced_core/image.rs.md) (lines 275-280)
