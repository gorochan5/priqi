---
title: "src/iced_core/shadow.rs"
kind: source
path: "src/iced_core/shadow.rs"
crate: "iced_core"
file: "src/iced_core/shadow.rs.md"
docs_url: "https://docs.iced.rs/src/iced_core/shadow.rs.html"
source_file: "src/iced_core/shadow.rs"
---

# `src/iced_core/shadow.rs`

```rust
use crate::{Color, Vector};

/// A shadow.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Shadow {
    /// The color of the shadow.
    pub color: Color,

    /// The offset of the shadow.
    pub offset: Vector,

    /// The blur radius of the shadow.
    pub blur_radius: f32,
}
```
