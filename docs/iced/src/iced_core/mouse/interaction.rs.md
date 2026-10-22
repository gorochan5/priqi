---
title: "src/iced_core/mouse/interaction.rs"
kind: source
path: "src/iced_core/mouse/interaction.rs"
crate: "iced_core"
file: "src/iced_core/mouse/interaction.rs.md"
docs_url: "https://docs.iced.rs/src/iced_core/mouse/interaction.rs.html"
source_file: "src/iced_core/mouse/interaction.rs"
---

# `src/iced_core/mouse/interaction.rs`

```rust
/// The interaction of a mouse cursor.
#[derive(Debug, Eq, PartialEq, Clone, Copy, PartialOrd, Ord, Default)]
#[allow(missing_docs)]
pub enum Interaction {
    #[default]
    None,
    Hidden,
    Idle,
    ContextMenu,
    Help,
    Pointer,
    Progress,
    Wait,
    Cell,
    Crosshair,
    Text,
    Alias,
    Copy,
    Move,
    NoDrop,
    NotAllowed,
    Grab,
    Grabbing,
    ResizingHorizontally,
    ResizingVertically,
    ResizingDiagonallyUp,
    ResizingDiagonallyDown,
    ResizingColumn,
    ResizingRow,
    AllScroll,
    ZoomIn,
    ZoomOut,
}
```
