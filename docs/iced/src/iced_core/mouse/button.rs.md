---
title: "src/iced_core/mouse/button.rs"
kind: source
path: "src/iced_core/mouse/button.rs"
crate: "iced_core"
file: "src/iced_core/mouse/button.rs.md"
docs_url: "https://docs.iced.rs/src/iced_core/mouse/button.rs.html"
source_file: "src/iced_core/mouse/button.rs"
---

# `src/iced_core/mouse/button.rs`

```rust
/// The button of a mouse.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Button {
    /// The left mouse button.
    Left,

    /// The right mouse button.
    Right,

    /// The middle (wheel) button.
    Middle,

    /// The back mouse button.
    Back,

    /// The forward mouse button.
    Forward,

    /// Some other button.
    Other(u16),
}
```
